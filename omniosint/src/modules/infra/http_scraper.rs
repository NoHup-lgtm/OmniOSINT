use crate::core::types::{Target, TargetType, OsintModule};
use async_trait::async_trait;
use anyhow::Result;
use reqwest;
use regex::Regex;

pub struct HttpScraperModule;

#[async_trait]
impl OsintModule for HttpScraperModule {
    fn name(&self) -> String { "HTML Email/Info Extractor".to_string() }
    
    #[allow(dead_code)]
    fn description(&self) -> String { "Busca e-mails em páginas web encontradas".to_string() }

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
            .timeout(std::time::Duration::from_secs(10))
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko)") // Finge ser Chrome no Mac
            .build()?;

        let resp = client.get(&url).send().await;
        
        match resp {
            Ok(response) => {
                let text = response.text().await.unwrap_or_default();

                let re = Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap();
                let mut findings = Vec::new();

                for caps in re.captures_iter(&text) {
                    if let Some(match_) = caps.get(0) {
                        let email_candidate = match_.as_str().to_lowercase();

                        if email_candidate.ends_with(".png") 
                           || email_candidate.ends_with(".jpg") 
                           || email_candidate.ends_with(".jpeg") 
                           || email_candidate.ends_with(".gif") 
                           || email_candidate.ends_with(".svg") 
                           || email_candidate.ends_with(".webp") {
                            continue;
                        }

                        if email_candidate.contains("example.com") 
                           || email_candidate.contains("domain.com") 
                           || email_candidate.contains("yourdomain.com") 
                           || email_candidate.contains("you@") {
                            continue;
                        }
                        // ----------------------------

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