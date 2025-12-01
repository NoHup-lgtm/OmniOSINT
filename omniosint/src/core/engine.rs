// src/core/engine.rs
use crate::core::types::{Target, TargetType, OsintModule};
use crate::core::reporter::HtmlReporter;
use colored::*;
use std::sync::Arc;
use std::collections::{VecDeque, HashSet};
use std::fs::File;
use std::io::BufWriter;
use reqwest::{Client, Proxy};

pub struct Engine {
    modules: Vec<Arc<dyn OsintModule + Send + Sync>>,
}

impl Engine {
    pub fn new() -> Self {
        Self { modules: Vec::new() }
    }

    pub fn register_module(&mut self, module: impl OsintModule + 'static + Send + Sync) {
        self.modules.push(Arc::new(module));
    }

    // ATUALIZADO: Agora aceita 'proxy_url'
    pub async fn scan(&self, initial_target: Target, output_file: Option<String>, proxy_url: Option<String>) {
        println!("{}", format!("[*] Iniciando Scan Recursivo...").green().bold());

        // --- 1. CONFIGURAÇÃO DO CLIENTE HTTP CENTRAL (STEALTH) ---
        let mut client_builder = Client::builder()
            .timeout(std::time::Duration::from_secs(15))
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko)")
            .danger_accept_invalid_certs(true);

        // Se o usuário passou --proxy, configuramos aqui
        if let Some(p) = proxy_url {
            println!("{}", format!("[🛡️] Modo Stealth Ativado via Proxy: {}", p).magenta().bold());
            match Proxy::all(&p) {
                Ok(proxy) => {
                    client_builder = client_builder.proxy(proxy);
                },
                Err(e) => {
                    println!("{}", format!("[!] Erro ao configurar Proxy: {}", e).red());
                    return;
                }
            }
        }
        
        // Cria o cliente que será usado por TODOS os módulos
        let client = client_builder.build().unwrap_or_default();
        // ---------------------------------------------------------

        let mut queue = VecDeque::new();
        queue.push_back(initial_target);

        let mut seen = HashSet::new();
        let mut all_results = Vec::new();

        while let Some(target) = queue.pop_front() {
            if seen.contains(&target.value) { continue; }
            seen.insert(target.value.clone());
            all_results.push(target.clone());

            // Filtros de parada para evitar loops infinitos
            if matches!(target.kind, 
                TargetType::OpenPort | TargetType::Email | TargetType::Technology | 
                TargetType::SensitiveFile | TargetType::Vulnerability | TargetType::Location |
                TargetType::DiscordID | TargetType::Phone
            ) {
                continue;
            }

            println!("\n{}", format!("┏━ Analisando: {}", target).yellow().bold());

            for module in &self.modules {
                // ATUALIZADO: Passamos o 'client' aqui!
                match module.run(&target, &client).await {
                    Ok(results) => {
                        if !results.is_empty() {
                            println!("┃  ├── [{}] Detectou {} novos itens", module.name().cyan(), results.len());
                            for new_target in results {
                                println!("┃  │   └── {}", new_target);
                                queue.push_back(new_target);
                            }
                        }
                    }
                    Err(e) => println!("┃  ├── [{}] Erro: {}", module.name().red(), e),
                }
            }
        }
        println!("\n{}", "[*] Varredura Finalizada.".green().bold());

        if let Some(path) = output_file {
            self.save_report(&all_results, &path);
        }
    }

    fn save_report(&self, results: &[Target], path: &str) {
        println!("{}", format!("[*] Salvando relatório em: {}", path).blue());
        
        let file = match File::create(path) {
            Ok(f) => f,
            Err(e) => {
                println!("{}", format!("[!] Erro ao criar arquivo: {}", e).red());
                return;
            }
        };

        let writer = BufWriter::new(file);
        
        match serde_json::to_writer_pretty(writer, results) {
            Ok(_) => println!("{}", "[+] Relatório JSON salvo!".green()),
            Err(e) => println!("{}", format!("[!] Erro JSON: {}", e).red()),
        }

        let html_path = path.replace(".json", ".html");
        
        // Ignoramos erro de HTML se houver
        let _ = HtmlReporter::save_report(results, &html_path);
        println!("{}", format!("[+] Dashboard HTML salvo em: {}", html_path).green().bold());
    }
}