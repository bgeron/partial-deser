use std::os::fd::OwnedFd;
use std::process::Stdio;
use std::sync::Arc;

use anyhow::Context as _;
use futures::FutureExt;
use futures::future::BoxFuture;
use tap::{Conv, Pipe, TapFallible};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::LinesStream;
use tracing::info;

use crate::generic::format::ParseResult;

use super::ActiveDisplay;

pub const KNOWN_GOOD_NU_VERSION: &str = "0.101.0";

pub const NU_COMMAND: &str =
    "open -r /dev/stdin | lines | each { from json | table | ansi strip | to json } | to text ";

/// Format values using nushell.
///
/// This implementation may be a bit horrible, threads-wise.
pub struct Display {
    pub prefix: String,
    runtime: Runtime,
    pub tableize: Tableize,
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

        let runtime = Runtime::new().unwrap();

        let tableize = runtime
            .block_on(async { tableize_json_with_nu().await })
            .ok()?;

        Some(Self {
            prefix,
            runtime,
            tableize,
        })
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
        Some(version.trim().to_string())
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
    result.ok() == Some(r#"'hello"world'"#.to_string())
}

pub type Tableize = Box<dyn FnMut(String) -> BoxFuture<'static, String>>;

/// Use nushell to convert JSONs into tables.
///
/// May leak a process.
pub async fn tableize_json_with_nu() -> anyhow::Result<Tableize> {
    let (out_tx, out_rx) =
        interprocess::unnamed_pipe::pipe().context("could not construct pipe for output")?;

    let cmd = Command::new("nu")
        .args(&["-c", NU_COMMAND])
        .stdin(Stdio::piped())
        .stdout(OwnedFd::from(out_tx))
        .spawn()
        .context("could not spawn nushell to see if we have a compatible version: {err}")?;

    let outputs = out_rx
        .conv::<OwnedFd>()
        .conv::<std::process::ChildStdout>()
        .pipe(tokio::process::ChildStdout::from_std)
        .context("could not wrap nu stdout")?
        .pipe(Some)
        .unwrap()
        .pipe(BufReader::new)
        .lines()
        .pipe(LinesStream::new)
        .map(|out_json_result| {
            let out_json = match out_json_result {
                Ok(out_json) => out_json,
                Err(err) => return format!("could not read nushell table output: {err}"),
            };
            serde_json::from_str::<String>(&out_json)
                .unwrap_or_else(|err| format!("could not parse nushell table output: {err}"))
        })
        .pipe(Mutex::new)
        .pipe(Arc::new);

    let stdin = Arc::new(Mutex::new(cmd.stdin.unwrap()));

    let f = move |input: String| {
        let stdin = stdin.clone();
        let outputs = outputs.clone();
        async move {
            let mut stdin_guard = stdin.lock().await;
            stdin_guard.write_all(input.as_bytes()).await.unwrap();
            stdin_guard.write_all(b"\n").await.unwrap();
            stdin_guard.flush().await.unwrap();

            outputs
                .try_lock()
                .expect("tried to tableize multiple values concurrently")
                .next()
                .await
                .unwrap_or_else(|| "nushell did not format our value".to_string())
        }
        .boxed()
    };
    Ok(Box::new(f))
}

impl ActiveDisplay for Display {
    fn display(&mut self, value: &ParseResult) -> String {
        self.runtime.block_on(async {
            let tableized = match value {
                Ok(value) => (self.tableize)(serde_json::to_string(value).unwrap()).await,
                Err(err) => format!("could not parse input: {err}"),
            };

            format!("{}{}", self.prefix, tableized)
        })
    }
}
