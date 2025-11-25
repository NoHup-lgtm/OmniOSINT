# 👁️ OmniOSINT

> **Offensive Reconnaissance and Modular Intelligence Framework in Rust.**

**OmniOSINT** is a high-performance command-line tool (CLI) designed for the reconnaissance phase of penetration testing and Bug Bounty hunting. Unlike simple scripts, it utilizes an asynchronous, event-based architecture to automatically correlate data.

-----

## 🚀 Features

The tool operates in recursive cycles: *Finds a target -\> Analyzes -\> Discovers new targets -\> Repeats.*

| Module | Function |
| :--- | :--- |
| **🔎 Domain Recon** | Expands domains using Certificate Transparency (CRT.sh). |
| **🚪 Fast PortScan** | Non-blocking TCP port scanner (Async/Tokio). |
| **🧬 Tech Fingerprint** | Identifies technologies via HTTP Headers (Server, X-Powered-By). |
| **📡 DNS Intel** | Maps email servers (MX) and security policies (TXT/SPF). |
| **💣 DirFuzzer** | Active search for sensitive files (`.env`, `.git`, `backup.zip`). |
| **👁️ Shodan Integration** | Queries known vulnerabilities (CVEs) on the infrastructure (API Key required). |
| **📊 Smart Reporting** | Generates **JSON** reports and Interactive Graph Dashboards in **HTML**. |

-----

## 🛠️ Installation

### Prerequisites

  - [Rust & Cargo](https://rustup.rs/) installed.

### Compiling from Source

```bash
# 1. Clone the repository
git clone https://github.com/YOUR_USERNAME/OmniOSINT.git
cd OmniOSINT/omniosint

# 2. Create the .env file (Optional, for Shodan)
echo "SHODAN_API_KEY=your_key_here" > .env

# 3. Build in Release mode (Optimized)
cargo build --release

# 4. (Optional) Install to system
sudo cp target/release/omniosint /usr/local/bin/
```

-----

## 💻 Usage

### Basic Scan

Performs full reconnaissance on a username or domain.

```bash
omniosint scan --target github.com --kind domain -o report.json
```

### Arguments

  - `-t, --target`: The initial target (Domain, IP, Username).
  - `-k, --kind`: The target type (`domain`, `ip`, `username`).
  - `-o, --output`: Path to save the report (generates .json and .html).

-----

## 📊 Visualization (Dashboard)

When running with the `-o` flag, the tool automatically generates an interactive connection graph in an `.html` file.

1.  Run the scan.
2.  Open the generated file:
    ```bash
    open report.html
    ```

-----

## 🏗️ Architecture

The project follows **Clean Architecture** and **Modularity** principles:

  - **Core:** Asynchronous decision engine that manages the scan queue and prevents loops.
  - **Modules:** Isolated Traits that allow plugging in new features without altering the core.
  - **Reporter:** Decoupled export system.

-----

## ⚠️ Legal Disclaimer

This tool was developed for educational purposes and authorized auditing. The author is not responsible for any misuse. **Do not scan targets without permission.**

-----

**Coded with 🦀 and coffee.**