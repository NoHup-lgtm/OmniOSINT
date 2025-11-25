use crate::core::reporter::HtmlReporter;
use crate::core::types::{OsintModule, Target, TargetType};
use colored::*;
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::BufWriter;
use std::sync::Arc;

pub struct Engine {
    modules: Vec<Arc<dyn OsintModule + Send + Sync>>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
        }
    }

    pub fn register_module(&mut self, module: impl OsintModule + 'static + Send + Sync) {
        self.modules.push(Arc::new(module));
    }

    pub async fn scan(&self, initial_target: Target, output_file: Option<String>) {
        println!(
            "{}",
            format!("[*] Iniciando Scan Recursivo...").green().bold()
        );

        let mut queue = VecDeque::new();
        queue.push_back(initial_target);

        let mut seen = HashSet::new();
        let mut all_results = Vec::new();

        while let Some(target) = queue.pop_front() {
            if seen.contains(&target.value) {
                continue;
            }
            seen.insert(target.value.clone());

            all_results.push(target.clone());

            if target.kind == TargetType::OpenPort
                || target.kind == TargetType::Email
                || target.kind == TargetType::Technology
                || target.kind == TargetType::SensitiveFile
                || target.kind == TargetType::Vulnerability
            {
                continue;
            }

            println!("\n{}", format!("┏━ Analisando: {}", target).yellow().bold());

            for module in &self.modules {
                match module.run(&target).await {
                    Ok(results) => {
                        if !results.is_empty() {
                            println!(
                                "┃  ├── [{}] Detectou {} novos itens",
                                module.name().cyan(),
                                results.len()
                            );

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

        match HtmlReporter::save_report(results, &html_path) {
            Ok(_) => println!(
                "{}",
                format!("[+] Dashboard HTML salvo em: {}", html_path)
                    .green()
                    .bold()
            ),
            Err(e) => println!("{}", format!("[!] Erro HTML: {}", e).red()),
        }
    }
}
