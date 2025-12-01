use crate::core::types::{Target, TargetType, OsintModule};
use async_trait::async_trait;
use anyhow::Result;
use reqwest;

pub struct PhoneDiscordModule;

#[async_trait]
impl OsintModule for PhoneDiscordModule {
    fn name(&self) -> String { "ID & Phone Intel".to_string() }
    fn description(&self) -> String { "Analisa metadados".to_string() }

    async fn run(&self, target: &Target, _client: &reqwest::Client) -> Result<Vec<Target>> {
        let mut findings = Vec::new();

        if target.kind == TargetType::Phone {
            let num = &target.value;
            if num.starts_with("+55") { findings.push(Target::new("Loc: Brasil", TargetType::Location)); }
            else if num.starts_with("+1") { findings.push(Target::new("Loc: USA", TargetType::Location)); }
        }

        if target.kind == TargetType::DiscordID {
            if let Ok(id) = target.value.parse::<u64>() {
                let timestamp = (id >> 22) + 1420070400000;
                findings.push(Target::new(&format!("Discord Created (Epoch): {}", timestamp), TargetType::Technology));
            }
        }
        Ok(findings)
    }
}