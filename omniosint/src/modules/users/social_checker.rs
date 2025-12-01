use crate::core::types::{OsintModule, Target, TargetType};
use anyhow::Result;
use async_trait::async_trait;
use reqwest;

pub struct SocialCheckerModule;

#[async_trait]
impl OsintModule for SocialCheckerModule {
    fn name(&self) -> String {
        "Social Media Hunter".to_string()
    }
    fn description(&self) -> String {
        "Verifica redes sociais".to_string()
    }

    async fn run(&self, target: &Target, client: &reqwest::Client) -> Result<Vec<Target>> {
        if target.kind != TargetType::Username {
            return Ok(vec![]);
        }
        let username = &target.value;
        let sites = vec![
            ("GitHub", "https://github.com/{}"),
            ("Twitter", "https://twitter.com/{}"),
            ("Instagram", "https://www.instagram.com/{}/"),
            ("Reddit", "https://www.reddit.com/user/{}"),
            ("TikTok", "https://www.tiktok.com/@{}"),
        ];

        println!("┃  │   🕵️‍♂️  Caçando '{}'...", username);
        let mut findings = Vec::new();
        for (_, tpl) in sites {
            let url = tpl.replace("{}", username);
            if let Ok(resp) = client.get(&url).send().await {
                if resp.status().is_success() {
                    findings.push(Target::new(&url, TargetType::Domain));
                }
            }
        }
        Ok(findings)
    }
}
