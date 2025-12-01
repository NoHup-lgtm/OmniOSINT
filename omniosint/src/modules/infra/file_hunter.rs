use crate::core::types::{Target, TargetType, OsintModule};
use async_trait::async_trait;
use anyhow::Result;
use reqwest;

pub struct FileHunterModule;

#[async_trait]
impl OsintModule for FileHunterModule {
    fn name(&self) -> String { "Sensitive File Hunter".to_string() }
    fn description(&self) -> String { "Busca documentos públicos (PDF, DOCX, XLSX)".to_string() }

    async fn run(&self, target: &Target, client: &reqwest::Client) -> Result<Vec<Target>> {
        if target.kind != TargetType::Domain { return Ok(vec![]); }

        let base_url = if target.value.starts_with("http") { target.value.clone() } else { format!("https://{}", target.value) };
        println!("┃  │   📄  Caçando documentos em {}...", target.value);

        // Arquivos comuns que empresas esquecem
        let files = vec![
            "manual.pdf", "policy.pdf", "contrato.pdf", "report.pdf",
            "funcionarios.xlsx", "salarios.xlsx", "budget.xlsx",
            "atas.docx", "resume.docx", "cv.pdf",
            "database.sql", "dump.sql"
        ];

        let mut findings = Vec::new();

        for file in files {
            let full_url = format!("{}/{}", base_url.trim_end_matches('/'), file);
            
            // Verifica apenas se existe (HEAD)
            if let Ok(resp) = client.head(&full_url).send().await {
                if resp.status().is_success() {
                    let size = resp.content_length().unwrap_or(0);
                    // Filtra arquivos vazios ou redirecionamentos falsos
                    if size > 1000 { 
                        findings.push(Target::new(&full_url, TargetType::SensitiveFile));
                        // Nota: Em uma v2.0, aqui baixaríamos o arquivo e leríamos os metadados.
                        findings.push(Target::new("METADATA: Baixe para extrair autor!", TargetType::Vulnerability));
                    }
                }
            }
        }
        Ok(findings)
    }
}