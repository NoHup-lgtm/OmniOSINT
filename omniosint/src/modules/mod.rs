pub mod users {
    pub mod email_intel; // <--- Novo
    pub mod github;
    pub mod person_recon; // <--- Novo
    pub mod phone_discord;
    pub mod social_checker; // <--- Novo
}
pub mod infra {
    pub mod dir_fuzzer;
    pub mod dns_intel;
    pub mod file_hunter;
    pub mod geo_ip;
    pub mod http_scraper;
    pub mod portscan;
    pub mod shodan_intel;
    pub mod subdomains;
    pub mod tech_stack;
    pub mod waf_check; // <--- Novo
}
