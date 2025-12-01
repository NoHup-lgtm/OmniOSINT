use crate::core::types::{Target, TargetType, OsintModule};
use async_trait::async_trait;
use anyhow::Result;
use md5;
use reqwest;

pub struct EmailIntelModule;

#[async_trait]
impl OsintModule for EmailIntelModule {
    fn name(&self) -> String { "Email/Discord Intel".to_string() }
    fn description(&self) -> String { "Busca Gravatar e sugere Discord".to_string() }

    async fn run(&self, target: &Target, client: &reqwest::Client) -> Result<Vec<Target>> {
        if target.kind != TargetType::Email { return Ok(vec![]); }

        let email = target.value.trim().to_lowercase();
        let hash = format!("{:x}", md5::compute(&email));
        let url = format!("https://en.gravatar.com/{}.json", hash);
        
        let mut findings = Vec::new();
        // Usa o client global (proxy)
        if let Ok(resp) = client.get(&url).send().await {
            if resp.status().is_success() {
                let text = resp.text().await.unwrap_or_default();
                if let Some(start) = text.find(r#""preferredUsername":""#) {
                    let rest = &text[start + 21..];
                    if let Some(end) = rest.find(r#"""#) {
                        let user = &rest[..end];
                        findings.push(Target::new(user, TargetType::Username));
                        findings.push(Target::new(&format!("Discord Possível: {}", user), TargetType::Technology));
                    }
                }
            }
        }
        Ok(findings)
    }
}