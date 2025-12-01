use crate::core::types::{Target, TargetType, OsintModule};
use async_trait::async_trait;
use anyhow::Result;
use reqwest;
use regex::Regex;

pub struct HttpScraperModule;

#[async_trait]
impl OsintModule for HttpScraperModule {
    fn name(&self) -> String { "HTML Email/Info Extractor".to_string() }
    fn description(&self) -> String { "Busca e-mails em páginas web".to_string() }

    async fn run(&self, target: &Target, client: &reqwest::Client) -> Result<Vec<Target>> {
        if target.kind != TargetType::Domain && target.kind != TargetType::IP {
            return Ok(vec![]);
        }

        let url = if target.value.starts_with("http") {
            target.value.clone()
        } else {
            format!("https://{}", target.value)
        };

        let resp = client.get(&url).send().await;
        
        match resp {
            Ok(response) => {
                let text = response.text().await.unwrap_or_default();
                let re = Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap();
                let mut findings = Vec::new();

                for caps in re.captures_iter(&text) {
                    if let Some(match_) = caps.get(0) {
                        let email_candidate = match_.as_str().to_lowercase();
                        if email_candidate.ends_with(".png") || email_candidate.ends_with(".jpg") || email_candidate.contains("example.com") { continue; }
                        findings.push(Target::new(&email_candidate, TargetType::Email));
                    }
                }
                findings.sort_by(|a, b| a.value.cmp(&b.value));
                findings.dedup_by(|a, b| a.value == b.value);
                Ok(findings)
            }
            Err(_) => Ok(vec![]) 
        }
    }
}