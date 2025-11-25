use crate::core::types::{Target, TargetType, OsintModule};
use async_trait::async_trait;
use anyhow::Result;
use reqwest;

pub struct GithubModule;

#[async_trait]
impl OsintModule for GithubModule {
    fn name(&self) -> String {
        "GitHub User Check".to_string()
    }

    fn description(&self) -> String {
        "Verifica se o username possui conta no GitHub".to_string()
    }

    async fn run(&self, target: &Target) -> Result<Vec<Target>> {
        if target.kind != TargetType::Username {
            return Ok(vec![]);
        }

        let username = &target.value;
        let url = format!("https://github.com/{}", username);

        let client = reqwest::Client::new();
        let resp = client.get(&url)
            .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)")
            .send()
            .await?;

        if resp.status().is_success() {
            return Ok(vec![
                Target::new(&url, TargetType::Domain) 
            ]);
        }

        Ok(vec![])
    }
}