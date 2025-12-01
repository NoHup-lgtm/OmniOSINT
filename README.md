# 👁️ OmniOSINT

> **Advanced Offensive Reconnaissance Framework written in Rust.**

![Rust](https://img.shields.io/badge/built_with-Rust-dca282.svg)
![Status](https://img.shields.io/badge/status-stable-green.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

**OmniOSINT** é um framework de inteligência de código aberto (OSINT) de alta performance. Diferente de scripts simples, ele utiliza uma arquitetura assíncrona baseada em eventos para correlacionar dados automaticamente, permitindo investigações profundas com anonimato total (Tor/Proxy).

**Author:** NoHup-ltgm

---

## 🚀 Funcionalidades

A ferramenta opera em ciclos recursivos: *Encontra um alvo -> Analisa -> Descobre novos alvos -> Repete.*

### 🛡️ Infraestrutura & Redes
| Módulo | Função |
| :--- | :--- |
| **🔎 Domain Recon** | Expande domínios usando Certificate Transparency (CRT.sh). |
| **🚪 Fast PortScan** | Scanner de portas TCP não-bloqueante. |
| **🧬 Tech Fingerprint** | Identifica tecnologias (Server, X-Powered-By). |
| **🛡️ WAF Detector** | Detecta proteções como Cloudflare, Akamai e AWS Shield. |
| **📡 DNS Intel** | Mapeia servidores de e-mail (MX) e políticas (TXT/SPF). |
| **💣 DirFuzzer** | Busca ativa por arquivos sensíveis (`.env`, `.git`, `backup.zip`). |
| **📄 File Hunter** | Encontra documentos públicos (PDF, DOCX, XLSX). |
| **👁️ Shodan Integration** | Consulta CVEs e Vulnerabilidades (API Key necessária). |
| **🌍 GeoIP** | Geolocalização física precisa de servidores e IPs. |

### 👤 Identidade & SOCMINT
| Módulo | Função |
| :--- | :--- |
| **🕵️‍♂️ Social Hunter** | Verifica contas em +20 redes sociais (Sherlock style). |
| **🧠 Person Recon** | Gera permutações de usernames a partir de Nome Real. |
| **📧 Email Intel** | Desanonimiza e-mails via Gravatar e correlaciona com Discord. |
| **📱 Phone & ID** | Analisa metadados de números de telefone e IDs do Discord (Snowflake). |

---

## 🛠️ Instalação

### Pré-requisitos
- [Rust & Cargo](https://rustup.rs/)
- [Tor Service](https://www.torproject.org/) (Opcional, para anonimato)

### Compilando

```bash
# 1. Clone o repositório
git clone [https://github.com/SEU_USUARIO/OmniOSINT.git](https://github.com/SEU_USUARIO/OmniOSINT.git)
cd OmniOSINT/omniosint

# 2. Crie o arquivo .env (Necessário para o módulo Shodan)
echo "SHODAN_API_KEY=sua_chave_aqui" > .env

# 3. Compile em modo Release (Otimizado)
cargo build --release

# 4. (Recomendado) Instale no sistema
sudo cp target/release/omniosint /usr/local/bin/
````

-----

## 💻 Manual de Uso

### Sintaxe

```bash
omniosint scan [OPTIONS] --target <ALVO>
```

### 🚩 Flags e Opções

| Flag Curta | Flag Longa | Obrigatório? | Descrição | Exemplo |
| :--- | :--- | :--- | :--- | :--- |
| **`-t`** | **`--target`** | ✅ Sim | O alvo a ser investigado. | `google.com`, `192.168.1.1` |
| **`-k`** | **`--kind`** | ❌ Não | Tipo inicial do alvo (Padrão: `domain`). | `domain`, `ip`, `username`, `name` |
| **`-o`** | **`--output`** | ❌ Não | Salva relatório em JSON e Dashboard HTML. | `report.json` |
| **`-p`** | **`--proxy`** | ❌ Não | Define Proxy HTTP/SOCKS5 para anonimato. | `socks5://127.0.0.1:9050` |

### 🎯 Tipos de Alvo (`--kind`)

  - `domain`: Empresas e Sites (ex: `tesla.com`).
  - `ip`: Servidores e Infraestrutura (ex: `142.250.1.1`).
  - `username`: Investigação de Nicknames (ex: `NoHup`).
  - `name`: Investigação de Pessoas Reais (ex: `"Arthur Araujo"`).
  - `email`: Investigação de E-mails (ex: `target@gmail.com`).
  - `phone`: Metadados de Telefone (ex: `+5511999999999`).
  - `discord`: Análise de ID do Discord (ex: `29384723984723`).

-----

## ⚡ Exemplos de Ataque

### 1\. Reconhecimento Corporativo Completo

Mapeia subdomínios, portas, WAF, arquivos e vulnerabilidades.

```bash
omniosint scan -t microsoft.com -k domain -o microsoft.json
```

### 2\. Modo Stealth (Tor/Proxy) 👻

Executa o scan através da rede Tor para evitar bloqueios de IP e manter anonimato.
*(Requer Tor rodando na porta 9050)*

```bash
omniosint scan -t "alvo.com" -p socks5://127.0.0.1:9050
```

### 3\. Caça Humana (Pessoa Real)

Gera permutações do nome e busca em todas as redes sociais.

```bash
omniosint scan -t "Nome Sobrenome" -k name -o dossie.json
```

### 4\. Investigação de E-mail

Tenta encontrar o dono do e-mail via Gravatar e sugere usuário do Discord.

```bash
omniosint scan -t "alvo@empresa.com" -k email
```

-----

## 📊 Visualização

A ferramenta gera automaticamente um **Dashboard Interativo** (`.html`) baseado em grafos.
Basta abrir o arquivo gerado no navegador para visualizar as conexões entre os alvos.

-----

**⚠️ Aviso Legal:** Ferramenta desenvolvida para fins educacionais e auditorias autorizadas.

```

---

### 3. Cheat Sheet: Todos os Comandos 💀

Aqui está a lista rápida para você copiar e colar no dia a dia.

#### **Iniciação**
* **Instalar:** `cargo build --release && sudo cp target/release/omniosint /usr/local/bin/`
* **Iniciar Tor:** `sudo systemctl start tor`

#### **Tipos de Scan**

1.  **Scan de Site (Padrão):**
    `omniosint scan -t google.com`

2.  **Scan de IP (Infra):**
    `omniosint scan -t 8.8.8.8 -k ip`

3.  **Scan de Username (Social):**
    `omniosint scan -t usuario_alvo -k username`

4.  **Scan de Nome Real (Gera users):**
    `omniosint scan -t "Fulano da Silva" -k name`

5.  **Scan de E-mail (Gravatar/Discord):**
    `omniosint scan -t alvo@gmail.com -k email`

6.  **Scan de Telefone:**
    `omniosint scan -t +5511999999999 -k phone`

7.  **Scan de Discord ID:**
    `omniosint scan -t 4562234234234 -k discord`

#### **Opções Avançadas**

* **Salvar Relatório (JSON + HTML):**
    Adicione: `-o resultado.json`

* **Usar Proxy Tor (Anonimato):**
    Adicione: `-p socks5://127.0.0.1:9050`

* **Usar Proxy HTTP Comum:**
    Adicione: `-p http://200.10.10.10:8080`