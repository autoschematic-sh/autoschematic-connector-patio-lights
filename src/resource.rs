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
    fn to_string(&self) -> Result<String, anyhow::Error> {
        Ok(RON.to_string_pretty(self, PrettyConfig::new())?)
    }

    fn from_str(
        addr: &impl autoschematic_core::connector::ResourceAddress,
        s: &str,
    ) -> Result<Self, anyhow::Error>
    where
        Self: Sized,
    {
        Ok(RON.from_str(s)?)
    }
}
