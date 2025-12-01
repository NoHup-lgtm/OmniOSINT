use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use anyhow::Result;
use std::fmt;
use colored::*;
use reqwest::Client; // <--- Importante!

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TargetType {
    IP,
    Domain,
    Username,
    RealName,    // <--- Adicionei
    Email,
    Phone,
    DiscordID,   // <--- Adicionei
    OpenPort,
    Technology,
    SensitiveFile,
    Vulnerability,
    Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub value: String,
    pub kind: TargetType,
}

impl Target {
    pub fn new(value: &str, kind: TargetType) -> Self {
        Self {
            value: value.to_string(),
            kind,
        }
    }
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind_str = format!("[{:?}]", self.kind).bold().blue();
        write!(f, "{} {}", kind_str, self.value)
    }
}

#[async_trait]
pub trait OsintModule: Send + Sync {
    fn name(&self) -> String;
    
    #[allow(dead_code)] 
    fn description(&self) -> String;
    
    // MUDANÇA CRÍTICA: Adicionamos 'client' aqui!
    async fn run(&self, target: &Target, client: &Client) -> Result<Vec<Target>>;
}