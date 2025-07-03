use std::{path::{Path, PathBuf}, sync::Arc};

use async_trait::async_trait;
use autoschematic_core::{
    connector::{
        Connector, ConnectorOp, ConnectorOutbox, FilterOutput, GetResourceOutput, OpExecOutput, OpPlanOutput, Resource,
        ResourceAddress,
    },
    connector_op,
    diag::DiagnosticOutput,
    get_resource_output,
    util::{ron_check_eq, ron_check_syntax},
};

use anyhow::Context;

use crate::{addr::LightAddress, op::LightConnectorOp, resource::LightState};

pub struct LightConnector {
    url:    String,
    prefix: PathBuf,
}

#[async_trait]
impl Connector for LightConnector {
    async fn new(name: &str, prefix: &Path, outbox: ConnectorOutbox) -> Result<Arc<dyn Connector>, anyhow::Error>
    where
        Self: Sized,
    {
        let url = std::env::var("AUTOSCHEMATIC_LIGHT_URL").unwrap_or(String::from("https://backend.autoschematic.sh"));

        Ok(Arc::new(LightConnector {
            url,
            prefix: prefix.into(),
        }))
    }

    async fn init(&self) -> anyhow::Result<()> {
        Ok(())
    }

    async fn filter(&self, addr: &Path) -> Result<FilterOutput, anyhow::Error> {
        if let Ok(_addr) = LightAddress::from_path(addr) {
            Ok(FilterOutput::Resource)
        } else {
            Ok(FilterOutput::None)
        }
    }

    async fn list(&self, subpath: &Path) -> Result<Vec<PathBuf>, anyhow::Error> {
        Ok(vec![LightAddress {}.to_path_buf()])
    }

    async fn get(&self, addr: &Path) -> Result<Option<GetResourceOutput>, anyhow::Error> {
        let _addr = LightAddress::from_path(addr)?;
        let res = reqwest::get(format!("{}/api/lights/status", self.url)).await?;
        // eprintln!("{:?}", res.text().await);
        let light_state: LightState = res.json().await?;
        return get_resource_output!(light_state);
    }

    async fn plan(
        &self,
        addr: &Path,
        current: Option<Vec<u8>>,
        desired: Option<Vec<u8>>,
    ) -> Result<Vec<OpPlanOutput>, anyhow::Error> {
        let addr = LightAddress::from_path(addr)?;

        match (current, desired) {
            (Some(current), Some(desired)) => {
                let _current = LightState::from_bytes(&addr, &current)?;
                let desired = LightState::from_bytes(&addr, &desired)?;

                let colored_light_string: Vec<String> =
                    desired.lights.iter().map(|s| s.to_colored_string().to_string()).collect();

                Ok(vec![connector_op!(
                    LightConnectorOp::SetLights(desired),
                    format!("Set the lights to {}", colored_light_string.join(", "))
                )])
            }
            other => Ok(Vec::new()),
        }
    }

    async fn op_exec(&self, addr: &Path, op: &str) -> Result<OpExecOutput, anyhow::Error> {
        let _addr = LightAddress::from_path(addr)?;
        let op = LightConnectorOp::from_str(op)?;

        match op {
            LightConnectorOp::SetLights(light_state) => {
                reqwest::Client::new()
                    .post(format!("{}/api/lights", self.url))
                    .json(&light_state)
                    .send()
                    .await?;

                return Ok(OpExecOutput {
                    outputs: None,
                    friendly_message: Some("Set the lights to the desired setting.".into()),
                });
            }
        }
    }

    async fn eq(&self, addr: &Path, a: &[u8], b: &[u8]) -> Result<bool, anyhow::Error> {
        let _addr = LightAddress::from_path(addr)?;

        return ron_check_eq::<LightState>(a, b);
    }

    async fn diag(&self, addr: &Path, a: &[u8]) -> Result<DiagnosticOutput, anyhow::Error> {
        let _addr = LightAddress::from_path(addr)?;

        return ron_check_syntax::<LightState>(a);
    }
}
