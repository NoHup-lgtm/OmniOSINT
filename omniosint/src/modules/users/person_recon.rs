use crate::core::types::{Target, TargetType, OsintModule};
use async_trait::async_trait;
use anyhow::Result;
use reqwest;

pub struct PersonReconModule;

#[async_trait]
impl OsintModule for PersonReconModule {
    fn name(&self) -> String { "Name Permutation Engine".to_string() }
    fn description(&self) -> String { "Gera usernames a partir de Nome Real".to_string() }

    async fn run(&self, target: &Target, _client: &reqwest::Client) -> Result<Vec<Target>> {
        if target.kind != TargetType::RealName { return Ok(vec![]); }

        let full_name = target.value.to_lowercase();
        let parts: Vec<&str> = full_name.split_whitespace().collect();
        if parts.len() < 2 { return Ok(vec![]); }

        let first = parts[0];
        let last = parts.last().unwrap();
        println!("┃  │   🧠  Gerando permutações para '{}'...", full_name);

        let perms = vec! [
            format!("{}{}", first, last),
            format!("{}.{}", first, last),
            format!("{}_{}", first, last),
            format!("{}{}", first.chars().next().unwrap(), last),
            format!("{}123", first),
            format!("its{}", first),
        ];
        
        let mut findings = Vec::new();
        for p in perms { findings.push(Target::new(&p, TargetType::Username)); }
        Ok(findings)
    }
}