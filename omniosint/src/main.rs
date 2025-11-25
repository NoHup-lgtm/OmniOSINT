/*
 * ____                  _  ____  _____ ___ _   _ _______ 
 * |  _ \                (_)/ __ \| ____|_ _| \ | |__   __|
 * | |_) |_   _    / \   _ | |  | | (___ | ||  \| |  | |   
 * |  _ <| | | |  / _ \ | || |  | |\___ \| || . ` |  | |   
 * | |_) | |_| | / ___ \| || |__| |____) | || |\  |  | |   
 * |____/ \__, |/_/   \_\_| \____/|_____/___|_| \_|  |_|   
 * __/ |                                           
 * |___/                                            
 *
 * OMNIOSINT - Open Source Intelligence Framework
 * --------------------------------------------------
 * Author: NoHup-lgtm
 * License: MIT
 * "Watching the watchers."
 */

mod core;
mod modules;

use clap::{Parser, Subcommand};
use core::engine::Engine;
use core::banner;
use core::types::{Target, TargetType};
use dotenv::dotenv;
use modules::users::github::GithubModule;
use modules::infra::http_scraper::HttpScraperModule;
use modules::infra::portscan::PortScanModule;
use modules::infra::subdomains::SubdomainModule;
use modules::infra::tech_stack::TechStackModule;
use modules::infra::dns_intel::DnsIntelModule; 
use modules::infra::dir_fuzzer::DirFuzzerModule;
use modules::infra::shodan_intel::ShodanIntelModule;


#[derive(Parser)]
#[command(name = "OmniOSINT")]
#[command(about = "Ferramenta All-in-One de Inteligência Open Source", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Scan {
        #[arg(short, long)]
        target: String,
        
        #[arg(short = 'k', long, default_value = "domain")]
        kind: String,

        #[arg(short = 'o', long)]
        output: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    banner::print_banner();
    let cli = Cli::parse();
    let mut engine = Engine::new();

    engine.register_module(GithubModule);
    engine.register_module(HttpScraperModule);
    engine.register_module(PortScanModule);
    engine.register_module(SubdomainModule);
    engine.register_module(TechStackModule);
    engine.register_module(DnsIntelModule); 
    engine.register_module(DirFuzzerModule);
    engine.register_module(ShodanIntelModule);

    match &cli.command {
        Commands::Scan { target, kind, output } => {
            let target_type = match kind.as_str() {
                "ip" => TargetType::IP,
                "username" => TargetType::Username,
                "email" => TargetType::Email,
                _ => TargetType::Domain,
            };

            let initial_target = Target::new(target, target_type);
            engine.scan(initial_target, output.clone()).await; 
        }
    }
}
