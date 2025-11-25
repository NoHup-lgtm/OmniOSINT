use crate::core::types::{Target, TargetType, OsintModule};
use async_trait::async_trait;
use anyhow::Result;
use reqwest;

pub struct TechStackModule;

#[async_trait]
impl OsintModule for TechStackModule {
    fn name(&self) -> String { "Tech Stack Fingerprinter".to_string() }
    
    #[allow(dead_code)]
    fn description(&self) -> String { "Identifica tecnologias via HTTP Headers (Server, X-Powered-By)".to_string() }

    async fn run(&self, target: &Target) -> Result<Vec<Target>> {
        if target.kind != TargetType::Domain && target.kind != TargetType::IP {
            return Ok(vec![]);
        }

        let url = if target.value.starts_with("http") {
            target.value.clone()
        } else {
            format!("https://{}", target.value)
        };

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .user_agent("Mozilla/5.0") 
            .danger_accept_invalid_certs(true) 
            .build()?;

        let resp = match client.head(&url).send().await {
            Ok(r) => r,
            Err(_) => return Ok(vec![]), 
        };

        let headers = resp.headers();
        let mut tech_findings = Vec::new();

        let interesting_headers = vec![
            "server",
            "x-powered-by",
            "x-aspnet-version",
            "x-generator",
            "via"
        ];

        println!("┃  │   🛠️  Analisando headers de {}...", target.value);

        for header_name in interesting_headers {
            if let Some(value) = headers.get(header_name) {
                if let Ok(value_str) = value.to_str() {
                    let finding = format!("{}: {}", header_name, value_str);

                    tech_findings.push(Target::new(&finding, TargetType::Technology)); 
                }
            }
        }

        Ok(tech_findings)
    }
}