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
use dotenv::dotenv;

use crate::core::banner;
use crate::core::engine::Engine;
use crate::core::types::{Target, TargetType};
use modules::infra::dir_fuzzer::DirFuzzerModule;
use modules::infra::dns_intel::DnsIntelModule;
use modules::infra::file_hunter::FileHunterModule;
use modules::infra::geo_ip::GeoIpModule;
use modules::infra::http_scraper::HttpScraperModule;
use modules::infra::portscan::PortScanModule;
use modules::infra::shodan_intel::ShodanIntelModule;
use modules::infra::subdomains::SubdomainModule;
use modules::infra::tech_stack::TechStackModule;
use modules::infra::waf_check::WafCheckModule;
use modules::users::email_intel::EmailIntelModule;
use modules::users::github::GithubModule;
use modules::users::person_recon::PersonReconModule;
use modules::users::phone_discord::PhoneDiscordModule;
use modules::users::social_checker::SocialCheckerModule;

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

        #[arg(short = 'p', long)]
        proxy: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    banner::print_banner();

    let cli = Cli::parse();
    let mut engine = Engine::new();

    // --- REGISTRO DE TODOS OS MÓDULOS ---
    engine.register_module(GithubModule);
    engine.register_module(SocialCheckerModule);
    engine.register_module(PersonReconModule);
    engine.register_module(EmailIntelModule);
    engine.register_module(PhoneDiscordModule);
    engine.register_module(HttpScraperModule);
    engine.register_module(PortScanModule);
    engine.register_module(SubdomainModule);
    engine.register_module(TechStackModule);
    engine.register_module(DnsIntelModule);
    engine.register_module(DirFuzzerModule);
    engine.register_module(ShodanIntelModule);
    engine.register_module(GeoIpModule);
    engine.register_module(WafCheckModule);
    engine.register_module(FileHunterModule);

    match &cli.command {
        Commands::Scan {
            target,
            kind,
            output,
            proxy,
        } => {
            let target_type = match kind.as_str() {
                "ip" => TargetType::IP,
                "username" => TargetType::Username,
                "email" => TargetType::Email,
                "name" => TargetType::RealName,
                "phone" => TargetType::Phone,
                "discord" => TargetType::DiscordID,
                _ => TargetType::Domain,
            };

            let initial_target = Target::new(target, target_type);

            engine
                .scan(initial_target, output.clone(), proxy.clone())
                .await;
        }
    }
}
