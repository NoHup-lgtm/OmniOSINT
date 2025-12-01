use crate::core::types::{OsintModule, Target, TargetType};
use anyhow::Result;
use async_trait::async_trait;
use reqwest;

pub struct TechStackModule;

#[async_trait]
impl OsintModule for TechStackModule {
    fn name(&self) -> String {
        "Tech Stack Fingerprinter".to_string()
    }
    fn description(&self) -> String {
        "Identifica tecnologias via Headers".to_string()
    }

    async fn run(&self, target: &Target, client: &reqwest::Client) -> Result<Vec<Target>> {
        if target.kind != TargetType::Domain && target.kind != TargetType::IP {
            return Ok(vec![]);
        }

        let url = if target.value.starts_with("http") {
            target.value.clone()
        } else {
            format!("https://{}", target.value)
        };

        if let Ok(resp) = client.head(&url).send().await {
            let headers = resp.headers();
            let mut findings = Vec::new();
            let keys = vec!["server", "x-powered-by", "via"];

            for k in keys {
                if let Some(v) = headers.get(k) {
                    if let Ok(s) = v.to_str() {
                        findings.push(Target::new(
                            &format!("{}: {}", k, s),
                            TargetType::Technology,
                        ));
                    }
                }
            }
            return Ok(findings);
        }
        Ok(vec![])
    }
}
