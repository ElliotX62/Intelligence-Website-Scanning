Here I will explain the tasks and purposes of each file and folder in the directory architecture.
let's get started

explanation of each file in the root directory:

---

Alright, I will explain sequentially each folder and its contents. Let's start with the files in the project root first.

---

## 📁 **PROJECT ROOT - MAIN FILES**

---

### **1. `iws.py`**

**Task:** The main system entry point that serves as the first gateway when the user runs the `iws` command in the terminal.

**Purpose:** This file is responsible for initializing the entire system environment before running the main functionality. When executed, this file will display the system's ASCII art banner to provide visual identity, check that all required dependencies are installed with appropriate versions, load configuration from the `.env` file using python-dotenv with fallback to default values if any variables are not found, initialize the logging system with rotating file handlers (10MB per file with a maximum of 5 rotations), and launch the interactive Command Line Interface (CLI) with an asyncio event loop to efficiently process user commands. This file acts as the central brain that coordinates all system components from start to finish.

---

### **2. `main.py`**

**Task:** Provides an alternative entry point for development and debugging purposes.

**Purpose:** This file is specifically designed for the development environment by including debug mode flags that allow developers to run the system with more detailed logging. It uses argparse for command-line parsing with arguments such as `--debug` to enable debug mode, `--config` to specify an alternative configuration file, and `--log-level` to set the logging detail level. This file also triggers hot reload on file changes using the watchdog library when in debug mode, so developers don't need to manually restart every time there is a code change. It provides profiler integration using cProfile to perform performance analysis and identify bottlenecks in the code.

---

### **3. `Cargo.toml`**

**Task:** Manifest file for all Rust components in the project.

**Purpose:** Defines the workspace with members for all Rust sub-projects including shared contracts and types. Specifies the `release` profile with lto = true and codegen-units = 1 for maximum optimization in production builds. Defines all required Rust dependencies with specific versions (such as tokio for async runtime, reqwest for HTTP client, serde for serialization, clap for CLI parsing) and enabled features. Sets `[features]` for conditional compilation that allows building with different configurations (default, minimal, full) according to deployment needs.

---

### **4. `go.mod`**

**Task:** Manifest file for all Go components in the project.

**Purpose:** Defines the module path with a version compliant with semantic versioning (semver). Specifies the minimum required Go version (1.21) to ensure compatibility with the latest features. Declares all required Go dependencies with specific version constraints. Sets `replace` directives for local development if needed, allowing module replacement with local versions for testing and debugging purposes.

---

### **5. `requirements.txt`**

**Task:** Complete list of all Python dependencies required to run the project.

**Purpose:** Lists all Python packages with specific version pins (example: `requests==2.31.0`) to ensure reproducibility and avoid version conflicts. Organizes them in sections with comment headers like # Core, # AI, # Network, # Database, # Utils for easier navigation and understanding. Includes hashes for security verification using the `--hash` option, enabling package integrity verification during installation.

---

### **6. `setup.py`**

**Task:** File for Python packaging that allows `iws` to be installed as a system-installed package.

**Purpose:** Implements `setuptools.setup()` with `find_packages()` for auto-discovery of all Python packages in the project. Defines console script entry points `iws = iws:main` so the `iws` command can be run from the terminal after installation. Specifies `install_requires` parsed from requirements.txt for dependency consistency. Includes classifiers for PyPI categorization that aids in package distribution and discovery.

---

### **7. `.env_template`**

**Task:** Template for environment variables file containing all configurations required by the system.

**Purpose:** Defines all environment variables in `KEY=VALUE` format with `YOUR_API_KEY_HERE` placeholders to help users fill in the correct values. Includes `API_KEYS_SECTION` with comments for each service (Shodan, Censys, VirusTotal, AlienVault, URLScan, SecurityTrails, GreyHat Warfare). Includes `DATABASE_URL` for database connection, `REDIS_URL` for Redis cache connection, `LOG_LEVEL` to set logging level, `MAX_THREADS` to set maximum thread count, `TIMEOUT_SECONDS` for default timeout, and `USER_AGENT_ROTATION` flag to enable user agent rotation.

---

### **8. `.gitignore`**

**Task:** Defines files and folders that do not need to be version-controlled by Git.

**Purpose:** Implements exclusion patterns to protect sensitive files and reduce repository size. Includes `.env` to protect API keys and credentials, `*.pyc` and `__pycache__/` for Python cache files, `data/` containing scan results, `logs/` containing log files, `*.db` for database files, `*.log` for log files, `*.tmp` for temporary files, `*.swp` for Vim swap files, `.DS_Store` for macOS metadata files, `target/` for Rust build artifacts, and `vendor/` for Go dependencies. Adds patterns for IDE files (`.vscode/`, `.idea/`) to keep the repository clean.

---

### **9. `README.md`**

**Task:** Main project documentation that serves as the first guide for users and contributors.

**Purpose:** Structures documentation with a professional layout: Header with badges (version, license, build status) to provide quick information. Installation section for all supported platforms (Termux, Linux, Windows, macOS). Usage examples with code blocks showing basic commands. Features list with icons for visual appeal. Architecture overview with mermaid diagrams to explain the system flow. Configuration guide to help users set up the system. API documentation link for developers. Contributing guidelines for contributors. License information for legal use.

---

### **10. `LICENSE`**

**Task:** Defines the legal license for project use and distribution.

**Purpose:** Includes the full license text (MIT, GPL-3.0, or Apache-2.0) according to the developer's choice. Defines the copyright year and holder name for copyright protection. Includes a disclaimer of warranty that releases the developer from responsibility for any damages that may occur. Includes a limitation of liability to limit the developer's legal responsibility.

---

Indonesian:

---

## 📁 **ROOT PROYEK - FILE UTAMA**

---

### **1. `iws.py`**

**Tugas:** Entry point utama sistem yang menjadi pintu gerbang pertama ketika pengguna menjalankan perintah `iws` di terminal.

**Tujuan:** File ini bertanggung jawab untuk menginisialisasi seluruh lingkungan sistem sebelum menjalankan fungsionalitas utama. Ketika dieksekusi, file ini akan menampilkan banner ASCII art sistem untuk memberikan identitas visual, memeriksa semua dependensi yang diperlukan sudah terinstal dengan versi yang sesuai, memuat konfigurasi dari file `.env` menggunakan python-dotenv dengan fallback ke nilai default jika ada variabel yang tidak ditemukan, menginisialisasi sistem logging dengan rotating file handler (10MB per file dengan maksimal 5 rotasi), dan meluncurkan antarmuka Command Line Interface (CLI) interaktif dengan asyncio event loop untuk memproses perintah pengguna secara efisien. File ini bertindak sebagai otak pusat yang mengoordinasikan seluruh komponen sistem dari awal hingga akhir.

---

### **2. `main.py`**

**Tugas:** Menyediakan alternatif entry point untuk keperluan development dan debugging.

**Tujuan:** File ini dirancang khusus untuk environment pengembangan dengan menyertakan flag debug mode yang memungkinkan developer menjalankan sistem dengan logging lebih detail. Menggunakan argparse untuk parsing command line dengan argument seperti `--debug` untuk mengaktifkan mode debug, `--config` untuk menentukan file konfigurasi alternatif, dan `--log-level` untuk mengatur tingkat detail logging. File ini juga memicu hot reload pada perubahan file menggunakan library watchdog ketika dalam mode debug, sehingga developer tidak perlu restart manual setiap kali ada perubahan kode. Menyediakan profiler integration menggunakan cProfile untuk melakukan performance analysis dan mengidentifikasi bottleneck dalam kode.

---

### **3. `Cargo.toml`**

**Tugas:** File manifest untuk semua komponen Rust dalam proyek.

**Tujuan:** Mendefinisikan workspace dengan members untuk semua sub-proyek Rust termasuk shared contracts dan types. Menentukan profile `release` dengan lto = true dan codegen-units = 1 untuk optimasi maksimum pada build production. Mendefinisikan semua dependensi Rust yang dibutuhkan dengan versi spesifik (seperti tokio untuk async runtime, reqwest untuk HTTP client, serde untuk serialization, clap untuk CLI parsing) dan fitur yang diaktifkan. Mengatur `[features]` untuk conditional compilation yang memungkinkan build dengan konfigurasi berbeda (default, minimal, full) sesuai kebutuhan deployment.

---

### **4. `go.mod`**

**Tugas:** File manifest untuk semua komponen Go dalam proyek.

**Tujuan:** Mendefinisikan module path dengan versi yang compliant dengan semantic versioning (semver). Menentukan versi Go minimal yang dibutuhkan (1.21) untuk memastikan kompatibilitas dengan fitur-fitur terbaru. Mendeklarasikan semua dependensi Go yang diperlukan dengan version constraints yang spesifik. Mengatur `replace` directives untuk development lokal jika diperlukan, memungkinkan penggantian modul dengan versi lokal untuk keperluan testing dan debugging.

---

### **5. `requirements.txt`**

**Tugas:** Daftar lengkap semua dependensi Python yang diperlukan untuk menjalankan proyek.

**Tujuan:** Mendaftar semua package Python dengan version pins yang spesifik (contoh: `requests==2.31.0`) untuk memastikan reproduksibilitas dan menghindari konflik versi. Mengorganisir dalam sections dengan comment headers seperti # Core, # AI, # Network, # Database, # Utils untuk memudahkan navigasi dan pemahaman. Menyertakan hashes untuk security verification menggunakan `--hash` option, memungkinkan verifikasi integritas package saat instalasi.

---

### **6. `setup.py`**

**Tugas:** File untuk packaging Python yang memungkinkan instalasi `iws` sebagai paket yang terinstal di sistem.

**Tujuan:** Mengimplementasikan `setuptools.setup()` dengan `find_packages()` untuk auto-discovery semua package Python dalam proyek. Mendefinisikan entry points console scripts `iws = iws:main` sehingga perintah `iws` dapat dijalankan dari terminal setelah instalasi. Menentukan `install_requires` dari requirements.txt yang di-parse untuk konsistensi dependensi. Menyertakan classifiers untuk PyPI categorization yang membantu dalam distribusi dan discovery package.

---

### **7. `.env_template`**

**Tugas:** Template untuk file environment variables yang berisi semua konfigurasi yang diperlukan sistem.

**Tujuan:** Mendefinisikan semua environment variables dengan format `KEY=VALUE` dan placeholder `YOUR_API_KEY_HERE` untuk memudahkan pengguna mengisi nilai yang benar. Mencakup `API_KEYS_SECTION` dengan comment untuk setiap service (Shodan, Censys, VirusTotal, AlienVault, URLScan, SecurityTrails, GreyHat Warfare). Menyertakan `DATABASE_URL` untuk koneksi database, `REDIS_URL` untuk koneksi Redis cache, `LOG_LEVEL` untuk mengatur tingkat logging, `MAX_THREADS` untuk mengatur jumlah thread maksimum, `TIMEOUT_SECONDS` untuk timeout default, dan `USER_AGENT_ROTATION` flag untuk mengaktifkan rotasi user agent.

---

### **8. `.gitignore`**

**Tugas:** Mendefinisikan file dan folder yang tidak perlu di-version control oleh Git.

**Tujuan:** Mengimplementasikan exclusion pattern untuk melindungi file sensitif dan mengurangi ukuran repository. Mencakup `.env` untuk melindungi API keys dan credentials, `*.pyc` dan `__pycache__/` untuk file cache Python, `data/` yang berisi hasil scanning, `logs/` yang berisi file log, `*.db` untuk database files, `*.log` untuk log files, `*.tmp` untuk temporary files, `*.swp` untuk Vim swap files, `.DS_Store` untuk file metadata macOS, `target/` untuk build artifacts Rust, dan `vendor/` untuk dependencies Go. Menambahkan pattern untuk IDE files (`.vscode/`, `.idea/`) untuk menjaga kebersihan repository.

---

### **9. `README.md`**

**Tugas:** Dokumentasi utama proyek yang menjadi panduan pertama bagi pengguna dan kontributor.

**Tujuan:** Menyusun dokumentasi dengan structure yang profesional: Header dengan badge (version, license, build status) untuk memberikan informasi cepat. Installation section untuk semua platform yang didukung (Termux, Linux, Windows, macOS). Usage examples dengan code blocks yang menunjukkan command-command dasar. Features list dengan icons untuk visualisasi yang menarik. Architecture overview dengan mermaid diagram untuk menjelaskan alur sistem. Configuration guide untuk membantu pengguna mengatur sistem. API documentation link untuk developer. Contributing guidelines untuk kontributor. License information untuk penggunaan legal.

---

### **10. `LICENSE`**

**Tugas:** Mendefinisikan lisensi legal untuk penggunaan dan distribusi proyek.

**Tujuan:** Menyertakan full text license (MIT, GPL-3.0, atau Apache-2.0) sesuai pilihan pengembang. Mendefinisikan copyright year dan holder name untuk perlindungan hak cipta. Mencantumkan disclaimer of warranty yang membebaskan pengembang dari tanggung jawab atas kerusakan yang mungkin terjadi. Mencantumkan limitation of liability untuk membatasi tanggung jawab hukum pengembang.

---

Explanation of each file in the core folder section of the directory architecture:Explanation of each file in the core folder section of the directory architecture:

---

## 📁 **`core/` - SYSTEM CORE**

---

### **1. `scanner.rs` (Rust)**

**Task:** Manages the entire website scanning process from start to finish.

**Purpose:** This file is the main component responsible for coordinating all scanning modules (network, content, security, infrastructure, intelligence) in performing scans against target websites. The scanner implements scanning strategies based on the selected profile (aggressive for maximum speed, moderate for balance, stealth to avoid detection, or custom that can be configured by the user). Manages parallelization with thread pooling using `tokio::spawn` and `Semaphore` for concurrency control, timeout for each request to prevent hanging, and retry mechanism with exponential backoff if failures occur. Each module is run in parallel using `futures::stream::FuturesUnordered` for maximum efficiency. The scanner also stores raw scanning results into structured data ready to be processed by the analyzer, and provides a `cancel_scan()` function to stop ongoing scanning with graceful shutdown through a broadcast channel.

---

### **2. `analyzer.rs` (Rust)**

**Task:** Deeply analyzes scan result data to find vulnerabilities and security patterns.

**Purpose:** This file runs a complex analysis pipeline on data collected by the scanner. The pipeline consists of several stages: Stage 1 performs Data preprocessing (normalization, cleaning, deduplication) to clean data from noise. Stage 2 performs Pattern detection with `regex` and `aho-corasick` for efficient multiple pattern matching. Stage 3 performs Vulnerability matching with a CVE database loaded into memory (from `cve_data.json`) to match detected software versions with known vulnerabilities. Stage 4 performs Risk calculation using the formula `risk = (impact * likelihood) * business_factor` while considering CVSS (Common Vulnerability Scoring System) metrics and business context. Uses `rayon` for parallel processing in independent stages. Implements `cross_reference_analysis()` to connect findings between modules (for example: open ports with service version and CVE matching) resulting in more comprehensive and accurate analysis.

---

### **3. `orchestrator.go` (Go)**

**Task:** Manages the end-to-end workflow of the entire system in a coordinated manner.

**Purpose:** This file is the orchestra conductor that decides when to scan, when to run analysis, when to generate reports, and when to save data. Manages the state machine from IDLE → INITIALIZING → SCANNING → ANALYZING → REPORTING → COMPLETE → ERROR to ensure each stage runs in the correct order. Handles error handling and fallback mechanisms if failures occur in any stage, with retry logic using exponential backoff (1s, 2s, 4s, 8s, max 30s). Uses `context.WithTimeout` for each workflow step so no process hangs for too long. Implements the main loop with select statement for efficient multi-channel handling, and uses `sync.Map` for thread-safe state storage accessible by multiple goroutines.

---

### **4. `extractor.py` (Python)**

**Task:** Extracts specific information from raw scan result data.

**Purpose:** This file uses a combination of regex, Natural Language Processing (NLP), and pattern matching to extract valuable data from scanned website content. The `extract_emails()` method uses regex patterns from the `email-validator` library with optional SMTP validation via `smtplib` to ensure emails are truly valid. `extract_technologies()` uses the `Wappalyzer` library with a custom fingerprint database to detect frameworks, libraries, and technologies used by the website. `extract_api_keys()` uses pattern matching for common key formats (AWS keys with format `AKIA[0-9A-Z]{16}`, Google API keys with format `AIza[0-9A-Za-z\\-_]{35}`, Stripe keys with format `sk_live_[A-Za-z0-9]{24,32}`, GitHub tokens with format `ghp_[A-Za-z0-9]{36}`). Implements `extract_all()` with parallel extraction using `concurrent.futures.ThreadPoolExecutor` for maximum efficiency. Stores results in an `ExtractedData` dataclass with type hints for easy use by other components.

---

### **5. `validator.rs` (Rust)**

**Task:** Validates the integrity and accuracy of scan result data.

**Purpose:** This file is responsible for checking various security and validity aspects of collected data. `validate_ssl()` uses `rustls` and `webpki` for certificate chain validation, checking expiration, hostname validation, and revocation status through CRL or OCSP with `reqwest` for network requests. `validate_headers()` checks HTTP headers according to security standards (HSTS to enforce HTTPS, CSP to prevent XSS, X-Frame-Options to prevent clickjacking, X-Content-Type-Options to prevent MIME sniffing, Referrer-Policy to control referer information) with each rule having a severity level (CRITICAL, HIGH, MEDIUM, LOW). `check_mixed_content()` uses the `html5ever` parser to detect HTTP resources on HTTPS pages that can cause security warnings. `validate_redirects()` detects open redirect vulnerabilities and redirect chain issues that can be exploited for phishing attacks.

---

### **6. `engine.rs` (Rust)**

**Task:** Main engine that connects and manages all core components.

**Purpose:** This file is the heart of the system that manages dependency injection between components using the `EngineBuilder` pattern with methods `with_scanner()`, `with_analyzer()`, `with_orchestrator()`, `build() -> Engine`. Provides an event bus for inter-module communication through `mpsc::channel` with priority queue (high priority for scan commands, medium for analysis, low for reporting). Manages the lifecycle of each component (startup, running, shutdown) with `State` enum (Uninitialized, Initializing, Running, Pausing, Paused, ShuttingDown, Shutdown). Implements `start()` which initializes all components in the correct order (scanner -> analyzer -> orchestrator) and creates a thread pool with `tokio::runtime` for non-blocking operations. Implements `shutdown()` with graceful shutdown signal: sends `SIGTERM` to all components, waits 30 seconds for pending operations to complete, and force stops if timeout is reached. Implements `event_loop()` which processes events from all components through the `event_bus`.

---

Indonesian:

---

## 📁 **`core/` - INTI SISTEM**

---

### **1. `scanner.rs` (Rust)**

**Tugas:** Mengelola seluruh proses scanning website dari awal hingga akhir.

**Tujuan:** File ini adalah komponen utama yang bertanggung jawab untuk mengoordinasikan semua modul scanning (network, content, security, infrastructure, intelligence) dalam melakukan pemindaian terhadap target website. Scanner mengimplementasikan strategi scanning berdasarkan profile yang dipilih (aggressive untuk kecepatan maksimum, moderate untuk keseimbangan, stealth untuk menghindari deteksi, atau custom yang dapat dikonfigurasi pengguna). Mengatur paralelisasi dengan thread pooling menggunakan `tokio::spawn` dan `Semaphore` untuk concurrency control, timeout untuk setiap request agar tidak menggantung, dan retry mechanism dengan exponential backoff jika terjadi kegagalan. Setiap module di-run secara paralel menggunakan `futures::stream::FuturesUnordered` untuk efisiensi maksimum. Scanner juga menyimpan hasil scanning mentah ke dalam structured data yang siap diproses oleh analyzer, serta menyediakan fungsi `cancel_scan()` untuk menghentikan scanning yang sedang berjalan dengan graceful shutdown melalui broadcast channel.

---

### **2. `analyzer.rs` (Rust)**

**Tugas:** Menganalisis data hasil scanning secara mendalam untuk menemukan kerentanan dan pola keamanan.

**Tujuan:** File ini menjalankan pipeline analisis kompleks pada data yang telah dikumpulkan oleh scanner. Pipeline terdiri dari beberapa stage: Stage 1 melakukan Data preprocessing (normalisasi, cleaning, deduplication) untuk membersihkan data dari noise. Stage 2 melakukan Pattern detection dengan `regex` dan `aho-corasick` untuk multiple pattern matching yang efisien. Stage 3 melakukan Vulnerability matching dengan CVE database yang di-load ke memory (dari `cve_data.json`) untuk mencocokkan versi software yang terdeteksi dengan kerentanan yang diketahui. Stage 4 melakukan Risk calculation menggunakan formula `risk = (impact * likelihood) * business_factor` dengan mempertimbangkan CVSS (Common Vulnerability Scoring System) metrics dan konteks bisnis. Menggunakan `rayon` untuk parallel processing di stage-stage yang independen. Implementasi `cross_reference_analysis()` untuk menghubungkan temuan antar module (misalnya: port terbuka dengan service version dan CVE matching) sehingga menghasilkan analisis yang lebih komprehensif dan akurat.

---

### **3. `orchestrator.go` (Go)**

**Tugas:** Mengatur alur kerja end-to-end dari seluruh sistem secara terkoordinasi.

**Tujuan:** File ini adalah konduktor orkestra yang memutuskan kapan harus melakukan scanning, kapan menjalankan analisis, kapan menggenerate report, dan kapan harus menyimpan data. Mengelola state machine dari IDLE → INITIALIZING → SCANNING → ANALYZING → REPORTING → COMPLETE → ERROR untuk memastikan setiap tahap berjalan dalam urutan yang benar. Menangani error handling dan fallback mechanisms jika terjadi kegagalan di salah satu tahap, dengan retry logic menggunakan exponential backoff (1s, 2s, 4s, 8s, max 30s). Menggunakan `context.WithTimeout` untuk setiap workflow step agar tidak ada proses yang menggantung terlalu lama. Mengimplementasikan main loop dengan select statement untuk multi-channel handling yang efisien, dan menggunakan `sync.Map` untuk thread-safe state storage yang dapat diakses oleh multiple goroutines.

---

### **4. `extractor.py` (Python)**

**Tugas:** Mengekstrak informasi spesifik dari data mentah hasil scanning.

**Tujuan:** File ini menggunakan kombinasi regex, Natural Language Processing (NLP), dan pattern matching untuk mengambil data berharga dari konten website yang telah di-scan. Metode `extract_emails()` menggunakan regex pattern dari `email-validator` library dengan validasi SMTP optional melalui `smtplib` untuk memastikan email benar-benar valid. `extract_technologies()` menggunakan `Wappalyzer` library dengan custom fingerprint database untuk mendeteksi framework, library, dan teknologi yang digunakan website. `extract_api_keys()` menggunakan pattern matching untuk common key formats (AWS keys dengan format `AKIA[0-9A-Z]{16}`, Google API keys dengan format `AIza[0-9A-Za-z\\-_]{35}`, Stripe keys dengan format `sk_live_[A-Za-z0-9]{24,32}`, GitHub tokens dengan format `ghp_[A-Za-z0-9]{36}`). Implementasi `extract_all()` dengan parallel extraction menggunakan `concurrent.futures.ThreadPoolExecutor` untuk efisiensi maksimum. Menyimpan hasil dalam `ExtractedData` dataclass dengan type hints untuk memudahkan penggunaan oleh komponen lain.

---

### **5. `validator.rs` (Rust)**

**Tugas:** Memvalidasi integritas dan keakuratan data hasil scan.

**Tujuan:** File ini bertanggung jawab untuk memeriksa berbagai aspek keamanan dan validitas dari data yang dikumpulkan. `validate_ssl()` menggunakan `rustls` dan `webpki` untuk certificate chain validation, memeriksa expiration, hostname validation, dan revocation status melalui CRL atau OCSP dengan `reqwest` untuk network request. `validate_headers()` memeriksa HTTP headers sesuai standar keamanan (HSTS untuk memaksa HTTPS, CSP untuk mencegah XSS, X-Frame-Options untuk mencegah clickjacking, X-Content-Type-Options untuk mencegah MIME sniffing, Referrer-Policy untuk mengontrol informasi referer) dengan setiap rule memiliki severity level (CRITICAL, HIGH, MEDIUM, LOW). `check_mixed_content()` menggunakan `html5ever` parser untuk mendeteksi HTTP resources pada HTTPS pages yang dapat menyebabkan security warnings. `validate_redirects()` mendeteksi open redirect vulnerabilities dan redirect chain issues yang dapat dieksploitasi untuk phishing attacks.

---

### **6. `engine.rs` (Rust)**

**Tugas:** Mesin utama yang menghubungkan dan mengelola semua komponen core.

**Tujuan:** File ini adalah jantung dari sistem yang mengelola dependency injection antar komponen menggunakan `EngineBuilder` pattern dengan methods `with_scanner()`, `with_analyzer()`, `with_orchestrator()`, `build() -> Engine`. Menyediakan event bus untuk komunikasi antar modul melalui `mpsc::channel` dengan priority queue (high priority untuk scan commands, medium untuk analysis, low untuk reporting). Mengelola lifecycle dari setiap komponen (startup, running, shutdown) dengan `State` enum (Uninitialized, Initializing, Running, Pausing, Paused, ShuttingDown, Shutdown). Implementasi `start()` yang menginisialisasi semua komponen dalam urutan yang tepat (scanner -> analyzer -> orchestrator) dan membuat thread pool dengan `tokio::runtime` untuk non-blocking operations. Implementasi `shutdown()` dengan graceful shutdown signal: mengirim `SIGTERM` ke semua components, menunggu 30 detik untuk pending operations selesai, dan force stop jika timeout tercapai. Mengimplementasikan `event_loop()` yang memproses events dari semua components melalui `event_bus`.

---

Explanation of the network section modules folder:

---

## 📁 **`modules/` - FUNCTIONAL MODULES**

---

### 📂 **`modules/network/` - Network Scanning**

#### **1. `dns_enum.rs` (Rust)**

**Task:** Performs complete DNS enumeration to collect all information related to the target domain.

**Purpose:** This file is responsible for collecting all available DNS record types from the target domain using `trust-dns-resolver`. Records collected include: A (IPv4 address), AAAA (IPv6 address), CNAME (canonical name/alias), MX (mail exchange servers with priority), TXT (text records including SPF for email authentication, DKIM for email signing, DMARC for email policy), NS (nameservers managing the domain), SOA (start of authority with primary nameserver, admin email, serial number, refresh/retry/expire intervals), and SRV (service records for protocols such as SIP, LDAP). Attempts zone transfer using `AXFR` query to get all records at once if the DNS server is not securely configured. Searches for subdomains using brute force and dictionary attack techniques with a wordlist of 50,000+ common subdomains (admin, dev, staging, api, app, mail, ftp, blog, shop, forum, support, test, backup) using rate limiting with `Semaphore::new(50)` to avoid detection. Valid subdomain results are stored in a `HashSet` for deduplication.

---

#### **2. `port_scanner.go` (Go)**

**Task:** Scans TCP and UDP ports on the target to discover running services.

**Purpose:** This file implements various port scanning methods to detect running services. `ScanTCP()` uses `net.DialTimeout` with worker pool pattern for concurrency (create N workers = threads, each worker reads from portsChan, performs dial, sends results to resultsChan). `ScanSYN()` uses raw sockets with `golang.org/x/net/ipv4` to craft SYN packets (stealth scan) and listens for SYN-ACK responses with `conn.ReadFrom`. `ScanUDP()` to detect open UDP services. `FIN/NULL/XMAS` scan to bypass firewalls by sending non-standard flags. After ports are discovered, `ServiceDetection()` connects to the port, sends probes (HTTP, SMTP, FTP, SSH, MySQL, PostgreSQL, MongoDB, Redis, etc), and analyzes banner responses with regex patterns to detect service name and version (examples: "220" for FTP, "SSH-2.0" for SSH, "220 mail" for SMTP).

---

#### **3. `whois_lookup.py` (Python)**

**Task:** Retrieves and parses WHOIS data from the target domain.

**Purpose:** This file uses the `whois` library with custom server selection based on TLD (.com, .org, .net, .id, .uk, etc) to obtain domain ownership information. Implements `parse_whois()` with regex patterns to extract fields: `Registrar` (registrar company name), `Creation Date` (date domain was first registered), `Expiration Date` (domain expiration date), `Updated Date` (last update date), `Name Servers` (list of nameservers used), `Registrant Email` (domain owner's email), `Registrant Organization` (owner organization name), `Registrant Country` (owner's country), `Registrant Phone` (owner's phone number). For `Registrar` validation uses cache from IANA list to ensure the registrar is officially registered. Implements `check_availability(domain: str)` to check if the domain is available (not yet registered). Stores results in `WhoisData` dataclass with `__post_init__` to convert string dates to `datetime` objects.

---

#### **4. `traceroute_analyzer.rs` (Rust)**

**Task:** Traces and analyzes the network path to the target server.

**Purpose:** This file sends UDP packets to a high port (33434) with incrementing TTL (Time-To-Live) using the `TOS` field set for `IP_TOS` (0x10). Each hop returns an ICMP Time Exceeded response captured using `socket2` for raw socket access. For each hop, records IP address, RTT (round trip time) in ms (with 3 probes per hop for accuracy), and hostname (reverse DNS lookup using `trust-dns-resolver`). Implements `geolocate_hops()` using the `maxminddb` library with GeoLite2 City database to map IPs to locations (country, city, latitude, longitude, timezone). Generates ASCII art visualization for the traced path like:
```
1. 192.168.1.1 (Home Router) [1ms]
2. 10.0.0.1 (ISP Gateway) [5ms]
3. 172.16.1.2 (AS1234 - Telkom, Jakarta) [12ms]
4. 203.0.113.1 (AS5678 - Cloudflare, Singapore) [45ms]
5. 104.16.0.1 (Target Server - Cloudflare) [50ms]
```

---

#### **5. `ssl_cert_analyzer.rs` (Rust)**

**Task:** Analyzes the SSL/TLS certificate used by the website.

**Purpose:** This file uses `rustls` with `ClientConfig::builder()` to create a TLS connection to the target host and port. Handshake is performed with `ServerName::try_from(host)`, then after successful handshake, extracts the certificate chain from `server_cert_verifier`. Uses `webpki` to parse the certificate and extract information: `subject` (Common Name/CN), `issuer` (Certificate Authority that issued it), `validity` (not_before and not_after dates), `subject_alt_names` (SAN for all protected domains), `key_usage` (digitalSignature, keyEncipherment), `extended_key_usage` (serverAuth, clientAuth), `signature_algorithm` (RSA-SHA256, ECDSA-SHA256), `public_key` (type RSA or ECDSA and key size). Implements `check_weak_ciphers()` with a list of known weak/obsolete ciphers (RC4, 3DES, IDEA, export ciphers). Implements `check_protocols()` to check if SSLv2, SSLv3, TLSv1.0, TLSv1.1 are still enabled (should be disabled as they are deprecated). Generates a `CertRating` with score based on key length (>=2048 bits for RSA, >=256 bits for ECDSA), cipher strength (AES-GCM > AES-CBC > RC4), protocol version (TLSv1.3 > TLSv1.2 > TLSv1.1), and certificate validity (validity period not too long).

---

Indonesian:

---

### **📂 `modules/network/` - Pemindaian Jaringan**

#### **1. `dns_enum.rs` (Rust)**

**Tugas:** Melakukan enumerasi DNS lengkap untuk mengumpulkan semua informasi terkait domain target.

**Tujuan:** File ini bertanggung jawab untuk mengumpulkan semua tipe record DNS yang tersedia dari domain target menggunakan `trust-dns-resolver`. Record yang dikumpulkan meliputi: A (IPv4 address), AAAA (IPv6 address), CNAME (canonical name/alias), MX (mail exchange servers dengan priority), TXT (text records termasuk SPF untuk email authentication, DKIM untuk email signing, DMARC untuk email policy), NS (nameservers yang mengelola domain), SOA (start of authority dengan primary nameserver, email admin, serial number, refresh/retry/expire intervals), dan SRV (service records untuk protocol seperti SIP, LDAP). Mencoba melakukan zone transfer menggunakan `AXFR` query untuk mendapatkan semua records sekaligus jika DNS server tidak dikonfigurasi dengan aman. Mencari subdomain menggunakan teknik brute force dan dictionary attack dengan wordlist 50,000+ subdomain umum (admin, dev, staging, api, app, mail, ftp, blog, shop, forum, support, test, backup) menggunakan rate limiting dengan `Semaphore::new(50)` untuk menghindari detection. Hasil subdomain valid disimpan dalam `HashSet` untuk deduplication.

---

#### **2. `port_scanner.go` (Go)**

**Tugas:** Memindai port TCP dan UDP pada target untuk menemukan service yang berjalan.

**Tujuan:** File ini mengimplementasikan berbagai metode scanning port untuk mendeteksi service yang berjalan. `ScanTCP()` menggunakan `net.DialTimeout` dengan worker pool pattern untuk concurrency (create N workers = threads, each worker reads from portsChan, melakukan dial, hasil dikirim ke resultsChan). `ScanSYN()` menggunakan raw socket dengan `golang.org/x/net/ipv4` untuk crafting SYN packets (stealth scan) dan mendengarkan SYN-ACK responses dengan `conn.ReadFrom`. `ScanUDP()` untuk mendeteksi UDP services yang terbuka. `FIN/NULL/XMAS` scan untuk melewati firewall dengan mengirim flag yang tidak standar. Setelah port ditemukan, `ServiceDetection()` melakukan koneksi ke port, mengirim probe (HTTP, SMTP, FTP, SSH, MySQL, PostgreSQL, MongoDB, Redis, etc), dan menganalisis banner response dengan regex patterns untuk mendeteksi service name dan version (contoh: "220" untuk FTP, "SSH-2.0" untuk SSH, "220 mail" untuk SMTP).

---

#### **3. `whois_lookup.py` (Python)**

**Tugas:** Mengambil dan mem-parsing data WHOIS dari domain target.

**Tujuan:** File ini menggunakan `whois` library dengan custom server selection berdasarkan TLD (.com, .org, .net, .id, .uk, etc) untuk mendapatkan informasi kepemilikan domain. Implementasi `parse_whois()` dengan regex pattern untuk mengekstrak fields: `Registrar` (nama perusahaan pendaftar), `Creation Date` (tanggal domain pertama kali didaftarkan), `Expiration Date` (tanggal kadaluarsa domain), `Updated Date` (tanggal pembaruan terakhir), `Name Servers` (daftar nameserver yang digunakan), `Registrant Email` (email pemilik domain), `Registrant Organization` (nama organisasi pemilik), `Registrant Country` (negara pemilik), `Registrant Phone` (nomor telepon pemilik). Untuk `Registrar` validation menggunakan cache dari IANA list untuk memastikan registrar terdaftar secara resmi. Implementasi `check_availability(domain: str)` untuk mengecek apakah domain tersedia (belum terdaftar). Menyimpan hasil dalam `WhoisData` dataclass dengan `__post_init__` untuk convert string dates ke `datetime` objects.

---

#### **4. `traceroute_analyzer.rs` (Rust)**

**Tugas:** Melacak dan menganalisis jalur jaringan menuju target server.

**Tujuan:** File ini mengirimkan paket UDP ke port high (33434) dengan incrementing TTL (Time-To-Live) menggunakan `TOS` field set untuk `IP_TOS` (0x10). Setiap hop mengembalikan ICMP Time Exceeded response yang ditangkap menggunakan `socket2` untuk raw socket access. Untuk setiap hop, dicatat IP address, RTT (round trip time) dalam ms (dengan 3 probes per hop untuk akurasi), dan hostname (reverse DNS lookup menggunakan `trust-dns-resolver`). Implementasi `geolocate_hops()` menggunakan `maxminddb` library dengan GeoLite2 City database untuk mapping IP ke lokasi (country, city, latitude, longitude, timezone). Menghasilkan visualisasi ASCII art untuk path yang di-trace seperti: 
```
1. 192.168.1.1 (Home Router) [1ms]
2. 10.0.0.1 (ISP Gateway) [5ms]
3. 172.16.1.2 (AS1234 - Telkom, Jakarta) [12ms]
4. 203.0.113.1 (AS5678 - Cloudflare, Singapore) [45ms]
5. 104.16.0.1 (Target Server - Cloudflare) [50ms]
```

---

#### **5. `ssl_cert_analyzer.rs` (Rust)**

**Tugas:** Menganalisis sertifikat SSL/TLS yang digunakan oleh website.

**Tujuan:** File ini menggunakan `rustls` dengan `ClientConfig::builder()` untuk membuat TLS connection ke target host dan port. Handshake dilakukan dengan `ServerName::try_from(host)`, kemudian setelah handshake berhasil, ekstrak certificate chain dari `server_cert_verifier`. Gunakan `webpki` untuk parse certificate dan ekstrak informasi: `subject` (Common Name/CN), `issuer` (Certificate Authority yang menerbitkan), `validity` (not_before dan not_after dates), `subject_alt_names` (SAN untuk semua domain yang dilindungi), `key_usage` (digitalSignature, keyEncipherment), `extended_key_usage` (serverAuth, clientAuth), `signature_algorithm` (RSA-SHA256, ECDSA-SHA256), `public_key` (type RSA atau ECDSA dan key size). Implementasi `check_weak_ciphers()` dengan list known weak/obsolete ciphers (RC4, 3DES, IDEA, export ciphers). Implementasi `check_protocols()` untuk memeriksa apakah SSLv2, SSLv3, TLSv1.0, TLSv1.1 masih diaktifkan (harusnya disabled karena sudah deprecated). Menghasilkan `CertRating` dengan score berdasarkan panjang key (>=2048 bit untuk RSA, >=256 bit untuk ECDSA), cipher strength (AES-GCM > AES-CBC > RC4), protocol version (TLSv1.3 > TLSv1.2 > TLSv1.1), dan certificate validity (masa berlaku tidak terlalu panjang).

---

Explanation of the modules folder content section:

---

## 📂 **`modules/content/` - Content Analysis**

#### **1. `html_parser.rs` (Rust)**

**Task:** Parse HTML/DOM structure from the website page in depth.

**Purpose:** This file uses `html5ever::parse_document()` with `RcDom` sink to convert HTML into a DOM tree representation. After parsing, it performs tree traversal with depth-first search to extract all important elements: `title` tags (page title), `h1` through `h6` (headings with hierarchy), all `form` tags with `method` attributes (GET/POST), `action` (target URL), and all `input` fields (name, type, value, placeholder). Extracts all `a` tags with `href` (target URL) and `rel` attributes (nofollow, noopener). Extracts all `script` tags with `src` (external files) and inline content for further analysis. Extracts all `img` tags with `src` (image URL), `alt` (alternative text), `title` (tooltip). Implements `get_text_content()` to extract all visible text for NLP processing. Stores results in `ParsedHtml` struct with all extracted elements in `Vec<Element>` for easy access and analysis by other components.

---

#### **2. `js_analyzer.rs` (Rust)**

**Task:** Analyze JavaScript files to detect vulnerabilities and sensitive information.

**Purpose:** This file uses `oxc` parser or `swc` for parsing JavaScript into Abstract Syntax Tree (AST). Implements AST traversal with visitor pattern to find: `eval()` function calls that can execute arbitrary code (dangerous), `document.write()` that can cause XSS, `innerHTML` assignment that is potentially dangerous, `setTimeout` with string argument, `Function()` constructor, `postMessage` calls. Extracts all string literals that may contain API keys using regex patterns: `/([A-Z0-9]{32,40})/` for AWS keys, `/sk_live_[A-Za-z0-9]{24,32}/` for Stripe keys, `/AIza[0-9A-Za-z\\-_]{35}/` for Google API keys, `/ghp_[A-Za-z0-9]{36}/` for GitHub tokens. Implements `detect_framework()` with pattern matching for jQuery (`$` or `jQuery`), React (`React.createElement` or JSX), Vue (`Vue` or `v-` directives), Angular (`ng-` directives or `Angular`), and other frameworks. Generates `JsReport` with fields: `framework_detected` (list of detected frameworks), `api_keys_found` (exposed API keys), `dangerous_functions` (dangerous functions with line and column), `library_versions` (versions of libraries used), `obfuscation_detected` (whether code is obfuscated).

---

#### **3. `css_extractor.rs` (Rust)**

**Task:** Extract and analyze Cascading Style Sheets (CSS) used by the website.

**Purpose:** This file uses `cssparser` to parse CSS into AST (Abstract Syntax Tree). Extracts all `@import` (external CSS files being imported), `@font-face` (custom fonts used), `@media` (media queries for responsive design), `@keyframes` (animations). Extracts all selectors and declarations (properties and CSS values). Identifies CSS frameworks with pattern matching: Bootstrap (`.container`, `.row`, `.col-*`, `.btn`, `.navbar`), Tailwind (`tw-*` or `@tailwind` directives), Foundation (`row`, `small-*`, `large-*`, `.button`, `.top-bar`), Bulma (`.columns`, `.column`, `.button`, `.navbar`). Implements `detect_unused_css()` by comparing selectors with class names present in the scraped HTML to identify unused CSS that can be removed. Implements `find_css_variables()` to extract all custom properties (--*) for theme analysis and brand consistency. Generates `CssReport` with total rules, total selectors, detected frameworks, unused CSS percentage, and list of CSS variables.

---

#### **4. `meta_extractor.py` (Python)**

**Task:** Extract all metadata from the website for SEO and social media analysis.

**Purpose:** This file uses `BeautifulSoup` for HTML parsing and extracts all `meta` tags with `name` or `property` attributes. For `Open Graph` tags (og:title, og:description, og:image, og:url, og:type, og:site_name) used for social media sharing on Facebook, LinkedIn, and other platforms, extract from `property` attribute. For `Twitter Cards` (twitter:card, twitter:site, twitter:creator, twitter:description, twitter:image) used for Twitter preview, extract from `name` attribute. Implements `find_canonical_url()` to find the canonical link tag indicating the primary URL to avoid duplicate content in SEO. Implements `check_robots_txt()` with request to `/robots.txt` (follow redirects), parse with regex to find disallow rules that block crawlers. Implements `find_sitemap()` in robots.txt or `sitemap.xml` to find the XML sitemap listing all pages. Generates `MetaReport` with dictionary of all found meta tags, canonical URL, robots.txt status, sitemap URL, and language detection.

---

#### **5. `link_graph.rs` (Rust)**

**Task:** Build and analyze the relationship graph of links within the website.

**Purpose:** This file uses `BFS` (Breadth-First Search) traversal to explore the website from start_url up to depth_limit (default 3). For each visited page, extracts all `href` links using `html_parser`. Normalizes URLs with `Url::parse()` relative to base to obtain absolute URLs. Classifies links as `internal` (same domain) or `external` (different domain). For each link, adds an edge in the graph: `graph.add_edge(current_url, target_url)`. Implements `find_broken_links()` with concurrent HEAD requests using `reqwest` `Client::head()`, timeout 5s, and records status codes (404, 410 for broken). Implements `detect_redirect_chains()` by following redirects up to max 10, records the chain to detect unnecessary redirects (e.g., /a -> /b -> /c -> /final). Implements `analyze_anchor_text()` for each link, extracts anchor text, and performs frequency analysis to detect SEO patterns (over-optimization, keyword stuffing). Generates `LinkReport` with total internal links, external links, broken links, redirect chains, and anchor text analysis.

---

Indonesian:

---

## 📂 **`modules/content/` - Analisis Konten**

#### **1. `html_parser.rs` (Rust)**

**Tugas:** Mem-parsing struktur HTML/DOM dari halaman website secara mendalam.

**Tujuan:** File ini menggunakan `html5ever::parse_document()` dengan `RcDom` sink untuk mengubah HTML menjadi DOM tree representation. Setelah parsing, melakukan tree traversal dengan depth-first search untuk mengekstrak semua elemen penting: `title` tags (judul halaman), `h1` sampai `h6` (heading dengan hierarchy), semua `form` tags dengan atribut `method` (GET/POST), `action` (URL tujuan), dan semua `input` fields (name, type, value, placeholder). Mengekstrak semua `a` tags dengan `href` (URL tujuan) dan `rel` attributes (nofollow, noopener). Mengekstrak semua `script` tags dengan `src` (file eksternal) dan inline content untuk dianalisis lebih lanjut. Mengekstrak semua `img` tags dengan `src` (URL gambar), `alt` (teks alternatif), `title` (tooltip). Implementasi `get_text_content()` untuk mengekstrak semua teks visible untuk NLP processing. Menyimpan hasil dalam `ParsedHtml` struct dengan semua extracted elements dalam `Vec<Element>` untuk memudahkan akses dan analisis oleh komponen lain.

---

#### **2. `js_analyzer.rs` (Rust)**

**Tugas:** Menganalisis file JavaScript untuk mendeteksi kerentanan dan informasi sensitif.

**Tujuan:** File ini menggunakan `oxc` parser atau `swc` untuk parsing JavaScript ke Abstract Syntax Tree (AST). Implementasi AST traversal dengan visitor pattern untuk menemukan: function calls ke `eval()` yang dapat mengeksekusi kode arbitrer (dangerous), `document.write()` yang dapat menyebabkan XSS, `innerHTML` assignment yang berpotensi berbahaya, `setTimeout` dengan string argument, `Function()` constructor, `postMessage` calls. Ekstrak semua string literals yang mungkin mengandung API keys menggunakan regex pattern: `/([A-Z0-9]{32,40})/` untuk AWS keys, `/sk_live_[A-Za-z0-9]{24,32}/` untuk Stripe keys, `/AIza[0-9A-Za-z\\-_]{35}/` untuk Google API keys, `/ghp_[A-Za-z0-9]{36}/` untuk GitHub tokens. Implementasi `detect_framework()` dengan pattern matching untuk jQuery (`$` atau `jQuery`), React (`React.createElement` atau JSX), Vue (`Vue` atau `v-` directives), Angular (`ng-` directives atau `Angular`), dan framework lainnya. Menghasilkan `JsReport` dengan fields: `framework_detected` (list framework yang ditemukan), `api_keys_found` (API keys yang terekspos), `dangerous_functions` (fungsi berbahaya dengan line dan column), `library_versions` (versi library yang digunakan), `obfuscation_detected` (apakah kode di-obfuscate).

---

#### **3. `css_extractor.rs` (Rust)**

**Tugas:** Mengekstrak dan menganalisis Cascading Style Sheets (CSS) yang digunakan website.

**Tujuan:** File ini menggunakan `cssparser` untuk parse CSS ke AST (Abstract Syntax Tree). Mengekstrak semua `@import` (file CSS eksternal yang diimpor), `@font-face` (custom fonts yang digunakan), `@media` (media queries untuk responsive design), `@keyframes` (animasi). Mengekstrak semua selectors dan declarations (properti dan nilai CSS). Identifikasi CSS frameworks dengan pattern matching: Bootstrap (`.container`, `.row`, `.col-*`, `.btn`, `.navbar`), Tailwind (`tw-*` atau `@tailwind` directives), Foundation (`row`, `small-*`, `large-*`, `.button`, `.top-bar`), Bulma (`.columns`, `.column`, `.button`, `.navbar`). Implementasi `detect_unused_css()` dengan membandingkan selectors dengan class names yang ada di HTML yang di-scrape untuk mengidentifikasi CSS yang tidak terpakai dan dapat dihapus. Implementasi `find_css_variables()` untuk mengekstrak semua custom properties (--*) untuk theme analysis dan brand consistency. Generate `CssReport` dengan total rules, total selectors, frameworks detected, unused CSS percentage, dan list CSS variables.

---

#### **4. `meta_extractor.py` (Python)**

**Tugas:** Mengekstrak semua metadata dari website untuk analisis SEO dan social media.

**Tujuan:** File ini menggunakan `BeautifulSoup` untuk parsing HTML dan mengekstrak semua `meta` tags dengan attribute `name` atau `property`. Untuk `Open Graph` tags (og:title, og:description, og:image, og:url, og:type, og:site_name) yang digunakan untuk social media sharing di Facebook, LinkedIn, dan platform lainnya, ekstrak dari `property` attribute. Untuk `Twitter Cards` (twitter:card, twitter:site, twitter:creator, twitter:description, twitter:image) yang digunakan untuk Twitter preview, ekstrak dari `name` attribute. Implementasi `find_canonical_url()` untuk menemukan canonical link tag yang menunjukkan URL utama untuk menghindari duplicate content di SEO. Implementasi `check_robots_txt()` dengan request ke `/robots.txt` (ikuti redirects), parse dengan regex untuk menemukan disallow rules yang memblokir crawler. Implementasi `find_sitemap()` di robots.txt atau `sitemap.xml` untuk menemukan XML sitemap yang mendaftar semua halaman. Generate `MetaReport` dengan dictionary dari semua meta tags yang ditemukan, canonical URL, robots.txt status, sitemap URL, dan language detection.

---

#### **5. `link_graph.rs` (Rust)**

**Tugas:** Membangun dan menganalisis graph hubungan antar link di website.

**Tujuan:** File ini menggunakan `BFS` (Breadth-First Search) traversal untuk menjelajahi website dari start_url hingga depth_limit (default 3). Untuk setiap halaman yang dikunjungi, ekstrak semua `href` links menggunakan `html_parser`. Normalisasi URL dengan `Url::parse()` relative to base untuk mendapatkan URL absolut. Klasifikasikan link sebagai `internal` (same domain) atau `external` (different domain). Untuk setiap link, tambahkan edge di graph: `graph.add_edge(current_url, target_url)`. Implementasi `find_broken_links()` dengan concurrent HEAD requests menggunakan `reqwest` `Client::head()`, timeout 5s, dan record status codes (404, 410 untuk broken). Implementasi `detect_redirect_chains()` dengan follow redirects sampai max 10, catat chain untuk mendeteksi redirect yang tidak perlu (misal: /a -> /b -> /c -> /final). Implementasi `analyze_anchor_text()` untuk setiap link, ekstrak anchor text, dan lakukan frequency analysis untuk mendeteksi SEO patterns (over-optimization, keyword stuffing). Generate `LinkReport` dengan total internal links, external links, broken links, redirect chains, dan anchor text analysis.

---

Explanation of the modules folder security section:

---

## 📂 **`modules/security/` - Security Analysis**

#### **1. `header_analyzer.rs` (Rust)**

**Task:** Analyze HTTP response headers to evaluate website security configuration.

**Purpose:** This file examines all HTTP response headers and provides a security score based on presence and correctness of configuration. For `HSTS` (HTTP Strict Transport Security): checks header, parses `max-age` (HTTPS enforcement duration), `includeSubDomains` (whether applies to subdomains), `preload` (whether ready for HSTS preload list). Scores: max-age >= 31536000 (1 year) score 10, >= 2592000 (30 days) score 8, < 30 days score 5, absent score 0. For `CSP` (Content Security Policy): parses header, checks `default-src` (fallback for all resources), `script-src` (allowed script sources), `style-src` (allowed style sources), detects presence of `unsafe-inline` (dangerous for XSS), `unsafe-eval` (dangerous for code injection). For `X-Frame-Options`: checks whether `DENY` (prevents all framing), `SAMEORIGIN` (only from same domain), or `ALLOW-FROM` (only from specific URL). For `X-XSS-Protection`: checks `1; mode=block` (active and blocking) vs `0` (disabled). Generates `HeaderSecurityScore` with percentage of total possible points.

---

#### **2. `cookie_scanner.rs` (Rust)**

**Task:** Scan and analyze cookies set by the website to find insecure configurations.

**Purpose:** This file extracts all `Set-Cookie` headers from responses and parses each cookie using the `cookie` crate. For each cookie, checks: `secure` flag (whether cookie is only sent over HTTPS - if not, vulnerable to session hijacking), `httpOnly` flag (whether cookie is inaccessible to JavaScript - if not, vulnerable to XSS), `same_site` attribute (Lax for moderate CSRF protection, Strict for maximum protection, None which is dangerous if not accompanied by Secure flag), `domain` attribute (whether too broad like `.example.com` that applies to all subdomains), `path` attribute (whether too permissive like `/`), `expires` or `max-age` (whether session cookie without expiration that keeps session alive forever). Implements `detect_session_fixation()`: checks whether session cookie lacks `secure` and `httpOnly` flags making it vulnerable to session fixation attacks. Generates `CookieReport` with list of cookies, each with flag statuses, and overall security score.

---

#### **3. `cve_checker.py` (Python)**

**Task:** Match detected software and library versions against CVE database to find known vulnerabilities.

**Purpose:** This file loads CVE database from `cve_data.json` periodically downloaded from NVD API. Database contains: `cve_id` (vulnerability identifier), `description` (vulnerability description), `cvss_v3_score` (severity score 0-10), `cvss_v3_vector` (CVSS vector string), `affected_software` (list of software with name and version_ranges). Implements `check_software(software: str, version: str) -> List[CveMatch]` that iterates CVE database and searches for matching affected_software. For version matching, implements version parsing with `packaging.version` for semantic versioning (e.g., version 2.4.3 vulnerable if version <= 2.4.3). Implements `check_cpe(cpe: str)` for CPE identifiers matching (format: `cpe:2.3:a:apache:http_server:2.4.49:*:*:*:*:*:*:*`). Generates `CveReport` with list of found vulnerabilities, each with severity (critical if CVSS >= 9.0, high if >= 7.0, medium if >= 4.0, low if < 4.0), CVSS score, description, and links to NVD.

---

#### **4. `xss_detector.rs` (Rust)**

**Task:** Detect potential Cross-Site Scripting (XSS) vulnerabilities on the website.

**Purpose:** This file tests all input fields and URL parameters for various types of XSS. `detect_reflected_xss()` injects payloads into URL parameters (e.g., `?q=<script>alert(1)</script>`), makes request to the page, and searches for payload in response to see if it is not properly encoded. Payload list from `xss-payload-list` includes `<script>alert(1)</script>`, `<img src=x onerror=alert(1)>`, `"><script>alert(1)</script>`, `javascript:alert(1)`, `<svg/onload=alert(1)>`. `detect_dom_xss()` uses AST analysis from JavaScript to detect `document.write()` with user input, `innerHTML` assignment with unsafe data, `eval()` with user-controlled data, `setTimeout()` with string. `context_aware_detection()` for HTML context (whether payload goes into HTML tag, attribute value, or JavaScript string) to determine whether context-appropriate escaping is performed. Generates `XssReport` with list of potential XSS points, each with context, payload used, and confidence level (high if payload found in response without encoding).

---

#### **5. `sql_injection_detector.rs` (Rust)**

**Task:** Detect potential SQL Injection vulnerabilities on the website.

**Purpose:** This file tests all URL parameters, form fields, and API endpoints with various SQL injection techniques. `detect_error_based()` injects payloads like `' OR '1'='1`, `' UNION SELECT NULL--`, `' AND SLEEP(5)--`, `' OR 1=1--` and extracts database error messages with `error_patterns` matching `MySQL` (You have an error in your SQL syntax), `PostgreSQL` (ERROR: syntax error at or near), `MS SQL` (Unclosed quotation mark), `Oracle` (ORA-01756: quoted string not properly terminated). `detect_blind_boolean()` compares response between `AND 1=1` and `AND 1=2`; if different, then possible blind boolean injection (because true vs false condition yields different responses). `detect_blind_time()` uses `AND SLEEP(5)` and measures response time; if response time > 3 seconds, then possible time-based blind injection. Generates `SqlReport` with list of potential SQLi points, type of vulnerability (error-based, boolean-based, time-based, union-based), database type detected, and confidence level.

---

#### **6. `csrf_analyzer.rs` (Rust)**

**Task:** Analyze Cross-Site Request Forgery (CSRF) protection implementation on the website.

**Purpose:** This file examines every form on the website to see if CSRF protection exists. For each form, checks whether there is a hidden input with `name` containing: `csrf_token`, `_token`, `csrfmiddlewaretoken`, `authenticity_token`, `csrf-token`, `xsrf-token`. Checks whether form uses `POST` method (CSRF only relevant for state-changing operations). Checks whether cookie has `SameSite` attribute: if `None` then vulnerable, if `Lax` then partially protected, if `Strict` then protected (because browser will not send cookies for cross-site requests). Implements `check_referer_header()`: checks whether `Referrer-Policy` header is strict enough (`no-referrer`, `same-origin`, `strict-origin-when-cross-origin`) to prevent CSRF via referer validation. Implements `token_mutation_analysis()`: extracts CSRF token, modifies it slightly (change one character), submits form, checks whether request is accepted (sign that token is not properly validated). Generates `CsrfReport` with list of vulnerable forms, each with csrf_token presence, same_site status, and overall vulnerability level.

---

Indonesian:

---
