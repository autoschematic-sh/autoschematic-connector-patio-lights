use std::path::PathBuf;

use autoschematic_core::{
    connector::ResourceAddress,
    error_util::{invalid_addr_path},
};

#[derive(Debug, Clone)]
pub struct LightAddress {}

impl ResourceAddress for LightAddress {
    fn to_path_buf(&self) -> std::path::PathBuf {
        PathBuf::from("patio/lights.ron")
    }

    fn from_path(path: &std::path::Path) -> Result<Self, anyhow::Error>
    where
        Self: Sized,
    {
        if path == PathBuf::from("patio/lights.ron") {
            Ok(LightAddress {})
        } else {
            Err(invalid_addr_path(path))
        }
    }
}
