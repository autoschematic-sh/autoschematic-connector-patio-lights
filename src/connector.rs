use std::path::{Path, PathBuf};

use async_trait::async_trait;
use autoschematic_core::{
    connector::{
        Connector, ConnectorOp, ConnectorOutbox, GetResourceOutput, OpExecOutput, OpPlanOutput,
        Resource, ResourceAddress,
    },
    connector_op,
    diag::DiagnosticOutput,
    get_resource_output,
    util::{ron_check_eq, ron_check_syntax},
};

use crate::{addr::LightAddress, op::LightConnectorOp, resource::LightState};

pub struct LightConnector {
    url: String,
    prefix: PathBuf,
}

#[async_trait]
impl Connector for LightConnector {
    async fn new(
        name: &str,
        prefix: &Path,
        outbox: ConnectorOutbox,
    ) -> Result<Box<dyn Connector>, anyhow::Error>
    where
        Self: Sized,
    {
        eprintln!("LIGHTCONNECTOR!");
        for v in std::env::vars() {
            eprintln!("{} = {}!", v.0, v.1);
        }
        let url = std::env::var("AUTOSCHEMATIC_LIGHT_URL").unwrap_or_default();
        eprintln!("LIGHTCONNECTOR!");
        // if name != "patio" {
        //     return Err(AutoschematicError {
        //         kind: AutoschematicErrorType::InvalidConnectorString(name.to_string()),
        //     }
        //     .into());
        // }

        Ok(Box::new(LightConnector {
            url,
            prefix: prefix.into(),
        }))
    }

    async fn filter(&self, addr: &Path) -> Result<bool, anyhow::Error> {
        if let Some(_addr) = LightAddress::from_path(addr)? {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn list(&self, subpath: &Path) -> Result<Vec<PathBuf>, anyhow::Error> {
        Ok(vec![LightAddress {}.to_path_buf()])
    }

    async fn get(&self, addr: &Path) -> Result<Option<GetResourceOutput>, anyhow::Error> {
        if let Some(_addr) = LightAddress::from_path(addr)? {
            let res = reqwest::get(format!("{}/api/lights/status", self.url)).await?;
            // eprintln!("{:?}", res.text().await);
            let light_state: LightState = res.json().await?;
            eprintln!("{:?}", light_state);
            return get_resource_output!(light_state);
            // Ok(None)
        } else {
            Ok(None)
        }
    }

    async fn plan(
        &self,
        addr: &Path,
        current: Option<String>,
        desired: Option<String>,
    ) -> Result<Vec<OpPlanOutput>, anyhow::Error> {
        let Ok(Some(addr)) = LightAddress::from_path(addr) else {
            return Ok(Vec::new());
        };

        match (current, desired) {
            (Some(current), Some(desired)) => {
                let _current = LightState::from_str(&addr, &current)?;
                let desired = LightState::from_str(&addr, &desired)?;

                Ok(vec![connector_op!(
                    LightConnectorOp::SetLights(desired),
                    format!("Set lights")
                )])
            }
            _ => Ok(Vec::new()),
        }
    }

    async fn op_exec(&self, addr: &Path, op: &str) -> Result<OpExecOutput, anyhow::Error> {
        let Ok(Some(addr)) = LightAddress::from_path(addr) else {
            return Ok(OpExecOutput {
                outputs: None,
                friendly_message: None,
            });
        };

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

    async fn eq(&self, addr: &Path, a: &str, b: &str) -> Result<bool, anyhow::Error> {
        let Ok(Some(_addr)) = LightAddress::from_path(addr) else {
            return Ok(false);
        };

        return ron_check_eq::<LightState>(a, b);
    }

    async fn diag(&self, addr: &Path, a: &str) -> Result<DiagnosticOutput, anyhow::Error> {
        let Ok(Some(_addr)) = LightAddress::from_path(addr) else {
            return Ok(DiagnosticOutput {
                diagnostics: Vec::new(),
            });
        };

        return ron_check_syntax::<LightState>(a);
    }
}
