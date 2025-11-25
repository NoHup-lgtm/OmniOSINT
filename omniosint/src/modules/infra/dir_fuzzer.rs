use crate::core::types::{Target, TargetType, OsintModule};
use async_trait::async_trait;
use anyhow::Result;
use reqwest;

pub struct DirFuzzerModule;

#[async_trait]
impl OsintModule for DirFuzzerModule {
    fn name(&self) -> String { "Sensitive File Fuzzer".to_string() }
    
    #[allow(dead_code)]
    fn description(&self) -> String { "Busca arquivos críticos expostos (.env, .git, backups)".to_string() }

    async fn run(&self, target: &Target) -> Result<Vec<Target>> {
        if target.kind != TargetType::Domain && target.kind != TargetType::IP {
            return Ok(vec![]);
        }

        let base_url = if target.value.starts_with("http") {
            target.value.clone()
        } else {
            format!("https://{}", target.value)
        };

        let wordlist = vec![
            ".env",                 
            ".git/HEAD",            
            ".vscode/sftp.json",    
            "wp-config.php.bak",    
            "config.php.bak",
            "backup.zip",           
            "backup.sql",           
            "id_rsa",               
            "robots.txt",           
            "sitemap.xml",
            "server-status",        
            ".DS_Store",            
            "phpinfo.php",          
            "admin/",              
            ".well-known/security.txt" 
        ];

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(4)) 
            .user_agent("Mozilla/5.0")
            .danger_accept_invalid_certs(true)
            .build()?;

        println!("┃  │   💣  Procurando {} arquivos sensíveis em {}...", wordlist.len(), target.value);

        let mut findings = Vec::new();

        for file in wordlist {
            let full_url = format!("{}/{}", base_url.trim_end_matches('/'), file);

            let resp = client.get(&full_url).send().await;

            match resp {
                Ok(response) => {
                    if response.status().is_success() {

                        let content_len = response.content_length().unwrap_or(0);
                        if content_len > 0 {
                             findings.push(Target::new(
                                &full_url, 
                                TargetType::SensitiveFile
                            ));
                        }
                    }
                },
                Err(_) => continue,
            }
        }

        Ok(findings)
    }
}