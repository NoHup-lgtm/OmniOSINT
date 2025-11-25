use crate::core::types::{Target, TargetType, OsintModule};
use async_trait::async_trait;
use anyhow::Result;
use reqwest;
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Deserialize)]
struct CrtShEntry {
    name_value: String,
}

pub struct SubdomainModule;

#[async_trait]
impl OsintModule for SubdomainModule {
    fn name(&self) -> String { "CRT.sh Subdomain Finder".to_string() }
    
    #[allow(dead_code)]
    fn description(&self) -> String { "Busca subdomínios via Certificate Transparency".to_string() }

    async fn run(&self, target: &Target) -> Result<Vec<Target>> {
        if target.kind != TargetType::Domain {
            return Ok(vec![]);
        }

        let raw_domain = &target.value;
        let clean_domain = if raw_domain.contains("://") {
            let without_proto: Vec<&str> = raw_domain.split("://").collect();
            without_proto[1].split('/').next().unwrap_or(without_proto[1])
        } else {
            raw_domain.split('/').next().unwrap_or(raw_domain)
        };
        // ------------------------------------------

        println!("┃  │   ☁️  Consultando certificados para {}...", clean_domain);

        let url = format!("https://crt.sh/?q=%.{}&output=json", clean_domain);
        
        let client = reqwest::Client::new();
        let resp = client.get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .timeout(std::time::Duration::from_secs(20)) 
            .send()
            .await;

        match resp {
            Ok(response) => {
                if !response.status().is_success() {
                    return Ok(vec![]);
                }
                
                let entries: Vec<CrtShEntry> = response.json().await.unwrap_or_default();
                let mut unique_subs = HashSet::new();
                let mut results = Vec::new();

                for entry in entries {
                    for sub in entry.name_value.split('\n') {
                        if !sub.contains('*') && sub != clean_domain && sub.contains(clean_domain) {
                            if unique_subs.insert(sub.to_string()) {
                                results.push(Target::new(sub, TargetType::Domain));
                            }
                        }
                    }
                }

                let limited_results = results.into_iter().take(5).collect();
                Ok(limited_results)
            },
            Err(_) => Ok(vec![])
        }
    }
}