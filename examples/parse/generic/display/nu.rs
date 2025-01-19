use std::{
    future::Future,
    io::{Read, Write},
    process::Stdio,
    sync::Arc,
    time::Duration,
};

use anyhow::Context as _;
use futures::{future::BoxFuture, FutureExt};
use tap::{Conv, Pipe, TapFallible};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt as _, AsyncWriteExt, BufReader},
    process::Command,
    runtime::Runtime,
    sync::Mutex,
};
use tokio_stream::{wrappers::LinesStream, StreamExt};
use tracing::{error, info};

use crate::generic::format::{ParseResult, Parsed};

use super::ActiveDisplay;

pub const KNOWN_GOOD_NU_VERSION: &str = "0.101.0";

pub const NU_COMMAND: &str =
    "open -r /dev/stdin | lines | each { from json | table | ansi strip | to json } | to text";

pub struct Display {
    pub prefix: String,
}

impl Display {
    pub fn new_if_nu_installed() -> Option<Self> {
        let version = nu_version()?;

        if check_nu_support() == false {
            return None;
        }

        let prefix = if version == KNOWN_GOOD_NU_VERSION {
            "".to_string()
        } else {
            format!(
                "Using nu version {version} (known good version = {KNOWN_GOOD_NU_VERSION}) \n\n"
            )
        };

        Some(Self { prefix })
    }
}

pub fn nu_version() -> Option<String> {
    let version = std::process::Command::new("nu")
        .args(&["-c", "version | get version"])
        .output()
        .tap_err(|err| info!("failed to get nu version: {}", err))
        .ok()?
        .stdout
        .pipe(|bytes| String::from_utf8_lossy(&bytes).into_owned());

    // Versions start with a digit.
    if version
        .chars()
        .next()
        .is_some_and(|c| char::is_ascii_digit(&c))
    {
        Some(version)
    } else {
        None
    }
}

/// Check whether our version of nushell supports the command we need.
///
/// This may leak a process.
pub fn check_nu_support() -> bool {
    let result: anyhow::Result<String> = Runtime::new().unwrap().block_on(async move {
        Ok(tableize_json_with_nu().await?("'hello\"world'".to_string()).await)
    });
    dbg!(result).ok() == Some(r#""hello\"world""#.to_string())
}

/// Use nushell to convert JSONs into tables.
///
/// May leak a process.
pub async fn tableize_json_with_nu(
) -> anyhow::Result<impl FnMut(String) -> BoxFuture<'static, String>> {
    let cmd = Command::new("nu")
        .args(&["-c", NU_COMMAND])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .context("could not spawn nushell to see if we have a compatible version: {err}")?;

    let outputs = cmd
        .stdout
        // .unwrap()
        // .pipe(BufReader::new)
        // .lines()
        // .pipe(LinesStream::new)
        // .map(|out_json_result| {
        //     let out_json = match dbg!(out_json_result) {
        //         Ok(out_json) => out_json,
        //         Err(err) => return format!("could not read nushell table output: {err}"),
        //     };
        //     serde_json::from_str::<String>(&out_json)
        //         .unwrap_or_else(|err| format!("could not parse nushell table output: {err}"))
        // })
        .pipe(Mutex::new)
        .pipe(Arc::new);

    let stdin = Arc::new(Mutex::new(cmd.stdin.unwrap()));

    let f = move |input: String| {
        let stdin = stdin.clone();
        let outputs = outputs.clone();
        async move {
            let mut stdin_guard = stdin.lock().await;
            stdin_guard
                .write_all("3\n3\n\n\n".as_bytes())
                .await
                .unwrap();
            stdin_guard.flush().await.unwrap();
            stdin_guard
                .write_all((input + "\n").as_bytes())
                .await
                .unwrap();
            stdin_guard.flush().await.unwrap();

            info!("flushed input");

            let mut outputs = outputs.lock().await.take().unwrap();
            let mut out = Vec::new();
            for _ in 0..3 {
                dbg!(outputs.read(&mut out).await.unwrap());
                tokio::time::sleep(Duration::from_secs(1)).await;
            }

            dbg!(String::from_utf8_lossy(&out));

            // dbg!(
            //     outputs
            //         .try_lock()
            //         .expect("tried to tableize multiple values concurrently")
            //         .next()
            //         .await
            // )
            // .unwrap_or_else(|| "nushell did not format our value".to_string())

            todo!()
        }
        .boxed()
    };
    Ok(f)
}

impl ActiveDisplay for Display {
    fn display(&self, value: &ParseResult) {
        println!("{}{:?}", self.prefix, value);
    }
}
