use crate::core::types::{OsintModule, Target, TargetType};
use anyhow::Result;
use async_trait::async_trait;
use trust_dns_resolver::config::*;
use trust_dns_resolver::TokioAsyncResolver;

pub struct DnsIntelModule;

#[async_trait]
impl OsintModule for DnsIntelModule {
    fn name(&self) -> String {
        "DNS Intelligence".to_string()
    }

    #[allow(dead_code)]
    fn description(&self) -> String {
        "Extrai registros MX, TXT e NS".to_string()
    }

    async fn run(&self, target: &Target, _client: &reqwest::Client) -> Result<Vec<Target>> {
        if target.kind != TargetType::Domain {
            return Ok(vec![]);
        }

        let domain = &target.value;
        let clean_domain = if domain.contains("://") {
            domain
                .split("://")
                .nth(1)
                .unwrap()
                .split('/')
                .next()
                .unwrap()
        } else {
            domain.split('/').next().unwrap()
        };

        println!(
            "┃  │   📡  Consultando registros DNS para {}...",
            clean_domain
        );

        let resolver = TokioAsyncResolver::tokio(ResolverConfig::google(), ResolverOpts::default());

        let mut findings = Vec::new();

        if let Ok(mx_records) = resolver.mx_lookup(clean_domain).await {
            for record in mx_records {
                let mx_host = record.exchange().to_string();
                findings.push(Target::new(
                    &format!("MX: {}", mx_host.trim_end_matches('.')),
                    TargetType::Technology,
                ));
            }
        }

        if let Ok(txt_records) = resolver.txt_lookup(clean_domain).await {
            for record in txt_records {
                for txt_data in record.txt_data() {
                    let txt_str = String::from_utf8_lossy(txt_data);
                    if txt_str.contains("spf")
                        || txt_str.contains("google-site-verification")
                        || txt_str.contains("docusign")
                    {
                        findings.push(Target::new(
                            &format!("TXT: {}", txt_str),
                            TargetType::Technology,
                        ));
                    }
                }
            }
        }

        Ok(findings)
    }
}
