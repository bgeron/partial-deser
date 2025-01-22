use std::default;
use std::fmt::Debug;

use clap::ValueEnum;
use partial_deser::Error;
use serde::Deserialize;

#[derive(Debug, Clone, ValueEnum, Default)]
pub enum FormatAndSettings {
    #[default]
    Json,
    Yaml,
}

pub type ParseResult = Result<Box<dyn Parsed>, Error<Box<dyn std::error::Error + Send + Sync>>>;

impl FormatAndSettings {
    pub fn parse<P>(&self, input: &[u8]) -> ParseResult
    where
        P: for<'de> Deserialize<'de> + Parsed + 'static,
    {
        match self {
            FormatAndSettings::Json => partial_deser::from_json_slice::<P>(input)
                .map(|ok| Box::new(ok) as Box<dyn Parsed>)
                .map_err(Error::erase),

            #[cfg(feature = "serde_yaml")]
            FormatAndSettings::Yaml => partial_deser::from_yaml_slice::<P>(input)
                .map(|ok| Box::new(ok) as Box<dyn Parsed>)
                .map_err(Error::erase),

            #[cfg(not(feature = "serde_yaml"))]
            FormatAndSettings::Yaml => {
                panic!("feature serde_yaml is not enabled (run cargo with --all-features)")
            }
        }
    }
}

pub trait Parsed: Debug + erased_serde::Serialize + Send + Sync {}
erased_serde::serialize_trait_object!(Parsed);

impl<T> Parsed for T where T: Debug + serde::Serialize + Send + Sync {}
