use autoschematic_core::tarpc_bridge::tarpc_connector_main;
use connector::LightConnector;

pub mod connector;
pub mod addr;
pub mod resource;
pub mod op;


#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    tarpc_connector_main::<LightConnector>().await?;
    Ok(())
}
