use crate::core::types::{Target, TargetType, OsintModule};
use async_trait::async_trait;
use anyhow::Result;
use reqwest;
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Deserialize)]
struct CrtShEntry { name_value: String }

pub struct SubdomainModule;

#[async_trait]
impl OsintModule for SubdomainModule {
    fn name(&self) -> String { "CRT.sh Subdomain Finder".to_string() }
    fn description(&self) -> String { "Busca subdomínios via Certificate Transparency".to_string() }

    async fn run(&self, target: &Target, client: &reqwest::Client) -> Result<Vec<Target>> {
        if target.kind != TargetType::Domain { return Ok(vec![]); }

        let raw = &target.value;
        let clean = if raw.contains("://") { raw.split("://").nth(1).unwrap().split('/').next().unwrap() } else { raw.split('/').next().unwrap() };

        println!("┃  │   ☁️  Consultando certificados para {}...", clean);
        let url = format!("https://crt.sh/?q=%.{}&output=json", clean);
        
        if let Ok(resp) = client.get(&url).send().await {
            if resp.status().is_success() {
                let entries: Vec<CrtShEntry> = resp.json().await.unwrap_or_default();
                let mut unique = HashSet::new();
                let mut results = Vec::new();
                for e in entries {
                    for sub in e.name_value.split('\n') {
                        if !sub.contains('*') && sub != clean && sub.contains(clean) {
                            if unique.insert(sub.to_string()) {
                                results.push(Target::new(sub, TargetType::Domain));
                            }
                        }
                    }
                }
                return Ok(results.into_iter().take(10).collect());
            }
        }
        Ok(vec![])
    }
}