use crate::core::types::{OsintModule, Target, TargetType};
use anyhow::Result;
use async_trait::async_trait;
use std::time::Duration;
use tokio::net::TcpStream as AsyncTcpStream;
use tokio::time::timeout;

pub struct PortScanModule;

#[async_trait]
impl OsintModule for PortScanModule {
    fn name(&self) -> String {
        "Fast Port Scanner".to_string()
    }

    #[allow(dead_code)]
    fn description(&self) -> String {
        "Verifica portas abertas comuns".to_string()
    }

    async fn run(&self, target: &Target) -> Result<Vec<Target>> {
        if target.kind != TargetType::Domain && target.kind != TargetType::IP {
            return Ok(vec![]);
        }

        let host_str = &target.value;
        let clean_host = if host_str.contains("://") {
            let without_protocol: Vec<&str> = host_str.split("://").collect();
            without_protocol[1]
                .split('/')
                .next()
                .unwrap_or(without_protocol[1])
        } else {
            host_str.split('/').next().unwrap_or(host_str)
        };

        let ports = vec![22, 53, 80, 443, 3306, 8080];
        let mut open_ports = Vec::new();

        println!(
            "┃  │   🔍 Escaneando portas em {} (Host Real)...",
            clean_host
        );

        for port in ports {
            let address = format!("{}:{}", clean_host, port);

            let connect_future = AsyncTcpStream::connect(&address);
            let result = timeout(Duration::from_millis(1000), connect_future).await;

            if let Ok(Ok(_)) = result {
                open_ports.push(Target::new(
                    &format!("{}:{}", clean_host, port), 
                    TargetType::OpenPort,                
                ));
            }
        }

        Ok(open_ports)
    }
}