use crate::core::types::{Target, TargetType, OsintModule};
use async_trait::async_trait;
use anyhow::Result;
use reqwest;

pub struct WafCheckModule;

#[async_trait]
impl OsintModule for WafCheckModule {
    fn name(&self) -> String { "WAF Detector".to_string() }
    fn description(&self) -> String { "Detecta proteções (Cloudflare, Akamai, AWS)".to_string() }

    async fn run(&self, target: &Target, client: &reqwest::Client) -> Result<Vec<Target>> {
        if target.kind != TargetType::Domain && target.kind != TargetType::IP { return Ok(vec![]); }

        let url = if target.value.starts_with("http") { target.value.clone() } else { format!("https://{}", target.value) };

        if let Ok(resp) = client.head(&url).send().await {
            let headers = resp.headers();
            let mut wafs = Vec::new();

            if headers.contains_key("cf-ray") || headers.contains_key("__cfduid") { wafs.push("Cloudflare"); }
            if headers.contains_key("x-amz-cf-id") { wafs.push("AWS CloudFront"); }
            if headers.contains_key("x-akamai-transformed") { wafs.push("Akamai"); }
            if headers.contains_key("x-sucuri-id") { wafs.push("Sucuri"); }
            if headers.contains_key("x-protected-by") { wafs.push("Generic WAF"); }
            
            if let Some(server) = headers.get("server") {
                let s = server.to_str().unwrap_or("").to_lowercase();
                if s.contains("cloudflare") { wafs.push("Cloudflare (Server)"); }
                if s.contains("akamai") { wafs.push("Akamai (Server)"); }
            }

            let mut findings = Vec::new();
            for waf in wafs {
                findings.push(Target::new(&format!("🛡️ WAF Detectado: {}", waf), TargetType::Technology));
            }
            return Ok(findings);
        }
        Ok(vec![])
    }
}