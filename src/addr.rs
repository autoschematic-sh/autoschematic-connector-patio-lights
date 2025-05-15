use std::path::PathBuf;

use autoschematic_core::connector::ResourceAddress;

#[derive(Debug, Clone)]
pub struct LightAddress {}

impl ResourceAddress for LightAddress {
    fn to_path_buf(&self) -> std::path::PathBuf {
        PathBuf::from("lighting/patio/lights.ron")
    }

    fn from_path(path: &std::path::Path) -> Result<Option<Self>, anyhow::Error>
    where
        Self: Sized,
    {
        if path == PathBuf::from("lighting/patio/lights.ron") {
            Ok(Some(LightAddress {}))
        } else {
            Ok(None)
        }
    }
}
