use crate::core::types::{OsintModule, Target, TargetType};
use anyhow::Result;
use async_trait::async_trait;
use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
struct GeoResponse {
    status: String,
    country: Option<String>,
    city: Option<String>,
    lat: Option<f64>,
    lon: Option<f64>,
    isp: Option<String>,
}

pub struct GeoIpModule;

#[async_trait]
impl OsintModule for GeoIpModule {
    fn name(&self) -> String {
        "GeoIP Location Tracker".to_string()
    }
    fn description(&self) -> String {
        "Rastreia a localização física e ISP".to_string()
    }

    async fn run(&self, target: &Target, client: &reqwest::Client) -> Result<Vec<Target>> {
        if target.kind != TargetType::IP {
            return Ok(vec![]);
        }
        let ip = &target.value;
        if ip.starts_with("192.168") || ip.starts_with("127.") || ip.starts_with("10.") {
            return Ok(vec![]);
        }

        println!("┃  │   🌍  Rastreando satélite para o IP {}...", ip);
        let url = format!("http://ip-api.com/json/{}", ip);

        // USA O CLIENTE GLOBAL AGORA
        if let Ok(resp) = client.get(&url).send().await {
            if let Ok(geo) = resp.json::<GeoResponse>().await {
                if geo.status == "success" {
                    let mut findings = Vec::new();
                    if let (Some(c), Some(co)) = (geo.city, geo.country) {
                        findings.push(Target::new(
                            &format!("Loc: {}, {}", c, co),
                            TargetType::Location,
                        ));
                    }
                    if let (Some(lat), Some(lon)) = (geo.lat, geo.lon) {
                        findings.push(Target::new(
                            &format!("Coords: {}, {}", lat, lon),
                            TargetType::Location,
                        ));
                    }
                    if let Some(isp) = geo.isp {
                        findings.push(Target::new(
                            &format!("ISP: {}", isp),
                            TargetType::Technology,
                        ));
                    }
                    return Ok(findings);
                }
            }
        }
        Ok(vec![])
    }
}
