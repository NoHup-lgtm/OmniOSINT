use crate::core::types::{Target, TargetType, OsintModule};
use async_trait::async_trait;
use anyhow::Result;
use reqwest;

pub struct GithubModule;

#[async_trait]
impl OsintModule for GithubModule {
    fn name(&self) -> String { "GitHub User Check".to_string() }
    fn description(&self) -> String { "Verifica GitHub".to_string() }

    async fn run(&self, target: &Target, client: &reqwest::Client) -> Result<Vec<Target>> {
        if target.kind != TargetType::Username { return Ok(vec![]); }
        let url = format!("https://github.com/{}", target.value);
        
        if let Ok(resp) = client.get(&url).send().await {
            if resp.status().is_success() {
                return Ok(vec![Target::new(&url, TargetType::Domain)]);
            }
        }
        Ok(vec![])
    }
}