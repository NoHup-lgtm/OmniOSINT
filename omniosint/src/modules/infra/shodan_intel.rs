use crate::core::types::{Target, TargetType, OsintModule};
use async_trait::async_trait;
use anyhow::Result;
use reqwest;
use serde_json::Value;
use std::env;

pub struct ShodanIntelModule;

#[async_trait]
impl OsintModule for ShodanIntelModule {
    fn name(&self) -> String { "Shodan API Intelligence".to_string() }
    fn description(&self) -> String { "Busca vulnerabilidades (CVEs)".to_string() }

    async fn run(&self, target: &Target, client: &reqwest::Client) -> Result<Vec<Target>> {
        if target.kind != TargetType::IP { return Ok(vec![]); }
        let api_key = match env::var("SHODAN_API_KEY") { Ok(k) => k, Err(_) => return Ok(vec![]) };
        let ip_str = target.value.split(':').next().unwrap_or(&target.value).trim();
        if !ip_str.chars().any(|c| c.is_numeric()) { return Ok(vec![]); }

        println!("┃  │   👁️  Consultando Shodan para {}...", ip_str);
        let url = format!("https://api.shodan.io/shodan/host/{}?key={}", ip_str, api_key);
        
        // USA O CLIENTE GLOBAL
        if let Ok(resp) = client.get(&url).send().await {
            if resp.status().is_success() {
                let json: Value = resp.json().await.unwrap_or(Value::Null);
                let mut findings = Vec::new();
                if let Some(os) = json.get("os").and_then(|v| v.as_str()) {
                    findings.push(Target::new(&format!("OS: {}", os), TargetType::Technology));
                }
                if let Some(isp) = json.get("isp").and_then(|v| v.as_str()) {
                    findings.push(Target::new(&format!("ISP: {}", isp), TargetType::Technology));
                }
                if let Some(vulns) = json.get("vulns").and_then(|v| v.as_array()) {
                    for vuln in vulns {
                        if let Some(cve) = vuln.as_str() {
                            findings.push(Target::new(cve, TargetType::Vulnerability));
                        }
                    }
                }
                return Ok(findings);
            }
        }
        Ok(vec![])
    }
}