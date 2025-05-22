use std::{
    ffi::{OsStr, OsString},
    os::unix::ffi::OsStrExt,
};

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
    fn to_os_string(&self) -> Result<OsString, anyhow::Error> {
        Ok(RON.to_string_pretty(self, PrettyConfig::new())?.into())
    }

    fn from_os_str(
        addr: &impl autoschematic_core::connector::ResourceAddress,
        s: &OsStr,
    ) -> Result<Self, anyhow::Error>
    where
        Self: Sized,
    {
        let s = str::from_utf8(s.as_bytes())?;
        let state: LightState = RON.from_str(s)?;
        eprintln!("Lighstate::from_os_str()");
        Ok(state)
    }
}
