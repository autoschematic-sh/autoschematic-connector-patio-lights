use autoschematic_core::{
    connector::Resource,
    util::{PrettyConfig, RON},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
// #[serde(rename_all = "lowercase")]
pub enum LightColour {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Indigo,
    Violet,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LightState {
    lights: [LightColour; 7],
}

impl Resource for LightState {
    fn to_bytes(&self) -> Result<Vec<u8>, anyhow::Error> {
        Ok(RON.to_string_pretty(self, PrettyConfig::new())?.into())
    }

    fn from_bytes(addr: &impl autoschematic_core::connector::ResourceAddress, s: &[u8]) -> Result<Self, anyhow::Error>
    where
        Self: Sized,
    {
        let s = str::from_utf8(s)?;
        let state: LightState = RON.from_str(s)?;
        eprintln!("Lighstate::from_bytes()");
        Ok(state)
    }
}
