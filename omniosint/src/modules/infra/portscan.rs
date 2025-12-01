use crate::core::types::{OsintModule, Target, TargetType};
use anyhow::Result;
use async_trait::async_trait;
use std::time::Duration;
use tokio::net::TcpStream as AsyncTcpStream;
use tokio::time::timeout;
use reqwest;

pub struct PortScanModule;

#[async_trait]
impl OsintModule for PortScanModule {
    fn name(&self) -> String { "Fast Port Scanner".to_string() }
    fn description(&self) -> String { "Verifica portas abertas comuns".to_string() }

    // CORREÇÃO: Adicionado _client (com underscore pois não usamos)
    async fn run(&self, target: &Target, _client: &reqwest::Client) -> Result<Vec<Target>> {
        if target.kind != TargetType::Domain && target.kind != TargetType::IP {
            return Ok(vec![]);
        }

        let host_str = &target.value;
        let clean_host = if host_str.contains("://") {
            host_str.split("://").nth(1).unwrap().split('/').next().unwrap()
        } else {
            host_str.split('/').next().unwrap()
        };

        let ports = vec![22, 53, 80, 443, 3306, 8080];
        let mut open_ports = Vec::new();

        println!("┃  │   🔍 Escaneando portas em {}...", clean_host);

        for port in ports {
            let address = format!("{}:{}", clean_host, port);
            let connect_future = AsyncTcpStream::connect(&address);
            if let Ok(Ok(_)) = timeout(Duration::from_millis(1000), connect_future).await {
                open_ports.push(Target::new(&format!("{}:{}", clean_host, port), TargetType::OpenPort));
            }
        }
        Ok(open_ports)
    }
}