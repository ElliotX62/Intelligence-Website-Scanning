**Here I will explain the tasks and purposes of each file and folder in the directory architecture.
let's get started**

**explanation of each file in the root directory:**

---

**Alright, I will explain sequentially each folder and its contents. Let's start with the files in the project root first.**

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

**Indonesian:**

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

**Explanation of each file in the core folder section of the directory architecture:Explanation of each file in the core folder section of the directory architecture:**

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

**indonesian:**

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

**Explanation of the network section modules folder:**

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

**indonesian:**

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

**Explanation of the modules folder content section:**

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

**Indonesian:**

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

**Explanation of the modules folder security section:**

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

**Indonesian:**

---

## 📂 **`modules/security/` - Analisis Keamanan**

#### **1. `header_analyzer.rs` (Rust)**

**Tugas:** Menganalisis HTTP response headers untuk mengevaluasi konfigurasi keamanan website.

**Tujuan:** File ini memeriksa semua HTTP response headers dan memberikan skor keamanan berdasarkan keberadaan dan ketepatan konfigurasi. Untuk `HSTS` (HTTP Strict Transport Security): memeriksa header, mem-parsing `max-age` (durasi enforce HTTPS), `includeSubDomains` (apakah berlaku untuk subdomain), `preload` (apakah siap untuk HSTS preload list). Memberi skor: max-age >= 31536000 (1 tahun) score 10, >= 2592000 (30 hari) score 8, < 30 hari score 5, tidak ada score 0. Untuk `CSP` (Content Security Policy): mem-parsing header, memeriksa `default-src` (fallback untuk semua resource), `script-src` (sumber script yang diizinkan), `style-src` (sumber style yang diizinkan), mendeteksi adanya `unsafe-inline` (berbahaya untuk XSS), `unsafe-eval` (berbahaya untuk code injection). Untuk `X-Frame-Options`: memeriksa apakah `DENY` (mencegah semua framing), `SAMEORIGIN` (hanya dari domain yang sama), atau `ALLOW-FROM` (hanya dari URL tertentu). Untuk `X-XSS-Protection`: memeriksa `1; mode=block` (aktif dan blocking) vs `0` (nonaktif). Generate `HeaderSecurityScore` dengan percentage dari total possible points.

---

#### **2. `cookie_scanner.rs` (Rust)**

**Tugas:** Memindai dan menganalisis cookie yang diset oleh website untuk menemukan konfigurasi yang tidak aman.

**Tujuan:** File ini mengekstrak semua `Set-Cookie` headers dari response dan mem-parsing setiap cookie menggunakan `cookie` crate. Untuk setiap cookie, periksa: `secure` flag (apakah cookie hanya dikirim melalui HTTPS - jika tidak, rentan terhadap session hijacking), `httpOnly` flag (apakah cookie tidak dapat diakses oleh JavaScript - jika tidak, rentan terhadap XSS), `same_site` attribute (Lax untuk perlindungan CSRF moderate, Strict untuk perlindungan maksimum, None yang berbahaya jika tidak disertai Secure flag), `domain` attribute (apakah terlalu broad seperti `.example.com` yang berlaku untuk semua subdomain), `path` attribute (apakah terlalu permissive seperti `/`), `expires` atau `max-age` (apakah session cookie tanpa expiration yang membuat session tetap hidup selamanya). Implementasi `detect_session_fixation()`: memeriksa apakah session cookie tidak memiliki `secure` dan `httpOnly` flags yang membuatnya rentan terhadap session fixation attacks. Generate `CookieReport` dengan list cookies, masing-masing dengan flags status, dan overall security score.

---

#### **3. `cve_checker.py` (Python)**

**Tugas:** Mencocokkan versi software dan library yang terdeteksi dengan database CVE untuk menemukan kerentanan yang diketahui.

**Tujuan:** File ini memuat database CVE dari `cve_data.json` yang di-download dari NVD API secara periodik. Database berisi: `cve_id` (identifikasi kerentanan), `description` (deskripsi kerentanan), `cvss_v3_score` (severity score 0-10), `cvss_v3_vector` (CVSS vector string), `affected_software` (daftar software dengan name dan version_ranges). Implementasi `check_software(software: str, version: str) -> List[CveMatch]` yang mengiterasi CVE database dan mencari yang affected_software matching. Untuk version matching, implementasi version parsing dengan `packaging.version` untuk semantic versioning (contoh: version 2.4.3 vulnerable if version <= 2.4.3). Implementasi `check_cpe(cpe: str)` untuk CPE identifiers matching (format: `cpe:2.3:a:apache:http_server:2.4.49:*:*:*:*:*:*:*`). Generate `CveReport` dengan list vulnerabilities yang ditemukan, masing-masing dengan severity (critical jika CVSS >= 9.0, high jika >= 7.0, medium jika >= 4.0, low jika < 4.0), CVSS score, description, dan links ke NVD.

---

#### **4. `xss_detector.rs` (Rust)**

**Tugas:** Mendeteksi potensi Cross-Site Scripting (XSS) vulnerabilities pada website.

**Tujuan:** File ini menguji semua input fields dan parameter URL untuk berbagai jenis XSS. `detect_reflected_xss()` meng-inject payloads ke parameter URL (contoh: `?q=<script>alert(1)</script>`), melakukan request ke page, dan mencari payload dalam response untuk melihat apakah tidak di-encode dengan benar. Payload list dari `xss-payload-list` mencakup `<script>alert(1)</script>`, `<img src=x onerror=alert(1)>`, `"><script>alert(1)</script>`, `javascript:alert(1)`, `<svg/onload=alert(1)>`. `detect_dom_xss()` menggunakan AST analysis dari JavaScript untuk mendeteksi `document.write()` dengan input user, `innerHTML` assignment dengan data yang tidak aman, `eval()` dengan user-controlled data, `setTimeout()` dengan string. `context_aware_detection()` untuk HTML context (apakah payload masuk ke dalam tag HTML, attribute value, atau JavaScript string) untuk menentukan apakah context-appropriate escaping dilakukan. Generate `XssReport` dengan list potential XSS points, each with context, payload yang digunakan, dan confidence level (high jika payload ditemukan dalam response tanpa encoding).

---

#### **5. `sql_injection_detector.rs` (Rust)**

**Tugas:** Mendeteksi potensi SQL Injection vulnerabilities pada website.

**Tujuan:** File ini menguji semua parameter URL, form fields, dan API endpoints dengan berbagai teknik SQL injection. `detect_error_based()` meng-inject payloads seperti `' OR '1'='1`, `' UNION SELECT NULL--`, `' AND SLEEP(5)--`, `' OR 1=1--` dan mengekstrak database error messages dengan `error_patterns` yang match `MySQL` (You have an error in your SQL syntax), `PostgreSQL` (ERROR: syntax error at or near), `MS SQL` (Unclosed quotation mark), `Oracle` (ORA-01756: quoted string not properly terminated). `detect_blind_boolean()` membandingkan response antara `AND 1=1` dan `AND 1=2`; jika berbeda, maka possible blind boolean injection (karena kondisi true vs false menghasilkan response yang berbeda). `detect_blind_time()` menggunakan `AND SLEEP(5)` dan mengukur response time; jika response time > 3 seconds, maka possible time-based blind injection. Generate `SqlReport` dengan list potential SQLi points, type of vulnerability (error-based, boolean-based, time-based, union-based), database type detected, dan confidence level.

---

#### **6. `csrf_analyzer.rs` (Rust)**

**Tugas:** Menganalisis implementasi Cross-Site Request Forgery (CSRF) protection pada website.

**Tujuan:** File ini memeriksa setiap form di website untuk melihat apakah ada perlindungan CSRF. Untuk setiap form, periksa apakah ada hidden input dengan `name` mengandung: `csrf_token`, `_token`, `csrfmiddlewaretoken`, `authenticity_token`, `csrf-token`, `xsrf-token`. Periksa apakah form menggunakan method `POST` (CSRF hanya relevan untuk state-changing operations). Periksa apakah cookie memiliki `SameSite` attribute: jika `None` maka vulnerable, jika `Lax` maka partially protected, jika `Strict` maka protected (karena browser tidak akan mengirim cookie untuk cross-site requests). Implementasi `check_referer_header()`: memeriksa apakah `Referrer-Policy` header cukup ketat (`no-referrer`, `same-origin`, `strict-origin-when-cross-origin`) untuk mencegah CSRF via referer validation. Implementasi `token_mutation_analysis()`: mengekstrak CSRF token, memodifikasi sedikit (ubah satu karakter), submit form, periksa apakah request diterima (tanda token tidak divalidasi dengan benar). Generate `CsrfReport` dengan list forms yang vulnerable, each with csrf_token presence, same_site status, dan overall vulnerability level.

---

**Explanation of the modules folder infrastructure section:**

---

## 📂 **`modules/infrastructure/` - Infrastructure & Hosting**

#### **1. `server_fingerprint.rs` (Rust)**

**Task:** Detect the type of web server used by the website and its version.

**Purpose:** This file analyzes response headers and server behavior to identify the type and version of the web server. For `Server` header, parse the value to detect server type (Apache, Nginx, Microsoft IIS, Lighttpd, Caddy, Tomcat, Jetty) and version. If header is absent or removed (common security practice), use behavioral analysis: request non-existent path (`/nonexistent-xyz`), analyze `404` error page format (Apache has different format compared to Nginx, IIS has typical Windows error page). Use `Wappalyzer` patterns to detect server and framework based on response patterns. Implements `detect_os()` from TTL value (Time-To-Live): Windows typically TTL 128, Linux TTL 64, macOS TTL 64) and TCP/IP fingerprinting using p0f patterns for more accurate OS identification. Generates `ServerInfo` struct with fields: `server_type` (Apache, Nginx, IIS, etc), `version` (specific version), `os` (operating system), `powered_by` (X-Powered-By header), `framework` (backend framework like Laravel, Django, Rails), `platform` (platform like .NET, PHP, Node.js, Python).

---

#### **2. `cloud_detector.go` (Go)**

**Task:** Detect whether the website is hosted on a cloud provider and determine the specific cloud provider.

**Purpose:** This file uses IP range databases downloaded from published providers to detect cloud provider. Uses `https://ip-ranges.amazonaws.com/ip-ranges.json` for AWS (covers all regions and services), `https://www.gstatic.com/ipranges/cloud.json` for GCP, `https://www.microsoft.com/en-us/download/details.aspx?id=56519` for Azure, and databases for DigitalOcean, Linode, Vultr, Heroku. Implements `detect_specific_services(ip: string) -> []CloudService` with DNS PTR records and HTTP response headers: `x-amz-*` for AWS services (S3, EC2, CloudFront), `x-cloud-trace-context` for GCP, `x-ms-*` for Azure. Identifies specific services such as EC2 (compute), S3 (storage), CloudFront (CDN), GCE (compute), App Engine (PaaS), Azure VMs, Azure Blob Storage. Generates `CloudReport` with provider, services used, region (us-east-1, eu-west-1, ap-southeast-1), and confidence level based on number of matching indicators.

---

#### **3. `cdn_detector.rs` (Rust)**

**Task:** Detect whether the website uses a Content Delivery Network (CDN) and identify the specific CDN provider.

**Purpose:** This file analyzes response headers to detect CDN provider. Detection based on response headers: `CF-*` headers for Cloudflare (CF-Ray, CF-Cache-Status), `Fastly-*` headers for Fastly (Fastly-Debug-Digest, X-Served-By), `X-Cache` and `X-Akamai-*` headers for Akamai, `X-Amz-Cf-*` headers for CloudFront, `X-Cache-Lookup` for Cloudflare. If headers are absent, perform reverse DNS lookup on IP and look for patterns like `cloudflare.com`, `fastly.net`, `akamai.net`, `cloudfront.net`, `stackpath.com`, `keycdn.com`. Implements `detect_edge_locations()` from `CF-Ray` header (Cloudflare provides data center code like `CDG`, `FRA`, `LHR`) or `X-Edge-Location` (Akamai). Implements `detect_caching_behavior()` from `Cache-Control` and `X-Cache` headers to see if CDN is caching properly (HIT, MISS, BYPASS). Generates `CdnReport` with provider, edge locations (data center serving the request), and proxy status (whether CDN acts as reverse proxy).

---

#### **4. `load_balancer_detector.rs` (Rust)**

**Task:** Detect the presence of a load balancer in front of the web server and determine its type.

**Purpose:** This file analyzes `Set-Cookie` headers to detect load balancer. Cookie patterns: `AWSALB` and `AWSELB` for AWS Application/Network Load Balancer, `BIGipServer` for F5 Big-IP, multiple `JSESSIONID` for same domain (indication of sticky session), multiple `PHPSESSID`. Analyzes `Server` header for `HAProxy` (load balancer software), `nginx` (if multiple backends with upstream), `F5` (hardware load balancer). Implements `analyze_ttl_differences()`: measure TTL (Time-To-Live) from multiple requests to the domain, if they differ then possible load balancing (because each request may be directed to different backend). Implements `detect_algorithm()` based on response cookie or header pattern: `round-robin` (sequential IPs), `least-connections` (random IPs), `ip-hash` (consistent IP for each client). Implements `estimate_backend_count()` by checking variations in response headers (different Server headers, different Date headers, different Session cookies). Generates `LBReport` with type (hardware/software/cloud), algorithm, and backend count estimation.

---

#### **5. `hosting_provider_lookup.rs` (Rust)**

**Task:** Look up and identify the hosting provider used by the website.

**Purpose:** This file uses `maxminddb` library with GeoLite2-ASN.mmdb database to obtain ASN (Autonomous System Number) and organization. Uses ISP database to get provider name (Telkom, Indosat, AWS, Google Cloud, etc). Uses Geo database to get country, city, latitude, longitude, timezone. Implements `resolve_hostname()` reverse DNS to get domain name (PTR record). Implements `check_rdns_pattern()` to detect provider from PTR record pattern: `ec2-*.compute.amazonaws.com` for AWS EC2, `*.cloud.google.com` for GCP, `*.cloudapp.azure.com` for Azure, `*.digitalocean.com` for DigitalOcean, `*.linode.com` for Linode. Implements `detect_data_center()` by checking whether IP belongs to known ranges for major data centers (Equinix, Digital Realty, NTT). Generates `HostingReport` with provider (hosting name), ISP, ASN (number and name), geolocation (country, city, coordinates), hostname (PTR record), and data center location (if detected).

---

**Indonesian:**

---

l## 📂 **`modules/infrastructure/` - Infrastruktur & Hosting**

#### **1. `server_fingerprint.rs` (Rust)**

**Tugas:** Mendeteksi jenis web server yang digunakan oleh website dan versinya.

**Tujuan:** File ini menganalisis response headers dan perilaku server untuk mengidentifikasi jenis dan versi web server. Untuk `Server` header, parse nilai untuk mendeteksi server type (Apache, Nginx, Microsoft IIS, Lighttpd, Caddy, Tomcat, Jetty) dan version. Jika header tidak ada atau dihapus (common security practice), gunakan behavioral analysis: request non-existent path (`/nonexistent-xyz`), analisis format `404` error page (Apache memiliki format berbeda dengan Nginx, IIS memiliki error page khas Windows). Gunakan `Wappalyzer` patterns untuk mendeteksi server dan framework berdasarkan pola response. Implementasi `detect_os()` dari TTL value (Time-To-Live): Windows biasanya TTL 128, Linux TTL 64, macOS TTL 64) dan TCP/IP fingerprinting menggunakan p0f patterns untuk identifikasi OS yang lebih akurat. Menghasilkan `ServerInfo` struct dengan fields: `server_type` (Apache, Nginx, IIS, etc), `version` (versi spesifik), `os` (operating system), `powered_by` (X-Powered-By header), `framework` (framework backend seperti Laravel, Django, Rails), `platform` (platform seperti .NET, PHP, Node.js, Python).

---

#### **2. `cloud_detector.go` (Go)**

**Tugas:** Mendeteksi apakah website di-hosting di cloud provider dan menentukan penyedia cloud spesifik.

**Tujuan:** File ini menggunakan IP range databases yang di-download dari provider yang dipublikasikan untuk mendeteksi cloud provider. Gunakan `https://ip-ranges.amazonaws.com/ip-ranges.json` untuk AWS (mencakup semua region dan service), `https://www.gstatic.com/ipranges/cloud.json` untuk GCP, `https://www.microsoft.com/en-us/download/details.aspx?id=56519` untuk Azure, dan database untuk DigitalOcean, Linode, Vultr, Heroku. Implementasi `detect_specific_services(ip: string) -> []CloudService` dengan DNS PTR records dan HTTP response headers: `x-amz-*` untuk AWS services (S3, EC2, CloudFront), `x-cloud-trace-context` untuk GCP, `x-ms-*` untuk Azure. Identifikasi layanan spesifik seperti EC2 (compute), S3 (storage), CloudFront (CDN), GCE (compute), App Engine (PaaS), Azure VMs, Azure Blob Storage. Generate `CloudReport` dengan provider, services used, region (us-east-1, eu-west-1, ap-southeast-1), dan confidence level berdasarkan jumlah indikator yang cocok.

---

#### **3. `cdn_detector.rs` (Rust)**

**Tugas:** Mendeteksi apakah website menggunakan Content Delivery Network (CDN) dan mengidentifikasi provider CDN spesifik.

**Tujuan:** File ini menganalisis response headers untuk mendeteksi CDN provider. Deteksi berdasarkan response headers: `CF-*` headers untuk Cloudflare (CF-Ray, CF-Cache-Status), `Fastly-*` headers untuk Fastly (Fastly-Debug-Digest, X-Served-By), `X-Cache` dan `X-Akamai-*` headers untuk Akamai, `X-Amz-Cf-*` headers untuk CloudFront, `X-Cache-Lookup` untuk Cloudflare. Jika header tidak ada, lakukan reverse DNS lookup pada IP dan cari pola `cloudflare.com`, `fastly.net`, `akamai.net`, `cloudfront.net`, `stackpath.com`, `keycdn.com`. Implementasi `detect_edge_locations()` dari `CF-Ray` header (Cloudflare memberikan data center code seperti `CDG`, `FRA`, `LHR`) atau `X-Edge-Location` (Akamai). Implementasi `detect_caching_behavior()` dari `Cache-Control` dan `X-Cache` headers untuk melihat apakah CDN melakukan caching dengan benar (HIT, MISS, BYPASS). Generate `CdnReport` dengan provider, edge locations (data center yang melayani request), dan proxy status (apakah CDN bertindak sebagai reverse proxy).

---

#### **4. `load_balancer_detector.rs` (Rust)**

**Tugas:** Mendeteksi adanya load balancer di depan web server dan menentukan tipenya.

**Tujuan:** File ini menganalisis `Set-Cookie` headers untuk mendeteksi load balancer. Pola cookie: `AWSALB` dan `AWSELB` untuk AWS Application/Network Load Balancer, `BIGipServer` untuk F5 Big-IP, `JSESSIONID` yang multiple untuk same domain (indikasi sticky session), `PHPSESSID` yang multiple. Analisis `Server` header untuk `HAProxy` (load balancer software), `nginx` (jika multiple backends dengan upstream), `F5` (hardware load balancer). Implementasi `analyze_ttl_differences()`: ukur TTL (Time-To-Live) dari multiple requests ke domain, jika berbeda-beda maka possible load balancing (karena setiap request mungkin diarahkan ke backend yang berbeda). Implementasi `detect_algorithm()` berdasarkan response cookie atau header pattern: `round-robin` (IP yang berurutan), `least-connections` (IP acak), `ip-hash` (IP consistent untuk setiap client). Implementasi `estimate_backend_count()` dengan memeriksa variasi dalam response header (Server header yang berbeda, Date header yang berbeda, Session cookie yang berbeda). Generate `LBReport` dengan type (hardware/software/cloud), algorithm, dan backend count estimation.

---

#### **5. `hosting_provider_lookup.rs` (Rust)**

**Tugas:** Mencari dan mengidentifikasi hosting provider yang digunakan oleh website.

**Tujuan:** File ini menggunakan `maxminddb` library dengan GeoLite2-ASN.mmdb database untuk mendapatkan ASN (Autonomous System Number) dan organization. Gunakan ISP database untuk mendapatkan provider name (Telkom, Indosat, AWS, Google Cloud, etc). Gunakan Geo database untuk mendapatkan country, city, latitude, longitude, timezone. Implementasi `resolve_hostname()` reverse DNS untuk mendapatkan domain name (PTR record). Implementasi `check_rdns_pattern()` untuk mendeteksi provider dari PTR record pattern: `ec2-*.compute.amazonaws.com` untuk AWS EC2, `*.cloud.google.com` untuk GCP, `*.cloudapp.azure.com` untuk Azure, `*.digitalocean.com` untuk DigitalOcean, `*.linode.com` untuk Linode. Implementasi `detect_data_center()` dengan memeriksa apakah IP termasuk dalam range yang diketahui untuk data center besar (Equinix, Digital Realty, NTT). Generate `HostingReport` dengan provider (nama hosting), ISP, ASN (nomor dan nama), geolocation (negara, kota, koordinat), hostname (PTR record), dan data center location (jika terdeteksi).

---

**Explanation of the modules folder intelligence section:**

---

## 📂 **`modules/intelligence/` - Threat Intelligence**

#### **1. `threat_intel_integration.rs` (Rust)**

**Task:** Integrate data from multiple threat intelligence feeds to obtain a comprehensive picture of threats associated with the target.

**Purpose:** This file connects the system with multiple threat intelligence platforms simultaneously. Queries are performed to AlienVault OTX (to obtain pulses and indicators from the community), IBM X-Force Exchange (for enterprise threat intelligence), MISP (Malware Information Sharing Platform - for sharing threat data between organizations), ThreatConnect (for threat intelligence aggregation), and Recorded Future (for predictive threat intelligence). Implements `query_all_sources()` which runs queries in parallel using `futures::stream::iter().for_each_concurrent()` with 30s timeout per source to prevent bottlenecks. Implements `merge_strategy()` to merge results from multiple sources: unique indicator lists (deduplication), merge confidence scores with weighted average (each source has weight based on reputation), and aggregate discovered threat families. Implements `score_threat()` to calculate risk score from number of indicators (IP, domain, hash, URL), detected malware families, and severity level from each source. Generates `CombinedThreatData` with fields: `indicators` (list of all threat indicators), `families` (malware families), `severity` (overall severity), `timestamp`, and `sources` (details from each source).

---

#### **2. `reputation_scanner.go` (Go)**

**Task:** Check domain and IP reputation across various security databases to assess trust level.

**Purpose:** This file queries multiple reputation databases in parallel. Queries to `VirusTotal` (https://www.virustotal.com/api/v3/domains/domain) to obtain detection from 70+ antivirus engines, `AlienVault OTX` (https://otx.alienvault.com/api/v1/indicators/domain/domain/general) for community threat intelligence, `URLScan` (https://urlscan.io/api/v1/search/?q=domain:domain) for website behavior, `Google Safe Browsing` (https://safebrowsing.googleapis.com/v4/threatMatches:find) for phishing and malware detection, and `Web of Trust (WOT)` for community trust ratings. For each source, parses response and extracts reputation score, categories (malicious, phishing, benign, suspicious, unsafe), and total vendors that performed detection. Generates `ReputationReport` with fields: `overall_score` (0-100, calculated from weighted average of all sources), `categories` (list of threat categories), `total_vendors` (total vendors that provided assessment), `detections` (number of vendors that detected as malicious), and `detailed_sources` (details from each source including URL, verdict, and detection rate).

---

#### **3. `domain_blacklist_checker.rs` (Rust)**

**Task:** Check whether the target domain is blacklisted across various sources for spam, malware, and abuse.

**Purpose:** This file performs DNS-based blacklist checking by querying multiple blacklist servers. Queries to `Spamhaus` (zen.spamhaus.org) - the largest blacklist for spam and malware, `SURBL` (multi.surbl.org) - URI reputation for malicious links, `URIBL` (multi.uribl.com) - domain reputation, `DNSBL` (list.dnswl.org, dnsbl-1.uceprotect.net) - DNS-based blacklists for spam, and `SORBS` (dnsbl.sorbs.net) - various abuse categories (spam, open relay, open proxy, etc). Implements `check_dnsbl()`: performs DNS query `domain + "." + server` -> `A` record. If response IP is 127.0.0.0/24 (returned IP in the 127.0.0.0/8 range), the domain is listed in the blacklist. Parses IP response to get category (127.0.0.2 for spam, 127.0.0.3 for malware, 127.0.0.4 for phishing, 127.0.0.5 for exploit, 127.0.0.10 for open relay, etc). Implements `check_multiple_servers()` which queries all DNSBL servers in parallel with 5s timeout per server for efficiency. Generates `BlacklistReport` with list of blacklist servers where domain is listed, category from each listing, and confidence level based on number of blacklists that agree.

---

#### **4. `email_harvester.py` (Python)**

**Task:** Collect all email addresses from the website using various methods and group them by category.

**Purpose:** This file uses various methods to collect emails from the website. `extract_from_html()` uses regex pattern `[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}` with `re.finditer` to obtain all matches from HTML content. `extract_mailto_links()` extracts from `a` tags with `href="mailto:email"` to obtain emails registered as links. `extract_from_js()` uses `js_analyzer` to search for string literals matching email pattern in JavaScript files. `extract_from_comments()` searches for emails in HTML comment blocks `<!-- ... -->` which often contain contact information. Implements `extract_from_files()` to extract emails from indexed files (PDF, DOC, TXT) using libraries such as `PyPDF2` for PDF and `python-docx` for DOCX. Validates each email with `email_validator` library to check syntax and checks disposable domains using `disposable-email-domains` list. Categorizes emails: `role` (admin, contact, support, info, sales, help, service), `personal` (firstname.lastname format or firstname format), `generic` (webmaster, postmaster, abuse, security). Implements `detect_patterns()` to find email patterns (e.g., john.doe@example.com, jdoe@example.com, j.doe@example.com). Generates `EmailReport` with list of emails, each with category, domain, source (HTML, JS, Mailto, File, Comment), and validation status (Valid, Invalid, Disposable, RoleBased).

---

**Indonesian:**

---

## 📂 **`modules/intelligence/` - Threat Intelligence**

#### **1. `threat_intel_integration.rs` (Rust)**

**Tugas:** Mengintegrasikan data dari berbagai threat intelligence feeds untuk mendapatkan gambaran lengkap tentang ancaman yang terkait dengan target.

**Tujuan:** File ini menghubungkan sistem dengan multiple threat intelligence platforms secara simultan. Query dilakukan ke AlienVault OTX (untuk mendapatkan pulses dan indicators dari komunitas), IBM X-Force Exchange (untuk threat intelligence enterprise), MISP (Malware Information Sharing Platform - untuk sharing threat data antar organisasi), ThreatConnect (untuk threat intelligence aggregation), dan Recorded Future (untuk predictive threat intelligence). Implementasi `query_all_sources()` yang menjalankan query secara paralel menggunakan `futures::stream::iter().for_each_concurrent()` dengan timeout 30s per source untuk mencegah bottleneck. Implementasi `merge_strategy()` untuk menggabungkan hasil dari multiple sources: unique indicator lists (deduplication), merge confidence scores dengan weighted average (setiap source memiliki weight berdasarkan reputasi), dan aggregate threat families yang ditemukan. Implementasi `score_threat()` untuk menghitung risk score dari jumlah indicators (IP, domain, hash, URL), malware families yang terdeteksi, dan severity level dari setiap source. Generate `CombinedThreatData` dengan fields: `indicators` (list semua indikator ancaman), `families` (malware families), `severity` (overall severity), `timestamp`, dan `sources` (detail dari setiap source).

---

#### **2. `reputation_scanner.go` (Go)**

**Tugas:** Mengecek reputasi domain dan IP di berbagai database keamanan untuk menilai tingkat kepercayaan.

**Tujuan:** File ini melakukan query ke multiple reputation databases secara paralel. Query ke `VirusTotal` (https://www.virustotal.com/api/v3/domains/domain) untuk mendapatkan deteksi dari 70+ antivirus engines, `AlienVault OTX` (https://otx.alienvault.com/api/v1/indicators/domain/domain/general) untuk threat intelligence komunitas, `URLScan` (https://urlscan.io/api/v1/search/?q=domain:domain) untuk perilaku website, `Google Safe Browsing` (https://safebrowsing.googleapis.com/v4/threatMatches:find) untuk deteksi phishing dan malware, dan `Web of Trust (WOT)` untuk rating kepercayaan dari komunitas. Untuk setiap source, parse response dan ekstrak reputation score, categories (malicious, phishing, benign, suspicious, unsafe), dan total vendors yang melakukan deteksi. Generate `ReputationReport` dengan fields: `overall_score` (0-100, dihitung dari weighted average semua source), `categories` (list kategori ancaman), `total_vendors` (total vendor yang memberikan penilaian), `detections` (jumlah vendor yang mendeteksi sebagai malicious), dan `detailed_sources` (detail dari setiap source termasuk URL, verdict, dan detection rate).

---

#### **3. `domain_blacklist_checker.rs` (Rust)**

**Tugas:** Mengecek apakah domain target masuk dalam blacklist di berbagai sumber untuk spam, malware, dan abuse.

**Tujuan:** File ini melakukan DNS-based blacklist checking dengan query ke multiple blacklist servers. Query ke `Spamhaus` (zen.spamhaus.org) - blacklist terbesar untuk spam dan malware, `SURBL` (multi.surbl.org) - URI reputation untuk link berbahaya, `URIBL` (multi.uribl.com) - domain reputation, `DNSBL` (list.dnswl.org, dnsbl-1.uceprotect.net) - DNS-based blacklist untuk spam, dan `SORBS` (dnsbl.sorbs.net) - berbagai kategori abuse (spam, open relay, open proxy, etc). Implementasi `check_dnsbl()`: lakukan DNS query `domain + "." + server` -> `A` record. Jika response IP 127.0.0.0/24 (returned IP in the 127.0.0.0/8 range), berarti domain terdaftar di blacklist. Parse IP response untuk mendapatkan category (127.0.0.2 untuk spam, 127.0.0.3 untuk malware, 127.0.0.4 untuk phishing, 127.0.0.5 untuk exploit, 127.0.0.10 untuk open relay, etc). Implementasi `check_multiple_servers()` yang melakukan query ke semua DNSBL servers secara paralel dengan timeout 5s per server untuk efisiensi. Generate `BlacklistReport` dengan list blacklist servers where domain is listed, category dari setiap listing, dan confidence level berdasarkan jumlah blacklist yang setuju.

---

#### **4. `email_harvester.py` (Python)**

**Tugas:** Mengumpulkan semua alamat email dari website dengan berbagai metode dan mengelompokkannya berdasarkan kategori.

**Tujuan:** File ini menggunakan berbagai metode untuk mengumpulkan email dari website. `extract_from_html()` menggunakan regex pattern `[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}` dengan `re.finditer` untuk mendapatkan semua matches dari konten HTML. `extract_mailto_links()` mengekstrak dari `a` tags dengan `href="mailto:email"` untuk mendapatkan email yang terdaftar sebagai link. `extract_from_js()` menggunakan `js_analyzer` untuk mencari string literals yang match email pattern dalam JavaScript files. `extract_from_comments()` mencari email dalam HTML comment blocks `<!-- ... -->` yang sering mengandung informasi kontak. Implementasi `extract_from_files()` untuk mengekstrak email dari file yang terindex (PDF, DOC, TXT) menggunakan library seperti `PyPDF2` untuk PDF dan `python-docx` untuk DOCX. Validasi setiap email dengan `email_validator` library untuk memeriksa syntax dan mengecek disposable domain menggunakan `disposable-email-domains` list. Kategorisasi emails: `role` (admin, contact, support, info, sales, help, service), `personal` (firstname.lastname format atau firstname format), `generic` (webmaster, postmaster, abuse, security). Implementasi `detect_patterns()` untuk menemukan pola email (contoh: john.doe@example.com, jdoe@example.com, j.doe@example.com). Generate `EmailReport` dengan list emails, each with category, domain, source (HTML, JS, Mailto, File, Comment), dan validation status (Valid, Invalid, Disposable, RoleBased).

---

**Agent folder explanation:**

---

## 📂 **`agents/` - INTELLIGENT AGENTS**

---

#### **1. `base_agent.rs` (Rust)**

**Task:** Provide a base class/template that serves as the foundation for all agents in the system.

**Purpose:** This file implements the `Agent` trait with methods that must be implemented by all derived agents: `fn init(&mut self) -> Result<()>` to initialize the agent, `fn run(&mut self) -> Result<()>` to execute the agent's main task, `fn pause(&mut self) -> Result<()>` to pause operations, `fn resume(&mut self) -> Result<()>` to resume paused operations, `fn shutdown(&mut self) -> Result<()>` to stop the agent with graceful cleanup, `fn get_state(&self) -> AgentState` to get the current state, and `fn set_state(&mut self, state: AgentState)` to change state. Implements `AgentManager` struct with fields: `agents: HashMap<String, Box<dyn Agent>>` to store all agents, `message_bus: Arc<MessageBus>` for inter-agent communication, `state_store: Arc<dyn StateStore>` for state persistence using `sled` database with TTL. Implements `broadcast_message(message: AgentMessage)` for inter-agent communication via message bus. Implements `heartbeat()` system: each agent sends heartbeat every 30 seconds, manager monitors and restarts if heartbeat is missing. Error handling with `anyhow` for recoverable errors and `panic` for unrecoverable, with automatic recovery via `supervisor` pattern that will restart crashed agents.

---

#### **2. `reconnaissance_agent.go` (Go)**

**Task:** Perform initial reconnaissance against the target website to collect basic data before in-depth scanning.

**Purpose:** This agent runs 4 reconnaissance stages in parallel using goroutines with `sync.WaitGroup`. Stage 1 - `gatherWhois()`: performs WHOIS lookup to obtain domain ownership information. Stage 2 - `gatherDNS()`: performs DNS enumeration to collect all DNS records (A, AAAA, CNAME, MX, TXT, NS, SOA). Stage 3 - `gatherSubdomains()`: performs subdomain discovery with dictionary attack to find unknown subdomains. Stage 4 - `gatherTechnologies()`: detects technologies used by the website (framework, library, server, CMS) using Wappalyzer patterns. Implements `updateProgress()` which sends progress events to monitoring agent via message bus each time a stage completes. Results are stored in `ReconResult` struct with fields: `Domain string`, `Whois WhoisInfo`, `DNS DnsInfo`, `Subdomains []string`, `Technologies []string`, `CreatedAt time.Time`. This agent also implements caching to avoid re-reconnaissance on the same domain within 24 hours.

---

#### **3. `analysis_agent.rs` (Rust)**

**Task:** Run complex analysis pipelines on collected data to find vulnerabilities and patterns.

**Purpose:** This agent runs an analysis pipeline with 5 sequential stages: `DataPreprocessing` (data cleaning and normalization), `PatternDetection` (detect security patterns using regex and Aho-Corasick), `VulnerabilityMatching` (match against CVE database), `RiskScoring` (calculate risk score with CVSS + business context), `ReportGeneration` (generate initial report). Each stage receives input from the previous stage and produces output for the next stage. Implements `cross_reference_analysis()` to correlate findings from multiple sources (DNS, WHOIS, ports, headers, content) - for example: if port 3306 is open and server header shows MySQL, cross-reference will identify known MySQL vulnerabilities. Uses `Trie` and `Aho-Corasick` for fast pattern matching on large text. Implements `confidence_scoring()` for each finding: high (100-75%) if multiple evidence found, medium (74-50%) if single evidence with verification, low (49-25%) if single evidence without verification, info (24-0%) for additional information. Stores results in `AnalysisResult` struct with fields: `findings: Vec<Finding>`, `summary: String`, `risk_score: f32`, `confidence: f32`.

---

#### **4. `reporting_agent.py` (Python)**

**Task:** Generate reports in various formats based on analyzed data.

**Purpose:** This agent uses strategy pattern to select reporter based on requested format (JSONReporter, TXTReporter, DOCSReporter, CSVReporter, HTMLReporter, PDFReporter). Each reporter implements `generate(data: AnalysisResult) -> bytes` to produce output in specific format. Uses `jinja2` templates for HTML and DOCX reports with user-customizable templates. Implements `executive_summary()` with AI model (LLama3/Mistral) to generate natural language summary: input findings list, prompt engineering for structured output format, and post-processing to ensure professional and informative output. Implements `schedule_report()` with `apscheduler` for regular reporting: runs report generation periodically (daily, weekly, monthly) and sends to stakeholders via email. Implements `customize_report()` to allow users to select templates, colors, branding, and content to be included in the report.

---

#### **5. `monitoring_agent.go` (Go)**

**Task:** Perform scheduled monitoring to detect changes on the target website.

**Purpose:** This agent starts a `cron` scheduler and runs jobs according to specified schedule (e.g., every 6 hours, daily, weekly). Each job: `checkChanges()` - performs a brief scan on the target website and compares results with baseline/previous results. Detects changes such as: new vulnerabilities found, ports changing status (open/close), new subdomains, header security configuration changes, SSL certificate updates, new technologies detected. Implements `generateAlert()` for detected changes with severity based on impact: critical (new high severity vulnerability), high (ports changes, SSL expiry), medium (new subdomain, header changes), low (new technology detected). Sends alerts to handlers: email (`smtp` with TLS), webhook (`http` with retry mechanism max 3 attempts), and Telegram (`telegram` bot API). Implements `storeHistory()` with time-series database (InfluxDB) for trend analysis - enables visualization of security posture changes over time. Implements `baseline_management()` to store initial baseline and periodically update baseline.

---

#### **6. `model_integration.rs` (Rust)**

**Task:** Connect core system with all AI models and manage inference.

**Purpose:** This agent manages the lifecycle of all AI/ML models used by the system. Implements `load_models()` which loads from local path or downloads from registry (Hugging Face) if new version is available. Model types: `MLScanner` (scikit-learn pickle for classification), `AnomalyDetector` (Rust implementation for anomaly detection), `NLPProcessor` (transformers for NLP tasks). Implements `run_inference(data: &[u8], model_name: &str) -> InferenceResult` with inference caching using `LruCache<String, InferenceResult>` to reduce redundant computation (cache based on hash of input data). Implements `model_versioning()`: each model has version tag, auto-update if new version available in registry, and rollback capability if new version causes performance degradation. Implements `distributed_inference()`: if data is large, split across threads using `rayon` for parallel processing. Implements `model_health_check()` to monitor model performance (accuracy, latency, memory usage) and alert if performance drops below threshold.

---

**Indonesian:**

---

## 📂 **`agents/` - AGEN CERDAS**

---

#### **1. `base_agent.rs` (Rust)**

**Tugas:** Menyediakan class dasar/template yang menjadi fondasi untuk semua agent dalam sistem.

**Tujuan:** File ini mengimplementasikan trait `Agent` dengan method-method yang harus diimplementasikan oleh semua agent turunan: `fn init(&mut self) -> Result<()>` untuk menginisialisasi agent, `fn run(&mut self) -> Result<()>` untuk menjalankan tugas utama agent, `fn pause(&mut self) -> Result<()>` untuk menjeda operasi, `fn resume(&mut self) -> Result<()>` untuk melanjutkan operasi yang dijeda, `fn shutdown(&mut self) -> Result<()>` untuk menghentikan agent dengan graceful cleanup, `fn get_state(&self) -> AgentState` untuk mendapatkan state saat ini, dan `fn set_state(&mut self, state: AgentState)` untuk mengubah state. Implementasi `AgentManager` struct dengan fields: `agents: HashMap<String, Box<dyn Agent>>` untuk menyimpan semua agent, `message_bus: Arc<MessageBus>` untuk komunikasi antar agent, `state_store: Arc<dyn StateStore>` untuk persistence state menggunakan `sled` database dengan TTL. Implementasi `broadcast_message(message: AgentMessage)` untuk komunikasi antar agent melalui message bus. Implementasi `heartbeat()` system: setiap agent mengirim heartbeat setiap 30 detik, manager monitor dan restart jika heartbeat missing. Error handling dengan `anyhow` untuk recoverable errors dan `panic` untuk unrecoverable, dengan automatic recovery via `supervisor` pattern yang akan merestart agent yang crash.

---

#### **2. `reconnaissance_agent.go` (Go)**

**Tugas:** Melakukan pengintaian awal terhadap target website untuk mengumpulkan data dasar sebelum scanning mendalam.

**Tujuan:** Agen ini menjalankan 4 stage pengintaian secara paralel menggunakan goroutine dengan `sync.WaitGroup`. Stage 1 - `gatherWhois()`: melakukan WHOIS lookup untuk mendapatkan informasi kepemilikan domain. Stage 2 - `gatherDNS()`: melakukan DNS enumeration untuk mengumpulkan semua DNS records (A, AAAA, CNAME, MX, TXT, NS, SOA). Stage 3 - `gatherSubdomains()`: melakukan subdomain discovery dengan dictionary attack untuk menemukan subdomain yang tidak diketahui. Stage 4 - `gatherTechnologies()`: mendeteksi teknologi yang digunakan website (framework, library, server, CMS) menggunakan Wappalyzer patterns. Implementasi `updateProgress()` yang mengirim progress events ke monitoring agent melalui message bus setiap kali stage selesai. Hasil disimpan di `ReconResult` struct dengan fields: `Domain string`, `Whois WhoisInfo`, `DNS DnsInfo`, `Subdomains []string`, `Technologies []string`, `CreatedAt time.Time`. Agen ini juga mengimplementasikan caching untuk menghindari pengintaian ulang pada domain yang sama dalam waktu 24 jam.

---

#### **3. `analysis_agent.rs` (Rust)**

**Tugas:** Menjalankan pipeline analisis kompleks pada data yang telah dikumpulkan untuk menemukan kerentanan dan pola.

**Tujuan:** Agen ini menjalankan pipeline analisis dengan 5 stage berurutan: `DataPreprocessing` (membersihkan dan normalisasi data), `PatternDetection` (mendeteksi pola keamanan menggunakan regex dan Aho-Corasick), `VulnerabilityMatching` (mencocokkan dengan database CVE), `RiskScoring` (menghitung risk score dengan CVSS + business context), `ReportGeneration` (membuat laporan awal). Setiap stage menerima input dari previous stage dan menghasilkan output untuk next stage. Implementasi `cross_reference_analysis()` untuk mengkorelasikan temuan dari multiple sources (DNS, WHOIS, ports, headers, content) - contoh: jika port 3306 terbuka dan server header menunjukkan MySQL, maka cross-reference akan mengidentifikasi potensi kerentanan MySQL yang diketahui. Menggunakan `Trie` dan `Aho-Corasick` untuk fast pattern matching pada teks yang besar. Implementasi `confidence_scoring()` untuk setiap finding: high (100-75%) jika ditemukan multiple evidence, medium (74-50%) jika single evidence dengan verifikasi, low (49-25%) jika single evidence tanpa verifikasi, info (24-0%) untuk informasi tambahan. Simpan results di `AnalysisResult` struct dengan fields: `findings: Vec<Finding>`, `summary: String`, `risk_score: f32`, `confidence: f32`.

---

#### **4. `reporting_agent.py` (Python)**

**Tugas:** Mengenerate laporan dalam berbagai format berdasarkan data yang telah dianalisis.

**Tujuan:** Agen ini menggunakan strategy pattern untuk memilih reporter berdasarkan format yang diminta (JSONReporter, TXTReporter, DOCSReporter, CSVReporter, HTMLReporter, PDFReporter). Setiap reporter mengimplementasikan `generate(data: AnalysisResult) -> bytes` untuk menghasilkan output dalam format tertentu. Menggunakan `jinja2` templates untuk HTML dan DOCX reports dengan template yang dapat disesuaikan pengguna. Implementasi `executive_summary()` dengan AI model (LLama3/Mistral) untuk generate natural language summary: input findings list, prompt engineering untuk format output yang terstruktur, dan post-processing untuk memastikan output profesional dan informatif. Implementasi `schedule_report()` dengan `apscheduler` untuk regular reporting: menjalankan report generation secara periodik (daily, weekly, monthly) dan mengirim ke stakeholder melalui email. Implementasi `customize_report()` untuk memungkinkan user memilih template, warna, branding, dan konten yang akan disertakan dalam laporan.

---

#### **5. `monitoring_agent.go` (Go)**

**Tugas:** Melakukan monitoring berjadwal untuk mendeteksi perubahan pada website target.

**Tujuan:** Agen ini memulai `cron` scheduler dan menjalankan job sesuai schedule yang ditentukan (misal: setiap 6 jam, daily, weekly). Setiap job: `checkChanges()` - melakukan scanning singkat pada website target dan membandingkan hasil dengan baseline/previous results. Mendeteksi perubahan seperti: new vulnerabilities ditemukan, ports yang berubah status (open/close), new subdomains, header security configuration changes, SSL certificate updates, new technologies detected. Implementasi `generateAlert()` untuk detected changes dengan severity berdasarkan impact: critical (new vulnerability high severity), high (ports changes, SSL expiry), medium (new subdomain, header changes), low (new technology detected). Kirim alert ke handlers: email (`smtp` with TLS), webhook (`http` with retry mechanism max 3 attempts), dan Telegram (`telegram` bot API). Implementasi `storeHistory()` dengan time-series database (InfluxDB) untuk trend analysis - memungkinkan visualisasi perubahan security posture over time. Implementasi `baseline_management()` untuk menyimpan baseline pertama kali dan update baseline secara periodik.

---

#### **6. `model_integration.rs` (Rust)**

**Tugas:** Menghubungkan core system dengan semua model AI dan mengelola inference.

**Tujuan:** Agen ini mengelola lifecycle dari semua AI/ML models yang digunakan sistem. Implementasi `load_models()` yang melakukan load dari path lokal atau download dari registry (Hugging Face) jika versi baru tersedia. Model types: `MLScanner` (scikit-learn pickle untuk klasifikasi), `AnomalyDetector` (Rust implementation untuk anomaly detection), `NLPProcessor` (transformers untuk NLP tasks). Implementasi `run_inference(data: &[u8], model_name: &str) -> InferenceResult` dengan inference caching menggunakan `LruCache<String, InferenceResult>` untuk mengurangi redundant computation (cache berdasarkan hash dari input data). Implementasi `model_versioning()`: setiap model punya version tag, auto-update jika ada version baru di registry, dan rollback capability jika version baru menyebabkan performance degradation. Implementasi `distributed_inference()`: jika data besar, split across threads menggunakan `rayon` untuk parallel processing. Implementasi `model_health_check()` untuk memonitoring performa model (accuracy, latency, memory usage) dan alert jika performa turun di bawah threshold.

---

**models folder explanation:**

---

## 📂 **`models/` - AI MODELS**

---

#### **1. `ml_scanner.py` (Python)**

**Task:** Provide machine learning models for classification and security pattern detection.

**Purpose:** This file implements a complete machine learning pipeline for detecting threat patterns. Implements `train()`: load dataset from CSV file containing feature vectors (response headers, status codes, content length, TTL, response time, etc.) and labels (benign, suspicious, malicious). Split data into train/test set (80/20). Train multiple models: RandomForestClassifier (for feature importance interpretation), Support Vector Machine (for high-dimensional data), Gradient Boosting (for high accuracy), Decision Trees (for interpretability). Hyperparameter tuning with `GridSearchCV` to find optimal parameters. Feature engineering: extract headers (HSTS, CSP, X-Frame-Options presence), response codes (200, 301, 302, 404, 500), content length, TTL, response time, number of external links, etc. Implements `predict(features: pd.DataFrame) -> Dict[str, float]` to generate predictions: `threat_probability` (0-1), `anomaly_score` (0-1), `classification` (benign, suspicious, malicious). Implements `explain_prediction()` with SHAP values for feature importance - explains why the model gave a particular prediction and which features were most influential. Implements `model_persistence()` to save and load models using `pickle` or `joblib`.

---

#### **2. `anomaly_detector.rs` (Rust)**

**Task:** Detect anomalies in data using various statistical methods.

**Purpose:** This file implements multiple anomaly detection techniques. Implements `fit(data: &[Vec<f64>])` using `IsolationForest` from Rust `linfa` or `ndarray` with custom implementation. Builds binary trees with `max_samples=256`, `n_trees=100` for isolation forest algorithm. Each tree isolates points with random splits, anomalies have shorter path lengths. Implements `detect(data: &[Vec<f64>]) -> Vec<Anomaly>` by computing anomaly score for each sample: score = 2^(-average_path_length / average_path_length_all). Implements `detect_time_series(series: &[f64]) -> Vec<Anomaly>` with `Z-score` method: calculate mean and standard deviation, points with Z-score > 3 are considered anomalies. Implements `IQR` method: detect outliers based on Interquartile Range, points outside Q1 - 1.5*IQR or Q3 + 1.5*IQR are considered anomalies. Implements `adaptive_threshold()`: dynamic threshold based on data distribution using `3-sigma` rule - threshold moves according to changes in data distribution. Implements `seasonal_anomaly_detection()` for time series with seasonal patterns: detrend data, decompose into seasonal, trend, residual, detect anomalies in residual.

---

#### **3. `pattern_recognizer.rs` (Rust)**

**Task:** Recognize patterns in data using various pattern matching techniques.

**Purpose:** This file implements multiple pattern recognition techniques. Implements `add_pattern(pattern: Pattern)` with compilation into Aho-Corasick automaton for efficient multiple pattern matching (O(n) time complexity). Pattern types: `regex` (regular expression patterns), `substring` (exact string matching), `regex_set` (multiple regex patterns). Implements `find_patterns(input: &str) -> Vec<PatternMatch>` by scanning input using automaton, recording all matches with positions (start, end, pattern_id). Implements `sequence_patterns()` to detect patterns in sequence data (packet sequences, behavior sequences): uses Dynamic Time Warping (DTW) for sequence alignment, Longest Common Subsequence (LCS) for pattern length. Uses `Boyer-Moore-Horspool` for fast substring search for single patterns. Implements `behavior_fingerprint()` to identify unique patterns that indicate specific attacks or activities: builds signature database from known attack patterns (SQL injection patterns, XSS patterns, header manipulation patterns). Implements `pattern_frequency_analysis()` to analyze pattern occurrence frequency and detect patterns that appear abnormally.

---

#### **4. `nlp_processor.py` (Python)**

**Task:** Process natural language for information extraction and content analysis.

**Purpose:** This file implements various NLP tasks using Hugging Face Transformers and spaCy. Implements `extract_entities(text: str) -> List[Entity]` using Hugging Face transformers `ner` pipeline with `dslim/bert-base-NER` model fine-tuned for cybersecurity domain. Extracts entity types: `ORG` (organization names), `PERSON` (person names), `LOCATION` (locations), `DATE` (dates), `MISC` (miscellaneous), `TECHNOLOGY` (framework, library, tool names), `VERSION` (software versions). Implements `sentiment_analysis(text: str) -> SentimentResult` using `textblob` or `vader` for sentiment polarity (positive, negative, neutral) and subjectivity (objective, subjective). Implements `summarize(text: str, max_length: int) -> str` using `facebook/bart-large-cnn` for abstractive summarization - produces summary that not only extracts sentences but rewrites with new words. Implements `topic_modeling(documents: List[str]) -> List[Topic]` with LDA (Latent Dirichlet Allocation) from `gensim` to find main topics in document collection, extracts top N topics with most representative words. Implements `keyword_extraction()` using `RAKE` (Rapid Automatic Keyword Extraction) or `YAKE` to extract important keywords from text.

---

#### **5. `risk_scorer.rs` (Rust)**

**Task:** Calculate comprehensive risk scores based on CVSS and business context.

**Purpose:** This file implements risk score calculation using CVSS v3.1 metrics. CVSS vector components: `AV` (Attack Vector: Network, Adjacent, Local, Physical), `AC` (Attack Complexity: Low, High), `PR` (Privileges Required: None, Low, High), `UI` (User Interaction: None, Required), `S` (Scope: Unchanged, Changed), `C` (Confidentiality Impact: None, Low, High), `I` (Integrity Impact: None, Low, High), `A` (Availability Impact: None, Low, High). Calculates base score using formula: `If (Impact <= 0) base = 0; else base = min(10, 8.22 * Exploitability * Impact)`. Impact = 1 - (1-C) * (1-I) * (1-A) for scope unchanged, or 1 - (1-C) * (1-I) * (1-A) * 1.08 for scope changed. Exploitability = 8.22 * AV * AC * PR * UI. Adds business factors: `asset_value` (asset value affected: low, medium, high, critical), `data_sensitivity` (data sensitivity: public, internal, confidential, restricted), `regulatory_impact` (regulatory impact: None, GDPR, HIPAA, PCI-DSS). Implements `calculate_temporal_score()` to incorporate temporal metrics (exploit code maturity, remediation level, report confidence). Implements `calculate_environmental_score()` to adjust based on environment (security requirements: low, medium, high for confidentiality, integrity, availability). Generates `RiskScore` struct with fields: `base_score: f64`, `temporal_score: f64`, `environmental_score: f64`, `business_score: f64`, `priority: Priority` (P0-Critical, P1-High, P2-Medium, P3-Low, P4-Info). Implements `generate_recommendations()` to provide remediation recommendations based on score and vulnerability type.

---

**Indonesian:**

---

## 📂 **`models/` - MODEL AI**

---

#### **1. `ml_scanner.py` (Python)**

**Tugas:** Menyediakan model machine learning untuk klasifikasi dan deteksi pola keamanan.

**Tujuan:** File ini mengimplementasikan pipeline machine learning lengkap untuk mendeteksi threat patterns. Implementasi `train()`: load dataset dari CSV file yang berisi feature vectors (response headers, status codes, content length, TTL, response time, etc) dan labels (benign, suspicious, malicious). Split data menjadi train/test set (80/20). Train multiple models: RandomForestClassifier (untuk interpretasi feature importance), Support Vector Machine (untuk high-dimensional data), Gradient Boosting (untuk high accuracy), Decision Trees (untuk interpretability). Tuning hyperparameters dengan `GridSearchCV` untuk mencari parameter optimal. Feature engineering: extract headers (HSTS, CSP, X-Frame-Options presence), response codes (200, 301, 302, 404, 500), content length, TTL, response time, number of external links, etc. Implementasi `predict(features: pd.DataFrame) -> Dict[str, float]` untuk menghasilkan predictions: `threat_probability` (0-1), `anomaly_score` (0-1), `classification` (benign, suspicious, malicious). Implementasi `explain_prediction()` dengan SHAP values untuk feature importance - menjelaskan mengapa model memberikan prediksi tertentu dan feature apa yang paling berpengaruh. Implementasi `model_persistence()` untuk save dan load model menggunakan `pickle` atau `joblib`.

---

#### **2. `anomaly_detector.rs` (Rust)**

**Tugas:** Mendeteksi anomali dalam data menggunakan berbagai statistical methods.

**Tujuan:** File ini mengimplementasikan multiple anomaly detection techniques. Implementasi `fit(data: &[Vec<f64>])` menggunakan `IsolationForest` dari Rust `linfa` atau `ndarray` dengan custom implementation. Build binary trees dengan `max_samples=256`, `n_trees=100` untuk isolation forest algorithm. Setiap tree mengisolasi points dengan random splits, anomalies memiliki path length yang lebih pendek. Implementasi `detect(data: &[Vec<f64>]) -> Vec<Anomaly>` dengan compute anomaly score untuk setiap sample: score = 2^(-average_path_length / average_path_length_all). Implementasi `detect_time_series(series: &[f64]) -> Vec<Anomaly>` dengan `Z-score` method: hitung mean dan standard deviation, point dengan Z-score > 3 dianggap anomaly. Implementasi `IQR` method: detect outliers based on Interquartile Range, points outside Q1 - 1.5*IQR atau Q3 + 1.5*IQR dianggap anomaly. Implementasi `adaptive_threshold()`: dynamic threshold based on data distribution using `3-sigma` rule - threshold bergerak sesuai dengan perubahan distribusi data. Implementasi `seasonal_anomaly_detection()` untuk time series dengan pola musiman: detrend data, decompose into seasonal, trend, residual, detect anomalies in residual.

---

#### **3. `pattern_recognizer.rs` (Rust)**

**Tugas:** Mengenali pola dalam data menggunakan berbagai teknik pattern matching.

**Tujuan:** File ini mengimplementasikan multiple pattern recognition techniques. Implementasi `add_pattern(pattern: Pattern)` dengan compile ke Aho-Corasick automaton untuk multiple pattern matching yang efisien (O(n) time complexity). Pattern types: `regex` (regular expression patterns), `substring` (exact string matching), `regex_set` (multiple regex patterns). Implementasi `find_patterns(input: &str) -> Vec<PatternMatch>` dengan scan input using automaton, record all matches with positions (start, end, pattern_id). Implementasi `sequence_patterns()` untuk detect patterns in sequence data (packet sequences, behavior sequences): menggunakan Dynamic Time Warping (DTW) untuk sequence alignment, Longest Common Subsequence (LCS) untuk pattern length. Menggunakan `Boyer-Moore-Horspool` untuk substring search yang cepat untuk single pattern. Implementasi `behavior_fingerprint()` untuk identify unique patterns that indicate specific attacks or activities: build signature database dari known attack patterns (SQL injection patterns, XSS patterns, header manipulation patterns). Implementasi `pattern_frequency_analysis()` untuk menganalisis frekuensi kemunculan pattern dan mendeteksi pattern yang muncul secara tidak normal.

---

#### **4. `nlp_processor.py` (Python)**

**Tugas:** Memproses bahasa alami untuk ekstraksi informasi dan analisis konten.

**Tujuan:** File ini mengimplementasikan berbagai NLP tasks menggunakan Hugging Face Transformers dan spaCy. Implementasi `extract_entities(text: str) -> List[Entity]` menggunakan Hugging Face transformers `ner` pipeline dengan model `dslim/bert-base-NER` yang di-fine-tune untuk cybersecurity domain. Ekstrak entity types: `ORG` (organization names), `PERSON` (person names), `LOCATION` (locations), `DATE` (dates), `MISC` (miscellaneous), `TECHNOLOGY` (framework, library, tool names), `VERSION` (software versions). Implementasi `sentiment_analysis(text: str) -> SentimentResult` menggunakan `textblob` atau `vader` untuk sentiment polarity (positive, negative, neutral) dan subjectivity (objective, subjective). Implementasi `summarize(text: str, max_length: int) -> str` menggunakan `facebook/bart-large-cnn` untuk abstractive summarization - menghasilkan ringkasan yang tidak hanya mengekstrak kalimat tetapi menulis ulang dengan kata-kata baru. Implementasi `topic_modeling(documents: List[str]) -> List[Topic]` dengan LDA (Latent Dirichlet Allocation) from `gensim` untuk menemukan topik-topik utama dalam kumpulan dokumen, ekstrak top N topics dengan kata-kata yang paling representatif. Implementasi `keyword_extraction()` menggunakan `RAKE` (Rapid Automatic Keyword Extraction) atau `YAKE` untuk mengekstrak keyword penting dari teks.

---

#### **5. `risk_scorer.rs` (Rust)**

**Tugas:** Menghitung risk score secara komprehensif berdasarkan CVSS dan business context.

**Tujuan:** File ini mengimplementasikan perhitungan risk score menggunakan CVSS v3.1 metrics. CVSS vector components: `AV` (Attack Vector: Network, Adjacent, Local, Physical), `AC` (Attack Complexity: Low, High), `PR` (Privileges Required: None, Low, High), `UI` (User Interaction: None, Required), `S` (Scope: Unchanged, Changed), `C` (Confidentiality Impact: None, Low, High), `I` (Integrity Impact: None, Low, High), `A` (Availability Impact: None, Low, High). Calculate base score using formula: `If (Impact <= 0) base = 0; else base = min(10, 8.22 * Exploitability * Impact)`. Impact = 1 - (1-C) * (1-I) * (1-A) for scope unchanged, atau 1 - (1-C) * (1-I) * (1-A) * 1.08 for scope changed. Exploitability = 8.22 * AV * AC * PR * UI. Tambahkan business factor: `asset_value` (nilai aset yang terkena dampak: low, medium, high, critical), `data_sensitivity` (sensitivitas data: public, internal, confidential, restricted), `regulatory_impact` (dampak regulasi: None, GDPR, HIPAA, PCI-DSS). Implementasi `calculate_temporal_score()` untuk incorporate temporal metrics (exploit code maturity, remediation level, report confidence). Implementasi `calculate_environmental_score()` untuk adjust berdasarkan environment (security requirements: low, medium, high untuk confidentiality, integrity, availability). Hasilkan `RiskScore` struct dengan fields: `base_score: f64`, `temporal_score: f64`, `environmental_score: f64`, `business_score: f64`, `priority: Priority` (P0-Critical, P1-High, P2-Medium, P3-Low, P4-Info). Implementasi `generate_recommendations()` untuk memberikan rekomendasi remediasi berdasarkan score dan vulnerability type.

---

**storage folder explanation:**

---

## 📂 **`storage/` - DATA STORAGE**

---

#### **1. `data_warehouse.rs` (Rust)**

**Task:** Manage main data storage with indexing, versioning, and query optimization.

**Purpose:** This file serves as the central data warehouse that aggregates all data from various sources. Implements `store_data(key: &str, data: &[u8]) -> Result<()>` with sharding based on `shard_key` to distribute data evenly and prevent bottlenecks on a single storage node. Implements `query_data(query: Query) -> Result<Vec<Data>>` with multiple indexing strategies in memory: `HashSet` for fast exact key lookup (O(1)), `BTreeMap` for range queries (O(log n)), `InvertedIndex` for full-text search on content. Implements `compact()` for data deduplication (removing duplicate entries) and compression using `Snappy` or `LZ4` to save storage space. Implements `versioning()` with `timestamp` per entry: each data update creates a new version, supports `get_version(key: &str, version: &str) -> Result<Data>` for rollback to previous versions. Implements `data_retention_policy()`: automatically archive data older than 90 days to cold storage and delete data older than 365 days according to retention policy. Implements `backup_trigger()` to perform automatic backups every 24 hours to the specified location.

---

#### **2. `json_handler.rs` (Rust)**

**Task:** Handle serialization and deserialization of data to and from JSON format.

**Purpose:** This file manages all JSON-related operations. Implements `to_json<T: Serialize>(data: &T) -> Result<String>` using `serde_json::to_string_pretty()` with `Serialize` trait to produce formatted JSON with indentation for human readability. Implements `from_json<T: Deserialize>(data: &str) -> Result<T>` using `serde_json::from_str()` with `Deserialize` trait for parsing JSON into Rust data structures. Implements `validate_json(data: &str, schema: &JsonSchema)` using `jsonschema` library to validate that JSON conforms to the specified schema (e.g., ensuring required fields exist, data types are correct). Implements `transform_json(data: &str, jq: &str)` using `jq` JSON processor to query and transform JSON data (e.g., `.data | map(select(.severity == "critical"))` to filter critical findings). Implements `pretty_print(data: &str) -> String` to format JSON with consistent indentation. Implements `stream_json()` to handle large JSON streaming (over 100MB) without loading entire data into memory.

---

#### **3. `txt_generator.rs` (Rust)**

**Task:** Generate text files with specific formatting for various purposes.

**Purpose:** This file produces text output in various formats. Implements `generate(data: &StructuredData) -> String` which can produce: `markdown` syntax with `#` for headers, `|` for tables, `-` for lists, ` ``` ` for code blocks. `ASCII tables` with column widths computed based on content (find maximum width of each column), format with `+---+` separators and `|` vertical separators. `structured logging` with KV format `key=value` and escaping for values containing spaces or special characters. `plain text reports` with section headers, bullet points, and numbered lists. Implements `markdown_to_txt()` to convert markdown to plain text for compatibility with systems that do not support markdown. Implements `table_to_txt()` to convert tables to neatly formatted text with alignment (left, center, right) based on data type. Implements `generate_summary()` to create a brief summary of data in text format.

---

#### **4. `docs_builder.rs` (Rust)**

**Task:** Build comprehensive documentation with template engine and professional formatting.

**Purpose:** This file uses `handlebars` template engine for rendering documentation with customizable templates. Template variables: `{{#each findings}}` for iterating findings, `{{severity}}` to display severity, `{{description}}` for description, `{{remediation}}` for remediation recommendations, `{{#if condition}}` for conditional rendering. Implements `generate_toc()` for table of contents based on heading levels (h1, h2, h3) with page numbers. Implements `cross_reference()` for internal links between sections (e.g., "see section 3.2 for details"). Implements `bibliography()` for reference list with APA or IEEE style formatting. Implements `generate_cover_page()` to create a cover page with document title, date, version, and author. Implements `apply_template()` to apply user-selected template (default, professional, minimal, dark). Implements `export_docs(format: DocFormat)` to export in DOCX, ODT, or HTML format.

---

#### **5. `csv_exporter.rs` (Rust)**

**Task:** Export and import data in CSV format for spreadsheet analysis.

**Purpose:** This file uses the `csv` crate to handle CSV operations. Implements `export<T: Serialize>(data: &[T]) -> Result<String>` which detects headers from struct fields with `serde` rename attribute for custom column names. Escapes values containing delimiters or newlines with quotes. Implements `export_with_schema(data: &[T], schema: &Schema)` for custom column mapping - allows users to specify which columns to export and their order. Implements `import<T: Deserialize>(data: &str) -> Result<Vec<T>>` to import data from CSV into Rust structures. Supports multiple delimiter options: comma (,), semicolon (;), tab (\t), and custom delimiters. Type inference: automatically determines data type for each column (string, integer, float, boolean, datetime). Implements `quote_strategy`: always (quote all values), never (never quote), when_necessary (quote only if containing delimiter or newline). Implements `null_handling()` to handle null values: replace with empty string, "NULL", or custom placeholder. Implements `export_large_dataset()` to export large datasets (millions of rows) with streaming to avoid memory overload.

---

#### **6. `html_reporter.rs` (Rust)**

**Task:** Generate interactive and responsive HTML reports.

**Purpose:** This file uses `handlebars` with HTML templates to generate reports that can be opened in a browser. Integrates `Chart.js` for data visualization: bar charts for severity distribution, pie charts for vulnerability type percentages, line charts for activity timeline, radar charts for comparing multiple metrics. Implements `interactive_tables()` with JavaScript `DataTable` library for sorting (click header to sort), filtering (text box per column), pagination (show 10/25/50/100 rows per page). Implements `responsive_design()` with CSS media queries for adaptation to various screen sizes (desktop, tablet, mobile). Implements `print_styles()` for print optimization: hide interactive elements, show all content, add page breaks, remove background colors. Implements `dark_mode_toggle()` with CSS variables and JavaScript to toggle between light and dark themes. Implements `export_html()` to save the report as an HTML file that can be opened offline. Implements `embed_images()` to embed images (screenshots, charts) as base64 so the report is self-contained without external dependencies.

---

#### **7. `pdf_generator.py` (Python)**

**Task:** Generate professional PDFs with formatting and security features.

**Purpose:** This file uses `pdfkit` (wkhtmltopdf wrapper) or `WeasyPrint` to generate PDF from HTML. Implements `generate_pdf(html: str, options: dict) -> bytes` with options: margins (top, bottom, left, right in mm), page size (A4, Letter, Legal), orientation (portrait, landscape), font size, and page numbering. Implements `add_watermark(pdf: bytes, watermark_text: str) -> bytes` using `PyPDF2` to overlay watermark text (diagonal) on each page with 0.3 opacity. Implements `add_signature(pdf: bytes, signature_image: bytes) -> bytes` to insert digital signature image on the last page with timestamp. Implements `add_metadata()` to add PDF metadata: title, author, subject, keywords, creation date. Implements `encrypt_pdf()` with AES-128 encryption for password protection with user password and owner password. Implements `generate_from_template(data: dict, template_path: str) -> bytes` with `jinja2` rendering for customizable PDF templates. Implements `add_table_of_contents()` to generate TOC with accurate page numbers. Implements `add_headers_footers()` to add headers (section title) and footers (page number, company name) on every page.

---

**indonesian:**

---

## 📂 **`storage/` - PENYIMPANAN DATA**

---

#### **1. `data_warehouse.rs` (Rust)**

**Tugas:** Mengelola penyimpanan data utama dengan indexing, versioning, dan query optimization.

**Tujuan:** File ini berfungsi sebagai gudang data sentral yang mengagregasi semua data dari berbagai sumber. Implementasi `store_data(key: &str, data: &[u8]) -> Result<()>` dengan sharding berdasarkan `shard_key` untuk mendistribusikan data secara merata dan mencegah bottleneck pada satu storage node. Implementasi `query_data(query: Query) -> Result<Vec<Data>>` dengan multiple indexing strategies di memory: `HashSet` untuk fast exact key lookup (O(1)), `BTreeMap` untuk range queries (O(log n)), `InvertedIndex` untuk full-text search pada konten. Implementasi `compact()` untuk data deduplication (menghapus duplicate entries) dan compression menggunakan `Snappy` atau `LZ4` untuk menghemat ruang penyimpanan. Implementasi `versioning()` dengan `timestamp` per entry: setiap update data membuat version baru, support `get_version(key: &str, version: &str) -> Result<Data>` untuk rollback ke versi sebelumnya. Implementasi `data_retention_policy()`: automatic archive data older than 90 days ke cold storage dan delete data older than 365 days sesuai kebijakan retensi. Implementasi `backup_trigger()` untuk melakukan backup otomatis setiap 24 jam ke lokasi yang ditentukan.

---

#### **2. `json_handler.rs` (Rust)**

**Tugas:** Menangani serialisasi dan deserialisasi data ke dan dari format JSON.

**Tujuan:** File ini mengelola semua operasi terkait JSON. Implementasi `to_json<T: Serialize>(data: &T) -> Result<String>` menggunakan `serde_json::to_string_pretty()` dengan `Serialize` trait untuk menghasilkan JSON yang diformat dengan indentation untuk human readability. Implementasi `from_json<T: Deserialize>(data: &str) -> Result<T>` menggunakan `serde_json::from_str()` dengan `Deserialize` trait untuk parsing JSON ke struktur data Rust. Implementasi `validate_json(data: &str, schema: &JsonSchema)` menggunakan `jsonschema` library untuk memvalidasi bahwa JSON sesuai dengan schema yang ditentukan (misal: memastikan field required ada, tipe data sesuai). Implementasi `transform_json(data: &str, jq: &str)` menggunakan `jq` JSON processor untuk query dan transform data JSON (contoh: `.data | map(select(.severity == "critical"))` untuk filter temuan critical). Implementasi `pretty_print(data: &str) -> String` untuk memformat JSON dengan indentation yang konsisten. Implementasi `stream_json()` untuk menangani JSON streaming yang besar (lebih dari 100MB) tanpa loading seluruh data ke memory.

---

#### **3. `txt_generator.rs` (Rust)**

**Tugas:** Mengenerate file teks dengan formatting khusus untuk berbagai keperluan.

**Tujuan:** File ini menghasilkan output teks dalam berbagai format. Implementasi `generate(data: &StructuredData) -> String` yang dapat menghasilkan: `markdown` syntax dengan `#` untuk headers, `|` untuk tables, `-` untuk lists, ` ``` ` untuk code blocks. `ASCII tables` dengan compute column widths based on content (cari lebar maksimum setiap kolom), format with `+---+` separators dan `|` vertical separators. `structured logging` dengan KV format `key=value` dan escaping untuk nilai yang mengandung spasi atau special characters. `plain text reports` dengan section headers, bullet points, dan numbered lists. Implementasi `markdown_to_txt()` untuk convert markdown ke plain text untuk compatibility dengan sistem yang tidak support markdown. Implementasi `table_to_txt()` untuk convert tabel ke format teks yang rapi dengan alignment (left, center, right) berdasarkan tipe data. Implementasi `generate_summary()` untuk membuat ringkasan singkat dari data dalam format teks.

---

#### **4. `docs_builder.rs` (Rust)**

**Tugas:** Membangun dokumentasi lengkap dengan template engine dan formatting profesional.

**Tujuan:** File ini menggunakan `handlebars` template engine untuk rendering dokumentasi dengan template yang dapat disesuaikan. Template variables: `{{#each findings}}` untuk iterasi findings, `{{severity}}` untuk menampilkan severity, `{{description}}` untuk deskripsi, `{{remediation}}` untuk rekomendasi perbaikan, `{{#if condition}}` untuk conditional rendering. Implementasi `generate_toc()` untuk table of contents berdasarkan heading levels (h1, h2, h3) dengan page numbers. Implementasi `cross_reference()` untuk internal links antar sections (contoh: "see section 3.2 for details"). Implementasi `bibliography()` untuk reference list dengan formatting APA atau IEEE style. Implementasi `generate_cover_page()` untuk membuat halaman sampul dengan judul dokumen, tanggal, versi, dan author. Implementasi `apply_template()` untuk menerapkan template yang dipilih pengguna (default, professional, minimal, dark). Implementasi `export_docs(format: DocFormat)` untuk mengekspor dalam format DOCX, ODT, atau HTML.

---

#### **5. `csv_exporter.rs` (Rust)**

**Tugas:** Mengekspor dan mengimpor data dalam format CSV untuk analisis spreadsheet.

**Tujuan:** File ini menggunakan `csv` crate untuk menangani operasi CSV. Implementasi `export<T: Serialize>(data: &[T]) -> Result<String>` yang mendeteksi headers dari struct fields dengan `serde` rename attribute untuk custom column names. Escape values yang mengandung delimiter atau newline dengan quotes. Implementasi `export_with_schema(data: &[T], schema: &Schema)` untuk custom column mapping - memungkinkan user menentukan kolom mana yang akan diekspor dan urutannya. Implementasi `import<T: Deserialize>(data: &str) -> Result<Vec<T>>` untuk mengimpor data dari CSV ke struktur Rust. Mendukung multiple delimiter options: comma (,), semicolon (;), tab (\t), dan custom delimiters. Type inference: otomatis menentukan tipe data setiap kolom (string, integer, float, boolean, datetime). Implementasi `quote_strategy`: always (quote semua nilai), never (tidak pernah quote), when_necessary (quote hanya jika mengandung delimiter atau newline). Implementasi `null_handling()` untuk menangani null values: replace with empty string, "NULL", atau custom placeholder. Implementasi `export_large_dataset()` untuk mengekspor dataset besar (jutaan rows) dengan streaming untuk menghindari memory overload.

---

#### **6. `html_reporter.rs` (Rust)**

**Tugas:** Mengenerate laporan HTML yang interaktif dan responsif.

**Tujuan:** File ini menggunakan `handlebars` dengan HTML template untuk menghasilkan laporan yang dapat dibuka di browser. Integrasi `Chart.js` untuk visualisasi data: bar charts untuk distribusi severity, pie charts untuk persentase jenis vulnerability, line charts untuk timeline activity, radar charts untuk perbandingan multiple metrics. Implementasi `interactive_tables()` dengan JavaScript `DataTable` library untuk sorting (klik header untuk sort), filtering (text box per kolom), pagination (show 10/25/50/100 rows per page). Implementasi `responsive_design()` dengan CSS media queries untuk adaptasi ke berbagai ukuran layar (desktop, tablet, mobile). Implementasi `print_styles()` untuk print optimization: hide interactive elements, show all content, add page breaks, remove background colors. Implementasi `dark_mode_toggle()` dengan CSS variables dan JavaScript untuk toggle antara light dan dark theme. Implementasi `export_html()` untuk menyimpan laporan sebagai file HTML yang dapat dibuka offline. Implementasi `embed_images()` untuk menyisipkan gambar (screenshots, charts) sebagai base64 sehingga laporan mandiri tanpa external dependencies.

---

#### **7. `pdf_generator.py` (Python)**

**Tugas:** Mengenerate PDF profesional dengan formatting dan security features.

**Tujuan:** File ini menggunakan `pdfkit` (wkhtmltopdf wrapper) atau `WeasyPrint` untuk generate PDF dari HTML. Implementasi `generate_pdf(html: str, options: dict) -> bytes` dengan options: margins (top, bottom, left, right dalam mm), page size (A4, Letter, Legal), orientation (portrait, landscape), font size, dan page numbering. Implementasi `add_watermark(pdf: bytes, watermark_text: str) -> bytes` menggunakan `PyPDF2` untuk overlay watermark text (diagonal) pada setiap halaman dengan opacity 0.3. Implementasi `add_signature(pdf: bytes, signature_image: bytes) -> bytes` untuk menyisipkan digital signature image di halaman terakhir dengan timestamp. Implementasi `add_metadata()` untuk menambahkan metadata PDF: title, author, subject, keywords, creation date. Implementasi `encrypt_pdf()` dengan AES-128 encryption untuk password protection dengan user password dan owner password. Implementasi `generate_from_template(data: dict, template_path: str) -> bytes` dengan `jinja2` rendering untuk PDF template yang dapat disesuaikan. Implementasi `add_table_of_contents()` untuk generate TOC dengan page numbers yang akurat. Implementasi `add_headers_footers()` untuk menambahkan header (judul section) dan footer (page number, company name) di setiap halaman.

---

**utils folder explanation:**

---

## 📂 **`utils/` - UTILITY**

---

#### **1. `request_handler.rs` (Rust)**

**Task:** Manage all HTTP requests to the internet with session pooling, retry mechanism, and timeout management.

**Purpose:** This file is the HTTP client handler that manages all HTTP communication to the internet. Implements `send_request(request: Request) -> Result<Response>` with session reuse for HTTP/1.1 keep-alive using `reqwest::Client` with connection pooling to avoid connection creation overhead for each request. Implements `retry_with_backoff()` with configuration: `max_attempts=3`, `initial_backoff=1s`, `multiplier=2`, `jitter=0.1` (to avoid thundering herd problem). Status codes that are retried: 408 (Request Timeout), 429 (Too Many Requests), 500 (Internal Server Error), 502 (Bad Gateway), 503 (Service Unavailable), 504 (Gateway Timeout). Implements `handle_redirects()` with maximum 10 redirects to prevent infinite redirect loops. Implements `set_timeout()` with `tokio::time::timeout` per request: connection timeout 5s, read timeout 10s, write timeout 5s. Implements `cookie_jar()` with `CookieStore` that persists across requests within a session. Implements `custom_headers()` to add custom headers (User-Agent, Accept-Language, Accept-Encoding) according to configuration. Implements `handle_compression()` to support gzip and brotli compression to save bandwidth.

---

#### **2. `proxy_manager.go` (Go)**

**Task:** Manage proxies for stealth scanning with rotation, validation, and anonymity tracking.

**Purpose:** This file manages the proxy pool for performing scanning without being detected. Implements `GetProxy() -> Proxy` with round-robin rotation to distribute load evenly across all proxies in the pool. Implements `ValidateProxy(proxy Proxy) -> bool` with test connection to `httpbin.org/ip` or `api.ipify.org`, timeout 5s, to ensure the proxy is active and working before use. Implements `MarkBadProxy(proxy Proxy)` to remove failed proxies from the pool (if a proxy fails 3 consecutive times, it is permanently removed). Implements `RefreshProxies()` to periodically re-validate proxies (every 5 minutes) and remove dead proxies. Proxy types supported: `HTTP`, `HTTPS`, `SOCKS5` (using `golang.org/x/net/proxy` for SOCKS5). Implements `anonymity_level_tracking()`: measures anonymity level of each proxy based on response headers (X-Forwarded-For, Via) - transparent (sends real IP), anonymous (hides IP), elite (hides all proxy traces). Implements `geolocation_routing()`: selects proxies based on target geolocation to reduce latency.

---

#### **3. `rate_limiter.rs` (Rust)**

**Task:** Manage rate limiting using token bucket algorithm to prevent abuse and avoid detection.

**Purpose:** This file implements the token bucket algorithm with atomic operations for thread safety. Struct `RateLimiter` with fields: `tokens: AtomicUsize` (current tokens), `capacity: usize` (maximum capacity), `fill_rate: usize` (tokens per second), `last_refill: AtomicUsize` (last refill timestamp). `Allow()` method: atomically decrements tokens (using `fetch_sub`), if tokens > 0 returns true (allow request), if tokens == 0 returns false (rate limited). `Refill()` called every second: `tokens = min(capacity, tokens + fill_rate)`. Implements `PerDomainLimiter` with `DashMap<String, RateLimiter>` to provide different rate limits for each target domain (e.g., 10 req/sec for domain, 50 req/sec for IP, 100 req/sec for URL). Implements `SlidingWindowCounter` for time-based rate limiting: stores timestamps in `VecDeque`, checks count in last N seconds (e.g., 100 requests in last 60 seconds). Implements `adaptive_rate_limiting()` that adjusts rate limit based on response time and error rate: if many 429 errors (Too Many Requests), decrease rate; if response time is fast, increase rate. Implements `burst_capacity()` to accommodate short request bursts (e.g., 10 requests in 1 second).

---

#### **4. `concurrency_manager.rs` (Rust)**

**Task:** Manage concurrency with async/await, thread pooling, and semaphore-based resource control.

**Purpose:** This file manages all concurrent operations in the system. Struct `ConcurrencyManager` with fields: `runtime: tokio::Runtime` (Tokio runtime for async operations), `thread_pool: ThreadPool` (Rayon thread pool for blocking operations), `semaphore: Arc<Semaphore>` (to limit access to limited resources), `task_counter: AtomicUsize` (to track active task count). Implements `spawn_task<F: Future>(future: F)` with `tokio::spawn` for non-blocking async tasks. Implements `run_in_thread<T: Send + 'static>(job: impl FnOnce() -> T + Send + 'static)` for blocking operations in thread pool (using `rayon::spawn()`). Implements `semaphore_control()`: `acquire()` for blocking tasks (waits if no slots available), `release()` to signal completion (frees a slot). Implements `graceful_shutdown()`: `signal::ctrl_c()` to catch SIGINT, waits for tasks to complete with 30s timeout, force shutdown if timeout (cancels remaining tasks). Implements `task_priority()`: high priority tasks (scan commands) receive resources first, low priority tasks (report generation) are deferred if resources are limited. Implements `load_shedding()`: if task queue exceeds threshold (1000 tasks), rejects new tasks with error "system overloaded".

---

#### **5. `logging_system.rs` (Rust)**

**Task:** Provide structured logging system with multiple outputs and log levels.

**Purpose:** This file manages all logging in the system. Struct `LoggingSystem` with fields: `logger: Logger` (main logger), `file_appender: RollingFileAppender` (file logging), `remote_appender: Option<RemoteAppender>` (remote logging). Implements `log(level: LogLevel, message: &str, context: &Context)` with structured JSON format: `{timestamp, level, message, context: {component, request_id, user_id, session_id, ip, duration}}`. Log levels: DEBUG (detailed debugging), INFO (general information), WARN (warnings), ERROR (recoverable errors), FATAL (critical errors causing shutdown). Implements `RollingFileAppender`: rotates when file size > 10MB, keeps 5 files (total 50MB), compresses old files with `gzip` to save disk space. Implements `RemoteAppender`: sends logs to remote server via UDP (for performance) or HTTP (for reliability), with batching (flush every 100 entries or every 5 seconds). Implements `Context::new()` to create log context with automatic `span` from `tracing` for correlation across requests. Implements `mask_sensitive_data()` to mask passwords, API keys, tokens in logs (replace with "***"). Implements `structured_query()` to query logs with filters (level, component, time range) using JSON query.

---

#### **6. `encryption.rs` (Rust)**

**Task:** Provide data encryption and decryption for storage and communication security.

**Purpose:** This file implements multiple encryption algorithms for various needs. Struct `Encryption` with fields: `algorithm: EncryptionAlgorithm` (AES-256-GCM, ChaCha20-Poly1305, RSA), `key: [u8; 32]` for AES-256, `iv: [u8; 12]` for GCM (12 bytes recommended). Implements `encrypt_aes_256(data: &[u8]) -> Result<Vec<u8>>` using `AES-256-GCM` from `aes-gcm` crate: generates random nonce (12 bytes) for each encryption for security, returns ciphertext + nonce + tag (16 bytes) in one vec. Implements `decrypt_aes_256(ciphertext: &[u8]) -> Result<Vec<u8>>`: extracts nonce, ciphertext, tag, verifies authentication tag, decrypts if valid. Implements `rsa_encrypt(public_key: &RsaPublicKey, data: &[u8]) -> Result<Vec<u8>>` with OAEP padding (Optimal Asymmetric Encryption Padding) for semantic security. Implements `rsa_decrypt(private_key: &RsaPrivateKey, data: &[u8]) -> Result<Vec<u8>>` to decrypt RSA-encrypted data. Implements `derive_key(password: &str, salt: &[u8]) -> [u8; 32]` using `PBKDF2` with 100,000 iterations, SHA-256 for key derivation from password. Implements `generate_secure_random()` using `rand::rngs::OsRng` to generate cryptographically secure random numbers for keys, nonces, salts.

---

#### **7. `hash_generator.rs` (Rust)**

**Task:** Provide hash functions for checksums, integrity verification, and password hashing.

**Purpose:** This file implements various hash algorithms. Struct `HashGenerator` with fields: `algorithm: HashAlgorithm`. Implements `hash_md5(data: &[u8]) -> [u8; 16]` using `md5` crate for fast checksums (not secure for security). Implements `hash_sha1(data: &[u8]) -> [u8; 20]` using `sha1` crate (not secure for security, only for legacy compatibility). Implements `hash_sha256(data: &[u8]) -> [u8; 32]` using `sha2` crate for integrity verification (secure). Implements `hash_sha3_256(data: &[u8]) -> [u8; 32]` using `sha3` crate for more modern and secure hashing. Implements `hash_bcrypt(password: &str) -> Result<String>` using `bcrypt` crate with cost factor 12 (balance between security and performance) for password hashing. Implements `hash_hmac_sha256(key: &[u8], data: &[u8]) -> [u8; 32]` for message authentication using HMAC-SHA256. Implements `hash_argon2id(password: &str) -> Result<String>` using `argon2` crate with Argon2id (memory-hard algorithm) for password hashing that is more secure than bcrypt. Implements `hash_file(path: &Path) -> Result<String>` to compute hash of a file (SHA-256) for integrity verification. Implements `verify_integrity(data: &[u8], expected_hash: &str) -> bool` to verify data integrity against expected hash.

---

#### **8. `validator_utils.rs` (Rust)**

**Task:** Provide validation functions for various data types and input sanitization.

**Purpose:** This file implements various validation functions. Struct `ValidatorUtils` with fields: `email_re: Regex` (RFC 5322 compliant email regex), `url_re: Regex` (URL validation), `ip_re: Regex` (IPv4 and IPv6), `domain_re: Regex` (domain name validation). Implements `validate_email(email: &str) -> bool` with RFC 5322 compliant regex to validate email format (local-part@domain). Implements `validate_url(url: &str) -> bool` with parsing using `url::Url` to ensure URL has scheme (http/https) and valid host. Implements `validate_ip(ip: &str) -> bool` with `std::net::IpAddr` parse for IPv4 and IPv6. Implements `validate_domain(domain: &str) -> bool` with DNS label rules (label length 1-63, total length 253, allowed characters a-z, 0-9, hyphen) and TLD validation. Implements `sanitize_input(input: &str) -> String` for HTML escaping (replace <, >, &, ", ' with HTML entities) and SQL injection prevention (escape single quotes). Implements `validate_port(port: u16)` to ensure port is within valid range (1-65535). Implements `validate_path(path: &str)` to ensure path does not contain traversal attacks (`../`). Implements `validate_content_type(content_type: &str)` to validate MIME type. Implements `validate_encoding(encoding: &str)` to validate character encoding.

---

**indonesian:**

---

## 📂 **`utils/` - UTILITY**

---

#### **1. `request_handler.rs` (Rust)**

**Tugas:** Mengelola semua HTTP request ke internet dengan session pooling, retry mechanism, dan timeout management.

**Tujuan:** File ini adalah HTTP client handler yang mengelola semua komunikasi HTTP ke internet. Implementasi `send_request(request: Request) -> Result<Response>` dengan session reuse untuk HTTP/1.1 keep-alive menggunakan `reqwest::Client` dengan connection pooling untuk menghindari overhead pembuatan koneksi baru setiap request. Implementasi `retry_with_backoff()` dengan konfigurasi: `max_attempts=3`, `initial_backoff=1s`, `multiplier=2`, `jitter=0.1` (untuk menghindari thundering herd problem). Status codes yang di-retry: 408 (Request Timeout), 429 (Too Many Requests), 500 (Internal Server Error), 502 (Bad Gateway), 503 (Service Unavailable), 504 (Gateway Timeout). Implementasi `handle_redirects()` dengan batas maksimum redirect 10 untuk mencegah infinite redirect loops. Implementasi `set_timeout()` dengan `tokio::time::timeout` per request: connection timeout 5s, read timeout 10s, write timeout 5s. Implementasi `cookie_jar()` dengan `CookieStore` yang persists across requests dalam session. Implementasi `custom_headers()` untuk menambahkan headers kustom (User-Agent, Accept-Language, Accept-Encoding) sesuai konfigurasi. Implementasi `handle_compression()` untuk mendukung gzip dan brotli compression untuk menghemat bandwidth.

---

#### **2. `proxy_manager.go` (Go)**

**Tugas:** Mengelola proxy untuk scanning stealth dengan rotation, validation, dan anonymity tracking.

**Tujuan:** File ini mengelola pool proxy untuk melakukan scanning tanpa terdeteksi. Implementasi `GetProxy() -> Proxy` dengan round-robin rotation untuk mendistribusikan beban secara merata ke semua proxy dalam pool. Implementasi `ValidateProxy(proxy Proxy) -> bool` dengan test connection ke `httpbin.org/ip` atau `api.ipify.org`, timeout 5s, untuk memastikan proxy aktif dan bekerja sebelum digunakan. Implementasi `MarkBadProxy(proxy Proxy)` untuk menghapus proxy yang gagal dari pool (jika proxy gagal 3 kali berturut-turut, dihapus permanen). Implementasi `RefreshProxies()` untuk periodically re-validate proxies (every 5 minutes) dan menghapus proxy yang mati. Proxy types supported: `HTTP`, `HTTPS`, `SOCKS5` (menggunakan `golang.org/x/net/proxy` untuk SOCKS5). Implementasi `anonymity_level_tracking()`: mengukur tingkat anonimitas setiap proxy berdasarkan response headers (X-Forwarded-For, Via) - transparent (mengirim IP asli), anonymous (menyembunyikan IP), elite (menyembunyikan semua proxy traces). Implementasi `geolocation_routing()`: memilih proxy berdasarkan geolokasi target untuk mengurangi latency.

---

#### **3. `rate_limiter.rs` (Rust)**

**Tugas:** Mengelola rate limiting menggunakan token bucket algorithm untuk mencegah abuse dan menghindari detection.

**Tujuan:** File ini mengimplementasikan token bucket algorithm dengan atomic operations untuk thread safety. Struct `RateLimiter` dengan fields: `tokens: AtomicUsize` (token saat ini), `capacity: usize` (kapasitas maksimum), `fill_rate: usize` (jumlah token per detik), `last_refill: AtomicUsize` (timestamp refill terakhir). `Allow()` method: atomically decrement tokens (menggunakan `fetch_sub`), jika tokens > 0 return true (allow request), jika tokens == 0 return false (rate limited). `Refill()` called every second: `tokens = min(capacity, tokens + fill_rate)`. Implementasi `PerDomainLimiter` dengan `DashMap<String, RateLimiter>` untuk memberikan rate limit yang berbeda untuk setiap domain target (contoh: 10 req/detik untuk domain, 50 req/detik untuk IP, 100 req/detik untuk URL). Implementasi `SlidingWindowCounter` untuk rate limiting berbasis waktu: store timestamps in `VecDeque`, check count in last N seconds (contoh: 100 requests in last 60 seconds). Implementasi `adaptive_rate_limiting()` yang menyesuaikan rate limit berdasarkan response time dan error rate: jika banyak error 429 (Too Many Requests), turunkan rate; jika response time cepat, naikkan rate. Implementasi `burst_capacity()` untuk mengakomodasi lonjakan request sesaat (misal: 10 requests in 1 second).

---

#### **4. `concurrency_manager.rs` (Rust)**

**Tugas:** Mengelola konkurensi dengan async/await, thread pooling, dan semaphore-based resource control.

**Tujuan:** File ini mengelola semua operasi konkuren dalam sistem. Struct `ConcurrencyManager` dengan fields: `runtime: tokio::Runtime` (Tokio runtime untuk async operations), `thread_pool: ThreadPool` (Rayon thread pool untuk blocking operations), `semaphore: Arc<Semaphore>` (untuk membatasi akses ke resource terbatas), `task_counter: AtomicUsize` (untuk tracking jumlah task aktif). Implementasi `spawn_task<F: Future>(future: F)` dengan `tokio::spawn` untuk async tasks yang non-blocking. Implementasi `run_in_thread<T: Send + 'static>(job: impl FnOnce() -> T + Send + 'static)` untuk blocking operations di thread pool (menggunakan `rayon::spawn()`). Implementasi `semaphore_control()`: `acquire()` untuk blocking task (menunggu jika tidak ada slot available), `release()` untuk signal completion (membebaskan slot). Implementasi `graceful_shutdown()`: `signal::ctrl_c()` untuk menangkap SIGINT, wait for tasks to complete dengan timeout 30s, force shutdown jika timeout (cancel remaining tasks). Implementasi `task_priority()`: high priority tasks (scan commands) mendapat resource terlebih dahulu, low priority tasks (report generation) ditunda jika resource terbatas. Implementasi `load_shedding()`: jika task queue melebihi threshold (1000 tasks), reject new tasks dengan error "system overloaded".

---

#### **5. `logging_system.rs` (Rust)**

**Tugas:** Menyediakan sistem logging terstruktur dengan multiple outputs dan log levels.

**Tujuan:** File ini mengelola semua logging dalam sistem. Struct `LoggingSystem` dengan fields: `logger: Logger` (main logger), `file_appender: RollingFileAppender` (file logging), `remote_appender: Option<RemoteAppender>` (remote logging). Implementasi `log(level: LogLevel, message: &str, context: &Context)` dengan structured JSON format: `{timestamp, level, message, context: {component, request_id, user_id, session_id, ip, duration}}`. Log levels: DEBUG (detail debugging), INFO (informasi umum), WARN (peringatan), ERROR (error yang bisa dipulihkan), FATAL (error kritis yang menyebabkan shutdown). Implementasi `RollingFileAppender`: rotate when file size > 10MB, keep 5 files (total 50MB), compress old files with `gzip` untuk menghemat disk space. Implementasi `RemoteAppender`: send logs to remote server via UDP (untuk performa) atau HTTP (untuk reliability), with batching (flush every 100 entries atau every 5 seconds). Implementasi `Context::new()` untuk create log context dengan automatic `span` dari `tracing` untuk correlation across requests. Implementasi `mask_sensitive_data()` untuk masking password, API keys, tokens dalam log (replace with "***"). Implementasi `structured_query()` untuk query logs dengan filter (level, component, time range) menggunakan JSON query.

---

#### **6. `encryption.rs` (Rust)**

**Tugas:** Menyediakan enkripsi dan dekripsi data untuk keamanan storage dan communication.

**Tujuan:** File ini mengimplementasikan multiple encryption algorithms untuk berbagai kebutuhan. Struct `Encryption` dengan fields: `algorithm: EncryptionAlgorithm` (AES-256-GCM, ChaCha20-Poly1305, RSA), `key: [u8; 32]` untuk AES-256, `iv: [u8; 12]` untuk GCM (12 bytes recommended). Implementasi `encrypt_aes_256(data: &[u8]) -> Result<Vec<u8>>` menggunakan `AES-256-GCM` from `aes-gcm` crate: generate random nonce (12 bytes) untuk setiap encryption untuk keamanan, return ciphertext + nonce + tag (16 bytes) dalam satu vec. Implementasi `decrypt_aes_256(ciphertext: &[u8]) -> Result<Vec<u8>>`: extract nonce, ciphertext, tag, verify authentication tag, decrypt if valid. Implementasi `rsa_encrypt(public_key: &RsaPublicKey, data: &[u8]) -> Result<Vec<u8>>` dengan OAEP padding (Optimal Asymmetric Encryption Padding) untuk keamanan semantic. Implementasi `rsa_decrypt(private_key: &RsaPrivateKey, data: &[u8]) -> Result<Vec<u8>>` untuk mendekripsi data yang dienkripsi dengan RSA. Implementasi `derive_key(password: &str, salt: &[u8]) -> [u8; 32]` menggunakan `PBKDF2` dengan 100,000 iterations, SHA-256 untuk key derivation from password. Implementasi `generate_secure_random()` menggunakan `rand::rngs::OsRng` untuk generate cryptographically secure random numbers untuk keys, nonces, salts.

---

#### **7. `hash_generator.rs` (Rust)**

**Tugas:** Menyediakan fungsi hash untuk checksum, integrity verification, dan password hashing.

**Tujuan:** File ini mengimplementasikan berbagai algoritma hash. Struct `HashGenerator` dengan fields: `algorithm: HashAlgorithm`. Implementasi `hash_md5(data: &[u8]) -> [u8; 16]` menggunakan `md5` crate untuk checksum cepat (tidak aman untuk security). Implementasi `hash_sha1(data: &[u8]) -> [u8; 20]` menggunakan `sha1` crate (tidak aman untuk security, hanya untuk legacy compatibility). Implementasi `hash_sha256(data: &[u8]) -> [u8; 32]` menggunakan `sha2` crate untuk integrity verification (secure). Implementasi `hash_sha3_256(data: &[u8]) -> [u8; 32]` menggunakan `sha3` crate untuk hashing yang lebih modern dan aman. Implementasi `hash_bcrypt(password: &str) -> Result<String>` menggunakan `bcrypt` crate dengan cost factor 12 (balance antara security dan performance) untuk password hashing. Implementasi `hash_hmac_sha256(key: &[u8], data: &[u8]) -> [u8; 32]` untuk message authentication menggunakan HMAC-SHA256. Implementasi `hash_argon2id(password: &str) -> Result<String>` menggunakan `argon2` crate dengan Argon2id (memory-hard algorithm) untuk password hashing yang lebih aman dari bcrypt. Implementasi `hash_file(path: &Path) -> Result<String>` untuk menghitung hash dari file (SHA-256) untuk integrity verification. Implementasi `verify_integrity(data: &[u8], expected_hash: &str) -> bool` untuk memverifikasi integritas data dengan expected hash.

---

#### **8. `validator_utils.rs` (Rust)**

**Tugas:** Menyediakan fungsi validasi untuk berbagai tipe data dan input sanitization.

**Tujuan:** File ini mengimplementasikan berbagai fungsi validasi. Struct `ValidatorUtils` dengan fields: `email_re: Regex` (RFC 5322 compliant email regex), `url_re: Regex` (URL validation), `ip_re: Regex` (IPv4 dan IPv6), `domain_re: Regex` (domain name validation). Implementasi `validate_email(email: &str) -> bool` dengan RFC 5322 compliant regex untuk memvalidasi format email (local-part@domain). Implementasi `validate_url(url: &str) -> bool` dengan parsing menggunakan `url::Url` untuk memastikan URL memiliki scheme (http/https) dan host yang valid. Implementasi `validate_ip(ip: &str) -> bool` dengan `std::net::IpAddr` parse untuk IPv4 dan IPv6. Implementasi `validate_domain(domain: &str) -> bool` dengan DNS label rules (panjang label 1-63, total panjang 253, karakter yang diizinkan a-z, 0-9, hyphen) dan TLD validation. Implementasi `sanitize_input(input: &str) -> String` untuk HTML escaping (replace <, >, &, ", ' dengan HTML entities) dan SQL injection prevention (escape single quotes). Implementasi `validate_port(port: u16)` untuk memastikan port dalam range yang valid (1-65535). Implementasi `validate_path(path: &str)` untuk memastikan path tidak mengandung traversal attacks (`../`). Implementasi `validate_content_type(content_type: &str)` untuk memvalidasi MIME type. Implementasi `validate_encoding(encoding: &str)` untuk memvalidasi character encoding.

---

**config folder explanation:**

---

## 📂 **`config/` - CONFIGURATION**

---

#### **1. `settings.rs` (Rust)**

**Task:** Manage the main system configuration with hierarchical loading from multiple sources.

**Purpose:** This file is the configuration center that loads all global system settings. Struct `Settings` with fields: `database: DatabaseConfig` (host, port, username, password, database name, connection pool size), `redis: RedisConfig` (host, port, password, database index), `api_keys: ApiKeys` (API keys for all integration services), `scanning: ScanningConfig` (default timeout, threads, retry count, profiles), `logging: LoggingConfig` (level, format, rotation, remote forwarding), `environment: Environment` (Development, Testing, Production). Implements `load()` using `config` crate with hierarchy: default values (hardcoded in code) -> config file (`config/settings.{env}.toml`) -> environment variables (prefix `IWS_`). Environment variable mapping: `IWS_DATABASE_URL` -> `database.url`, `IWS_LOG_LEVEL` -> `logging.level`, `IWS_MAX_THREADS` -> `scanning.max_threads`. Implements `validate()` with `validator` crate to check required fields (database.url cannot be empty, api_keys must be filled for enabled services). Implements `reload()` to reload configuration without restarting the application (listens to SIGHUP signal). Implements `export()` to export current configuration to file for debugging.

---

#### **2. `apikeys_template.py` (Python)**

**Task:** Provide template and management for API keys from various third-party services.

**Purpose:** This file is a Python template for storing all API keys required by the system. Class `ApiKeys` with fields: `SHODAN_API_KEY: str` (for Shodan network intelligence), `CENSYS_API_ID: str` and `CENSYS_API_SECRET: str` (for Censys internet scanning), `VIRUSTOTAL_API_KEY: str` (for VirusTotal malware scanning), `ALIENVAULT_API_KEY: str` (for AlienVault OTX threat intelligence), `URLSCAN_API_KEY: str` (for URLScan behavior analysis), `SECURITYTRAILS_API_KEY: str` (for SecurityTrails DNS history), `GREYHAT_API_KEY: str` (for GreyHat Warfare fast access), `GOOGLE_SAFE_BROWSING_KEY: str` (for Google Safe Browsing), `WOT_API_KEY: str` (for Web of Trust). Implements `load_from_env()` which reads from environment variables with prefix `IWS_` (e.g., `IWS_SHODAN_API_KEY`). Implements `validate_keys()` to check key format (length, charset) before use - for example: Shodan API key is 32 alphanumeric characters, VirusTotal key is 64 characters. Implements `get_active_services()` to return list of services that have valid API keys (to determine which services to enable). Implements `mask_keys()` to mask keys in logs (show only first and last 4 characters).

---

#### **3. `user_agents.rs` (Rust)**

**Task:** Provide User-Agent string database for rotation during scanning to avoid detection.

**Purpose:** This file manages the User-Agent database for rotation. Struct `UserAgents` with fields: `agents: Vec<String>` (list of User-Agent strings), `current_index: AtomicUsize` (index for round-robin). Implements `load()`: loads from `user_agents.txt` file (users can add custom User-Agents), falls back to built-in list if file not found. Built-in list includes: Chrome 120-124 (Windows, Mac, Linux), Firefox 120-123 (Windows, Mac, Linux), Safari 17.2 (Mac, iOS), Edge 120-123 (Windows), Opera 106-107 (Windows, Mac), and mobile User-Agents (Android, iOS). Implements `get_random_agent() -> String` with random selection using `rand::thread_rng()` for random distribution. Implements `get_next_agent() -> String` with round-robin rotation for even distribution (each request uses a different User-Agent in rotation). Implements `get_agent_by_platform(platform: &str) -> String` to get platform-specific User-Agent (Windows, Mac, Linux, Android, iOS). Implements `get_agent_by_browser(browser: &str) -> String` to get browser-specific User-Agent (Chrome, Firefox, Safari, Edge, Opera). Implements `refresh()` to reload User-Agent list from file without restart.

---

#### **4. `scanning_profiles.go` (Go)**

**Task:** Define and manage various scanning profiles for different needs.

**Purpose:** This file manages different scanning profiles. Struct `ScanningProfiles` with fields: `Profiles map[string]Profile`. Profile fields: `Threads int` (number of parallel threads), `Timeout int` (timeout per request in seconds), `Delay int` (delay between requests in milliseconds), `MaxPages int` (maximum pages to scan), `FollowRedirects bool` (whether to follow redirects), `RespectRobots bool` (whether to respect robots.txt). Implements `GetProfile(name string) -> Profile` with predefined profiles: `"aggressive"` (Threads=100, Timeout=10, Delay=0, MaxPages=1000, FollowRedirects=true, RespectRobots=false) for fast scanning with high resources. `"moderate"` (Threads=50, Timeout=15, Delay=100, MaxPages=500, FollowRedirects=true, RespectRobots=true) for balanced scanning. `"stealth"` (Threads=10, Timeout=30, Delay=1000, MaxPages=100, FollowRedirects=false, RespectRobots=true) for stealth scanning that is difficult to detect. `"comprehensive"` (Threads=30, Timeout=20, Delay=200, MaxPages=2000, FollowRedirects=true, RespectRobots=false) for in-depth scanning. Implements `CustomProfile` with override fields to create custom profiles. Implements `ValidateProfile()` to ensure profile has valid values (Threads > 0, Timeout > 0, Delay >= 0, MaxPages > 0). Implements `GetProfileRecommendation()` to provide profile recommendation based on target (e.g., banking site -> stealth, testing site -> aggressive).

---

#### **5. `webhook_configs.rs` (Rust)**

**Task:** Configure webhooks for integration with notification platforms.

**Purpose:** This file manages webhook configuration for notifications. Struct `WebhookConfig` with fields: `slack: Option<SlackWebhook>` (Slack configuration), `discord: Option<DiscordWebhook>` (Discord configuration), `telegram: Option<TelegramWebhook>` (Telegram configuration), `custom: Vec<CustomWebhook>` (custom webhook URLs). `SlackWebhook`: `url: String` (webhook URL), `channel: String` (channel name), `username: String` (bot username), `icon_emoji: String` (emoji icon). `DiscordWebhook`: `url: String`, `username: String` (bot username), `avatar_url: String` (avatar URL), `tts: bool` (text-to-speech). `TelegramWebhook`: `bot_token: String` (bot token), `chat_id: String` (chat ID), `parse_mode: String` (HTML/Markdown). `CustomWebhook`: `name: String`, `url: String`, `method: String` (GET/POST), `headers: HashMap<String, String>`, `body_template: String`. Implements `send_webhook(event: WebhookEvent)` with JSON payload, retry mechanism (`max_attempts=3`), timeout 5s. Implements `format_payload()` to format payload according to platform (Slack uses `{"text": "message"}`, Discord uses `{"content": "message"}`, Telegram uses `{"chat_id": id, "text": "message"}`). Implements `test_webhook()` to send test message to all configured webhooks. Implements `get_enabled_webhooks()` to get list of active webhooks.

---

**indonesian:**

---

## 📂 **`config/` - KONFIGURASI**

---

#### **1. `settings.rs` (Rust)**

**Tugas:** Mengelola konfigurasi utama sistem dengan hierarchical loading dari multiple sources.

**Tujuan:** File ini adalah pusat konfigurasi yang memuat semua pengaturan global sistem. Struct `Settings` dengan fields: `database: DatabaseConfig` (host, port, username, password, database name, connection pool size), `redis: RedisConfig` (host, port, password, database index), `api_keys: ApiKeys` (API keys untuk semua layanan integrasi), `scanning: ScanningConfig` (default timeout, threads, retry count, profiles), `logging: LoggingConfig` (level, format, rotation, remote forwarding), `environment: Environment` (Development, Testing, Production). Implementasi `load()` menggunakan `config` crate dengan hierarki: default values (hardcoded in code) -> config file (`config/settings.{env}.toml`) -> environment variables (prefix `IWS_`). Environment variable mapping: `IWS_DATABASE_URL` -> `database.url`, `IWS_LOG_LEVEL` -> `logging.level`, `IWS_MAX_THREADS` -> `scanning.max_threads`. Implementasi `validate()` dengan `validator` crate untuk checking required fields (database.url tidak boleh kosong, api_keys harus diisi untuk services yang diaktifkan). Implementasi `reload()` untuk reload konfigurasi tanpa restart aplikasi (mendengarkan SIGHUP signal). Implementasi `export()` untuk mengekspor konfigurasi saat ini ke file untuk debugging.

---

#### **2. `apikeys_template.py` (Python)**

**Tugas:** Menyediakan template dan manajemen untuk API keys dari berbagai layanan pihak ketiga.

**Tujuan:** File ini adalah template Python untuk menyimpan semua API keys yang diperlukan sistem. Class `ApiKeys` dengan fields: `SHODAN_API_KEY: str` (untuk Shodan network intelligence), `CENSYS_API_ID: str` dan `CENSYS_API_SECRET: str` (untuk Censys internet scanning), `VIRUSTOTAL_API_KEY: str` (untuk VirusTotal malware scanning), `ALIENVAULT_API_KEY: str` (untuk AlienVault OTX threat intelligence), `URLSCAN_API_KEY: str` (untuk URLScan behavior analysis), `SECURITYTRAILS_API_KEY: str` (untuk SecurityTrails DNS history), `GREYHAT_API_KEY: str` (untuk GreyHat Warfare fast access), `GOOGLE_SAFE_BROWSING_KEY: str` (untuk Google Safe Browsing), `WOT_API_KEY: str` (untuk Web of Trust). Implementasi `load_from_env()` yang membaca dari environment variables dengan prefix `IWS_` (contoh: `IWS_SHODAN_API_KEY`). Implementasi `validate_keys()` untuk memeriksa format key (panjang, charset) sebelum digunakan - contoh: Shodan API key panjang 32 karakter alfanumerik, VirusTotal key panjang 64 karakter. Implementasi `get_active_services()` untuk mengembalikan list services yang memiliki API key valid (untuk menentukan service mana yang akan diaktifkan). Implementasi `mask_keys()` untuk masking keys dalam log (tampilkan hanya 4 karakter pertama dan terakhir).

---

#### **3. `user_agents.rs` (Rust)**

**Tugas:** Menyediakan database User-Agent strings untuk rotasi saat scanning untuk menghindari deteksi.

**Tujuan:** File ini mengelola database User-Agent untuk rotasi. Struct `UserAgents` dengan fields: `agents: Vec<String>` (daftar User-Agent strings), `current_index: AtomicUsize` (index untuk round-robin). Implementasi `load()`: load from `user_agents.txt` file (user dapat menambahkan custom User-Agents), fallback to built-in list if file not found. Built-in list mencakup: Chrome 120-124 (Windows, Mac, Linux), Firefox 120-123 (Windows, Mac, Linux), Safari 17.2 (Mac, iOS), Edge 120-123 (Windows), Opera 106-107 (Windows, Mac), dan mobile User-Agents (Android, iOS). Implementasi `get_random_agent() -> String` dengan random selection menggunakan `rand::thread_rng()` untuk distribusi acak. Implementasi `get_next_agent() -> String` dengan round-robin rotation untuk distribusi yang merata (setiap request menggunakan User-Agent berbeda secara bergiliran). Implementasi `get_agent_by_platform(platform: &str) -> String` untuk mendapatkan User-Agent spesifik platform (Windows, Mac, Linux, Android, iOS). Implementasi `get_agent_by_browser(browser: &str) -> String` untuk mendapatkan User-Agent spesifik browser (Chrome, Firefox, Safari, Edge, Opera). Implementasi `refresh()` untuk reload User-Agent list dari file tanpa restart.

---

#### **4. `scanning_profiles.go` (Go)**

**Tugas:** Mendefinisikan dan mengelola berbagai profil scanning untuk berbagai kebutuhan.

**Tujuan:** File ini mengelola profil scanning yang berbeda-beda. Struct `ScanningProfiles` dengan fields: `Profiles map[string]Profile`. Profile fields: `Threads int` (jumlah thread parallel), `Timeout int` (timeout per request dalam detik), `Delay int` (delay antar request dalam milidetik), `MaxPages int` (maksimum halaman yang discan), `FollowRedirects bool` (apakah mengikuti redirects), `RespectRobots bool` (apakah menghormati robots.txt). Implementasi `GetProfile(name string) -> Profile` dengan predefined profiles: `"aggressive"` (Threads=100, Timeout=10, Delay=0, MaxPages=1000, FollowRedirects=true, RespectRobots=false) untuk scanning cepat dengan resource tinggi. `"moderate"` (Threads=50, Timeout=15, Delay=100, MaxPages=500, FollowRedirects=true, RespectRobots=true) untuk scanning balanced. `"stealth"` (Threads=10, Timeout=30, Delay=1000, MaxPages=100, FollowRedirects=false, RespectRobots=true) untuk scanning stealth yang sulit dideteksi. `"comprehensive"` (Threads=30, Timeout=20, Delay=200, MaxPages=2000, FollowRedirects=true, RespectRobots=false) untuk scanning mendalam. Implementasi `CustomProfile` dengan fields override untuk membuat profile kustom. Implementasi `ValidateProfile()` untuk memastikan profile memiliki nilai yang valid (Threads > 0, Timeout > 0, Delay >= 0, MaxPages > 0). Implementasi `GetProfileRecommendation()` untuk memberikan rekomendasi profile berdasarkan target (contoh: banking site -> stealth, testing site -> aggressive).

---

#### **5. `webhook_configs.rs` (Rust)**

**Tugas:** Mengkonfigurasi webhook untuk integrasi dengan platform notifikasi.

**Tujuan:** File ini mengelola konfigurasi webhook untuk notifikasi. Struct `WebhookConfig` dengan fields: `slack: Option<SlackWebhook>` (konfigurasi Slack), `discord: Option<DiscordWebhook>` (konfigurasi Discord), `telegram: Option<TelegramWebhook>` (konfigurasi Telegram), `custom: Vec<CustomWebhook>` (custom webhook URLs). `SlackWebhook`: `url: String` (webhook URL), `channel: String` (channel name), `username: String` (bot username), `icon_emoji: String` (emoji icon). `DiscordWebhook`: `url: String`, `username: String` (bot username), `avatar_url: String` (avatar URL), `tts: bool` (text-to-speech). `TelegramWebhook`: `bot_token: String` (bot token), `chat_id: String` (chat ID), `parse_mode: String` (HTML/Markdown). `CustomWebhook`: `name: String`, `url: String`, `method: String` (GET/POST), `headers: HashMap<String, String>`, `body_template: String`. Implementasi `send_webhook(event: WebhookEvent)` dengan JSON payload, retry mechanism (`max_attempts=3`), timeout 5s. Implementasi `format_payload()` untuk memformat payload sesuai dengan platform (Slack uses `{"text": "message"}`, Discord uses `{"content": "message"}`, Telegram uses `{"chat_id": id, "text": "message"}`). Implementasi `test_webhook()` untuk mengirim test message ke semua webhook yang dikonfigurasi. Implementasi `get_enabled_webhooks()` untuk mendapatkan list webhook yang aktif.

---

**database folder explanation:**

---

## 📂 **`database/` - DATABASE**

---

#### **1. `schema.sql` (SQL)**

**Task:** Define the complete database structure with all tables, relationships, indexes, and constraints.

**Purpose:** This file contains Data Definition Language (DDL) to create all tables required by the system. Defines `users` table: `id UUID PRIMARY KEY DEFAULT gen_random_uuid()`, `username VARCHAR(255) UNIQUE NOT NULL`, `password_hash VARCHAR(255) NOT NULL`, `email VARCHAR(255) UNIQUE NOT NULL`, `role ENUM('admin','user','guest') DEFAULT 'user'`, `created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP`, `last_login TIMESTAMP`, `is_active BOOLEAN DEFAULT true`. Defines `scan_results` table: `id UUID PRIMARY KEY DEFAULT gen_random_uuid()`, `user_id UUID REFERENCES users(id) ON DELETE CASCADE`, `target_url VARCHAR(2048) NOT NULL`, `scan_profile VARCHAR(50)`, `started_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP`, `completed_at TIMESTAMP`, `status ENUM('pending','active','completed','failed','cancelled') DEFAULT 'pending'`, `result JSONB`, `summary TEXT`, `risk_score DECIMAL(5,2)`. Defines `vulnerabilities` table: `id UUID PRIMARY KEY DEFAULT gen_random_uuid()`, `scan_id UUID REFERENCES scan_results(id) ON DELETE CASCADE`, `cve_id VARCHAR(50)`, `title VARCHAR(255)`, `description TEXT`, `severity ENUM('critical','high','medium','low','info')`, `cvss_score DECIMAL(3,1)`, `cvss_vector VARCHAR(100)`, `affected_component VARCHAR(255)`, `remediation TEXT`, `status ENUM('open','in_progress','fixed','won_fix') DEFAULT 'open'`, `discovered_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP`, `fixed_at TIMESTAMP`. Defines `agent_states` table: `id SERIAL PRIMARY KEY`, `agent_name VARCHAR(100) NOT NULL`, `state JSONB NOT NULL`, `updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP`. Defines `configuration` table: `id SERIAL PRIMARY KEY`, `key VARCHAR(255) UNIQUE NOT NULL`, `value JSONB NOT NULL`, `updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP`. Indexes for query optimization: `scan_results(user_id, started_at)`, `scan_results(status, started_at)`, `vulnerabilities(scan_id, severity)`, `vulnerabilities(cve_id)`, `agent_states(agent_name)`. Foreign key constraints with ON DELETE CASCADE to maintain referential integrity.

---

#### **2. `connection_pool.rs` (Rust)**

**Task:** Manage database connections with connection pooling for optimal performance.

**Purpose:** This file implements connection pooling for the database using `deadpool_postgres` crate. Struct `ConnectionPool` with fields: `pool: Arc<Pool<Postgres>>` (thread-safe connection pool), `config: PoolConfig` (pool configuration). Implements `new(config: PoolConfig)` which creates a connection pool with configuration: `max_size=20` (maximum 20 concurrent connections), `min_idle=5` (minimum 5 idle connections), `timeout=30s` (timeout to acquire connection), `idle_timeout=10m` (idle connections will be closed after 10 minutes). Implements `get_connection() -> Result<PooledConnection>` with `pool.get()` async that returns a connection from the pool or creates a new one if available. Implements `close_all()` for graceful shutdown: acquire all connections (loop until all connections are taken), close them properly. Implements `health_check()`: test query `SELECT 1`, returns true if successful, false if failed. Implements `monitor_pool()` to monitor pool metrics: active connections, idle connections, total connections, waiting requests, and logs warning if pool is near full (>80% utilization). Implements `reset_pool()` to reset pool if connection issues occur (close all connections and create new ones).

---

#### **3. `orm_models.py` (Python)**

**Task:** Define Object-Relational Mapping (ORM) models for database interaction using SQLAlchemy.

**Purpose:** This file uses SQLAlchemy ORM to define database models as Python classes. Class `User(Base)`: `__tablename__ = 'users'`, fields: `id` (UUID), `username` (String), `password_hash` (String), `email` (String), `role` (Enum), `created_at` (DateTime), `last_login` (DateTime), `is_active` (Boolean). Class `ScanResult(Base)`: `__tablename__ = 'scan_results'`, fields: `id` (UUID), `user_id` (UUID with relationship to User), `target_url` (String), `scan_profile` (String), `started_at` (DateTime), `completed_at` (DateTime), `status` (Enum), `result` (JSON), `summary` (String), `risk_score` (Float). Class `Vulnerability(Base)`: `__tablename__ = 'vulnerabilities'`, fields: `id` (UUID), `scan_id` (UUID with relationship to ScanResult), `cve_id` (String), `title` (String), `description` (Text), `severity` (Enum), `cvss_score` (Float), `cvss_vector` (String), `affected_component` (String), `remediation` (Text), `status` (Enum), `discovered_at` (DateTime), `fixed_at` (DateTime). Implements relationships: `ScanResult.vulnerabilities = relationship("Vulnerability", back_populates="scan", cascade="all, delete-orphan")`. Implements `to_dict()` method for serialization to dictionary for API responses. Implements `save()` and `delete()` with session management and transaction handling. Implements `query()` methods for common queries (get_by_id, get_by_user, get_by_status, get_by_date_range). Implements `__repr__()` for debugging representation.

---

#### **4. `query_builder.rs` (Rust)**

**Task:** Build SQL queries dynamically and safely to prevent SQL injection.

**Purpose:** This file implements a query builder pattern for safe dynamic SQL construction. Struct `QueryBuilder` with fields: `query: String` (query being built), `params: Vec<Value>` (parameters for parameterized query). Methods: `select(columns: &[&str]) -> Self` (specify columns to select), `from(table: &str) -> Self` (specify table), `where_cond(condition: &str) -> Self` (add WHERE condition), `and(condition: &str) -> Self` (add AND condition), `or(condition: &str) -> Self` (add OR condition), `order_by(column: &str, asc: bool) -> Self` (add ORDER BY), `limit(n: u32) -> Self` (add LIMIT), `offset(n: u32) -> Self` (add OFFSET), `join(join_type: &str, table: &str, on: &str) -> Self` (add JOIN). Implements `build() -> (String, Vec<Value>)` which returns query string and parameters for parameterized query. Implements parameterized queries using `$1, $2, $3` placeholders for PostgreSQL to prevent SQL injection (parameter values are not escaped directly into query). Implements `build_count()` to build COUNT query from existing query. Implements `build_paginated()` to build query with pagination (LIMIT + OFFSET). Implements `validate()` to validate query before execution (ensures table and column names are valid, no SQL injection patterns).

---

#### **5. `migrations.rs` (Rust)**

**Task:** Manage database migrations for schema version control and updates without data loss.

**Purpose:** This file manages version control for the database schema. Struct `MigrationManager` with fields: `conn: PgConnection` (database connection), `migrations_dir: PathBuf` (directory containing migration files). Implements `run_migrations()`: list all files in `migrations/` with pattern `{timestamp}_{name}.sql` (e.g., `20240101000000_initial_schema.sql`, `20240115000000_add_cve_table.sql`), sort by timestamp, execute in order. Tracks already executed migrations in `migrations` table (id, migration_name, executed_at, checksum) to prevent re-execution. Implements `rollback(last_n: usize)`: rollback N migrations by executing `down` section from migration file (if exists). Migration file format: `-- +migrate Up` for section executed during upgrade, `-- +migrate Down` for section executed during rollback. Implements `create_migration(name: &str)` to generate new migration file with timestamp and template. Implements `status()` to display current migration status: executed and pending migrations. Implements `validate()` to verify migration integrity (checksum match, no missing migrations). Implements `backup_before_migration()` to automatically backup database before running migrations.

---

**indonesian:**

---

## 📂 **`database/` - DATABASE**

---

#### **1. `schema.sql` (SQL)**

**Tugas:** Mendefinisikan struktur database lengkap dengan semua tabel, relasi, indeks, dan constraint.

**Tujuan:** File ini berisi Data Definition Language (DDL) untuk membuat semua tabel yang diperlukan sistem. Mendefinisikan `users` table: `id UUID PRIMARY KEY DEFAULT gen_random_uuid()`, `username VARCHAR(255) UNIQUE NOT NULL`, `password_hash VARCHAR(255) NOT NULL`, `email VARCHAR(255) UNIQUE NOT NULL`, `role ENUM('admin','user','guest') DEFAULT 'user'`, `created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP`, `last_login TIMESTAMP`, `is_active BOOLEAN DEFAULT true`. Mendefinisikan `scan_results` table: `id UUID PRIMARY KEY DEFAULT gen_random_uuid()`, `user_id UUID REFERENCES users(id) ON DELETE CASCADE`, `target_url VARCHAR(2048) NOT NULL`, `scan_profile VARCHAR(50)`, `started_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP`, `completed_at TIMESTAMP`, `status ENUM('pending','active','completed','failed','cancelled') DEFAULT 'pending'`, `result JSONB`, `summary TEXT`, `risk_score DECIMAL(5,2)`. Mendefinisikan `vulnerabilities` table: `id UUID PRIMARY KEY DEFAULT gen_random_uuid()`, `scan_id UUID REFERENCES scan_results(id) ON DELETE CASCADE`, `cve_id VARCHAR(50)`, `title VARCHAR(255)`, `description TEXT`, `severity ENUM('critical','high','medium','low','info')`, `cvss_score DECIMAL(3,1)`, `cvss_vector VARCHAR(100)`, `affected_component VARCHAR(255)`, `remediation TEXT`, `status ENUM('open','in_progress','fixed','won_fix') DEFAULT 'open'`, `discovered_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP`, `fixed_at TIMESTAMP`. Mendefinisikan `agent_states` table: `id SERIAL PRIMARY KEY`, `agent_name VARCHAR(100) NOT NULL`, `state JSONB NOT NULL`, `updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP`. Mendefinisikan `configuration` table: `id SERIAL PRIMARY KEY`, `key VARCHAR(255) UNIQUE NOT NULL`, `value JSONB NOT NULL`, `updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP`. Indeks untuk optimasi query: `scan_results(user_id, started_at)`, `scan_results(status, started_at)`, `vulnerabilities(scan_id, severity)`, `vulnerabilities(cve_id)`, `agent_states(agent_name)`. Foreign key constraints dengan ON DELETE CASCADE untuk menjaga referential integrity.

---

#### **2. `connection_pool.rs` (Rust)**

**Tugas:** Mengelola koneksi ke database dengan connection pooling untuk performa optimal.

**Tujuan:** File ini mengimplementasikan connection pooling untuk database menggunakan `deadpool_postgres` crate. Struct `ConnectionPool` dengan fields: `pool: Arc<Pool<Postgres>>` (thread-safe connection pool), `config: PoolConfig` (konfigurasi pool). Implementasi `new(config: PoolConfig)` yang membuat connection pool dengan konfigurasi: `max_size=20` (maksimum 20 koneksi concurrent), `min_idle=5` (minimum 5 koneksi idle), `timeout=30s` (timeout untuk mendapatkan koneksi), `idle_timeout=10m` (koneksi idle akan ditutup setelah 10 menit). Implementasi `get_connection() -> Result<PooledConnection>` dengan `pool.get()` async yang mengembalikan koneksi dari pool atau membuat baru jika tersedia. Implementasi `close_all()` untuk graceful shutdown: acquire semua connections (menggunakan loop sampai semua connections diambil), close them properly. Implementasi `health_check()`: test query `SELECT 1`, return true jika successful, false jika failed. Implementasi `monitor_pool()` untuk memonitoring pool metrics: active connections, idle connections, total connections, waiting requests, dan log peringatan jika pool hampir penuh (>80% utilization). Implementasi `reset_pool()` untuk mereset pool jika terjadi masalah koneksi (close all connections dan buat baru).

---

#### **3. `orm_models.py` (Python)**

**Tugas:** Mendefinisikan Object-Relational Mapping (ORM) models untuk interaksi dengan database menggunakan SQLAlchemy.

**Tujuan:** File ini menggunakan SQLAlchemy ORM untuk mendefinisikan model-model database sebagai class Python. Class `User(Base)`: `__tablename__ = 'users'`, fields: `id` (UUID), `username` (String), `password_hash` (String), `email` (String), `role` (Enum), `created_at` (DateTime), `last_login` (DateTime), `is_active` (Boolean). Class `ScanResult(Base)`: `__tablename__ = 'scan_results'`, fields: `id` (UUID), `user_id` (UUID dengan relationship ke User), `target_url` (String), `scan_profile` (String), `started_at` (DateTime), `completed_at` (DateTime), `status` (Enum), `result` (JSON), `summary` (String), `risk_score` (Float). Class `Vulnerability(Base)`: `__tablename__ = 'vulnerabilities'`, fields: `id` (UUID), `scan_id` (UUID dengan relationship ke ScanResult), `cve_id` (String), `title` (String), `description` (Text), `severity` (Enum), `cvss_score` (Float), `cvss_vector` (String), `affected_component` (String), `remediation` (Text), `status` (Enum), `discovered_at` (DateTime), `fixed_at` (DateTime). Implementasi relationships: `ScanResult.vulnerabilities = relationship("Vulnerability", back_populates="scan", cascade="all, delete-orphan")`. Implementasi `to_dict()` method untuk serialization ke dictionary untuk API responses. Implementasi `save()` dan `delete()` dengan session management dan transaction handling. Implementasi `query()` methods untuk common queries (get_by_id, get_by_user, get_by_status, get_by_date_range). Implementasi `__repr__()` untuk debugging representation.

---

#### **4. `query_builder.rs` (Rust)**

**Tugas:** Membangun query SQL secara dinamis dengan aman untuk mencegah SQL injection.

**Tujuan:** File ini mengimplementasikan query builder pattern untuk konstruksi SQL dinamis yang aman. Struct `QueryBuilder` dengan fields: `query: String` (query yang sedang dibangun), `params: Vec<Value>` (parameter untuk parameterized query). Methods: `select(columns: &[&str]) -> Self` (tentukan kolom yang akan di-select), `from(table: &str) -> Self` (tentukan table), `where_cond(condition: &str) -> Self` (tambahkan WHERE condition), `and(condition: &str) -> Self` (tambahkan AND condition), `or(condition: &str) -> Self` (tambahkan OR condition), `order_by(column: &str, asc: bool) -> Self` (tambahkan ORDER BY), `limit(n: u32) -> Self` (tambahkan LIMIT), `offset(n: u32) -> Self` (tambahkan OFFSET), `join(join_type: &str, table: &str, on: &str) -> Self` (tambahkan JOIN). Implementasi `build() -> (String, Vec<Value>)` yang mengembalikan query string dan parameter untuk parameterized query. Implementasi parameterized queries menggunakan `$1, $2, $3` placeholder untuk PostgreSQL untuk mencegah SQL injection (nilai parameter tidak di-escape langsung ke query). Implementasi `build_count()` untuk membangun COUNT query dari query yang ada. Implementasi `build_paginated()` untuk membangun query dengan pagination (LIMIT + OFFSET). Implementasi `validate()` untuk memvalidasi query sebelum eksekusi (memastikan table dan column names valid, tidak ada SQL injection patterns).

---

#### **5. `migrations.rs` (Rust)**

**Tugas:** Mengelola migrasi database untuk version control schema dan update tanpa data loss.

**Tujuan:** File ini mengelola version control untuk schema database. Struct `MigrationManager` dengan fields: `conn: PgConnection` (koneksi database), `migrations_dir: PathBuf` (direktori yang berisi file migrasi). Implementasi `run_migrations()`: list semua file di `migrations/` dengan pattern `{timestamp}_{name}.sql` (contoh: `20240101000000_initial_schema.sql`, `20240115000000_add_cve_table.sql`), sort by timestamp, execute in order. Track yang sudah dijalankan di table `migrations` (id, migration_name, executed_at, checksum) untuk mencegah eksekusi ulang. Implementasi `rollback(last_n: usize)`: rollback N migrations dengan execute `down` section dari migration file (if exists). Migration file format: `-- +migrate Up` untuk section yang dijalankan saat upgrade, `-- +migrate Down` untuk section yang dijalankan saat rollback. Implementasi `create_migration(name: &str)` untuk generate migration file baru dengan timestamp dan template. Implementasi `status()` untuk menampilkan status migrasi saat ini: migrations yang sudah dijalankan dan yang pending. Implementasi `validate()` untuk memverifikasi integritas migrasi (checksum match, tidak ada missing migrations). Implementasi `backup_before_migration()` untuk melakukan backup database otomatis sebelum menjalankan migrasi.

---

**api folder explanation:**

---

## 📂 **`api/` - REST API**

---

#### **1. `rest_api.rs` (Rust)**

**Task:** Provide REST API server for system interaction via HTTP.

**Purpose:** This file implements the REST API server using the `axum` framework for routing and request handling. Struct `RestApi` with fields: `app: Router` (axum router with all routes), `state: AppState` (shared state for handlers), `config: ApiConfig` (API configuration). Implements `new()` which creates `axum::Router` with all route definitions: `post /api/v1/scan` to start new scan (body: `{"url": "example.com", "profile": "moderate"}`), `get /api/v1/scan/:id/status` to get scan status, `get /api/v1/scan/:id/report` to get scan results report, `post /api/v1/analyze/:id` to run analysis on existing data, `get /api/v1/export/:id/:format` to export data in specific format (json, txt, docs, csv, html, pdf), `get /api/v1/history` to view scan history (with pagination), `post /api/v1/monitor` to start scheduled monitoring, `get /api/v1/health` for health check. Implements `run()`: serve on config `host:port`, with `tower::ServiceBuilder` middleware stack: `TraceLayer` for logging each request, `CorsLayer` for CORS headers (allow origins from config), `AuthorizationLayer` for authentication check. Implements `shutdown()` with graceful shutdown: listen for SIGTERM, finish pending requests (timeout 30s), shutdown server. Implements `openapi_docs()` to automatically generate OpenAPI/Swagger documentation.

---

#### **2. `websocket_handler.rs` (Rust)**

**Task:** Provide WebSocket server for real-time communication and streaming.

**Purpose:** This file implements the WebSocket server for real-time communication. Struct `WebSocketHandler` with fields: `clients: Arc<DashMap<String, WebSocket>>` (map from client_id to WebSocket connection), `message_sender: mpsc::Sender<Message>` (sender for broadcast messages). Implements `handle_connection(ws: WebSocket)` with `axum::extract::ws` for upgrading HTTP to WebSocket. Listens for messages from clients: `ping` -> respond `pong` (keep-alive), `subscribe {scan_id}` -> add client to subscribers list for scan_id (client will receive all updates for that scan_id), `unsubscribe {scan_id}` -> remove client from subscribers, `command {cmd}` -> execute command and stream output to client. Implements `broadcast_log(scan_id: &str, log_entry: &str)` for streaming logs to all subscribers of a specific scan_id. Implements `broadcast_progress(scan_id: &str, progress: f32, status: &str)` for streaming progress updates. Implements `broadcast_alert(alert: Alert)` to send alerts to all subscribed clients. Implements `heartbeat()`: send ping to all clients every 30 seconds, if client does not respond within 60 seconds, close connection. Implements `connection_pool()` to manage multiple WebSocket connections with max 1000 concurrent connections, idle timeout 60 seconds.

---

#### **3. `authentication.rs` (Rust)**

**Task:** Provide secure authentication and authorization system for the API.

**Purpose:** This file implements authentication using JWT (JSON Web Tokens) and API keys. Struct `Authentication` with fields: `jwt_secret: String` (secret key for signing JWT), `validator: Validator` (token validator). Implements `generate_jwt(user_id: &str, role: &str) -> String` using `jsonwebtoken` crate. Claims: `sub` (user_id), `role` (admin, user, guest), `exp` (expiration time = current_time + 24 hours), `iat` (issued at), `iss` (issuer = "iws"). Implements `validate_jwt(token: &str) -> Result<Claims>`: verify signature (using HS256 algorithm), check expiration (exp > current_time), check issuer (iss == "iws"), extract claims. Implements `api_key_middleware()`: check `X-API-Key` header, validate against stored keys in database (table `api_keys` with key, user_id, permissions, expires_at). Implements `permission_check(role: &str, required_role: &str) -> bool`: admin can perform all actions, user can perform actions for their own scans, guest can only read-only. Implements `rate_limit_by_user()`: users with admin role have higher limit (1000 requests/hour), user (100/hour), guest (10/hour). Implements `refresh_token()`: generate new JWT from refresh token to extend session without re-login. Implements `blacklist_token()` to add token to blacklist (Redis) if logout or token compromised.

---

#### **4. `rate_limiter_middleware.rs` (Rust)**

**Task:** Provide rate limiting on API to prevent abuse and DoS attacks.

**Purpose:** This file implements middleware for rate limiting on the API. Struct `RateLimiterMiddleware` with fields: `limiter: Arc<RateLimiter>` (shared rate limiter), `config: RateLimiterConfig` (rate limit configuration). Implements `call(request: Request) -> Result<Response>`: extract IP address from `X-Forwarded-For` header (if present) or socket address (`req.remote_addr()`). Check rate limit for IP: max 100 requests per minute (default). If exceeded, return HTTP 429 (Too Many Requests) with `Retry-After` header (time in seconds until rate limit resets). Implements `per_api_key_limit`: limit per API key based on tier (admin: 1000/hour, user: 100/hour, guest: 10/hour). Implements `per_route_limit`: different limits for different endpoints (`/api/v1/scan` stricter (10/minute) than `/api/v1/status` (1000/minute)). Implements `sliding_window_counter()` for more accurate rate limiting: store request timestamps in Redis sorted set, check count in last N seconds. Implements `token_bucket()` for burst handling: allow burst up to capacity (e.g., 100 requests in 1 minute, burst up to 20). Implements `get_rate_limit_status()` to return remaining requests and reset time in response headers (`X-RateLimit-Limit`, `X-RateLimit-Remaining`, `X-RateLimit-Reset`).

---

#### **5. `endpoint_routes.rs` (Rust)**

**Task:** Define all endpoint routes and handler functions in an organized manner.

**Purpose:** This file defines route definitions with `axum::Router` nesting. Groups: `/api/v1` root with sub-routes: `scan_routes()`: `/scan` (POST to start, GET for list, DELETE to cancel), `analysis_routes()`: `/analyze` (POST to run analysis, GET to get results), `report_routes()`: `/report` (GET to get report, POST to generate), `export_routes()`: `/export` (GET to export data), `history_routes()`: `/history` (GET for scan history), `monitor_routes()`: `/monitor` (POST to start monitoring, DELETE to stop). Implements handler functions with async `fn handler(State(state): State<AppState>, Query(params): Query<Params>) -> impl IntoResponse`. Each handler uses `axum::Json` for JSON serialization/deserialization. Implements `extractors`: `Path<Uuid>` to extract scan_id from URL, `Query<HashMap<String, String>>` for query parameters, `Json<T>` for JSON body. Implements `error_handling()`: convert internal errors to HTTP status codes (404 for Not Found, 400 for Bad Request, 500 for Internal Server Error) with JSON error response format: `{"error": {"code": "E1001", "message": "Scan not found", "details": {...}}}`. Implements `openapi_docs()` with `utoipa` library to generate OpenAPI specification from route definitions. Implements `validate_request()` to validate request body using `validator` crate before processing.

---

**indonesian:**

---

## 📂 **`api/` - REST API**

---

#### **1. `rest_api.rs` (Rust)**

**Tugas:** Menyediakan REST API server untuk interaksi dengan sistem melalui HTTP.

**Tujuan:** File ini mengimplementasikan REST API server menggunakan framework `axum` untuk routing dan handling request. Struct `RestApi` dengan fields: `app: Router` (axum router dengan semua routes), `state: AppState` (shared state untuk handlers), `config: ApiConfig` (konfigurasi API). Implementasi `new()` yang membuat `axum::Router` dengan semua route definitions: `post /api/v1/scan` untuk memulai scanning baru (body: `{"url": "example.com", "profile": "moderate"}`), `get /api/v1/scan/:id/status` untuk mendapatkan status scanning, `get /api/v1/scan/:id/report` untuk mendapatkan laporan hasil scan, `post /api/v1/analyze/:id` untuk menjalankan analisis pada data yang sudah ada, `get /api/v1/export/:id/:format` untuk mengekspor data dalam format tertentu (json, txt, docs, csv, html, pdf), `get /api/v1/history` untuk melihat riwayat scanning (dengan pagination), `post /api/v1/monitor` untuk memulai monitoring berjadwal, `get /api/v1/health` untuk health check. Implementasi `run()`: serve on config `host:port`, dengan `tower::ServiceBuilder` middleware stack: `TraceLayer` untuk logging setiap request, `CorsLayer` untuk CORS headers (allow origins dari config), `AuthorizationLayer` untuk authentication check. Implementasi `shutdown()` dengan graceful shutdown: listen untuk SIGTERM, finish pending requests (timeout 30s), shutdown server. Implementasi `openapi_docs()` untuk menghasilkan OpenAPI/Swagger documentation secara otomatis.

---

#### **2. `websocket_handler.rs` (Rust)**

**Tugas:** Menyediakan WebSocket server untuk real-time communication dan streaming.

**Tujuan:** File ini mengimplementasikan WebSocket server untuk komunikasi real-time. Struct `WebSocketHandler` dengan fields: `clients: Arc<DashMap<String, WebSocket>>` (map dari client_id ke WebSocket connection), `message_sender: mpsc::Sender<Message>` (sender untuk broadcast messages). Implementasi `handle_connection(ws: WebSocket)` dengan `axum::extract::ws` untuk upgrade HTTP ke WebSocket. Listen for messages dari client: `ping` -> respond `pong` (keep-alive), `subscribe {scan_id}` -> add client to subscribers list for scan_id (client akan menerima semua update untuk scan_id tersebut), `unsubscribe {scan_id}` -> remove client from subscribers, `command {cmd}` -> execute command dan stream output ke client. Implementasi `broadcast_log(scan_id: &str, log_entry: &str)` untuk streaming log ke semua subscribers dari scan_id tertentu. Implementasi `broadcast_progress(scan_id: &str, progress: f32, status: &str)` untuk streaming progress updates. Implementasi `broadcast_alert(alert: Alert)` untuk mengirim alert ke semua clients yang subscribe. Implementasi `heartbeat()`: kirim ping ke semua clients setiap 30 detik, jika client tidak merespon dalam 60 detik, tutup connection. Implementasi `connection_pool()` untuk manage multiple WebSocket connections dengan max 1000 concurrent connections, idle timeout 60 detik.

---

#### **3. `authentication.rs` (Rust)**

**Tugas:** Menyediakan sistem autentikasi dan otorisasi yang aman untuk API.

**Tujuan:** File ini mengimplementasikan autentikasi menggunakan JWT (JSON Web Tokens) dan API keys. Struct `Authentication` dengan fields: `jwt_secret: String` (secret key untuk signing JWT), `validator: Validator` (validator untuk token). Implementasi `generate_jwt(user_id: &str, role: &str) -> String` menggunakan `jsonwebtoken` crate. Claims: `sub` (user_id), `role` (admin, user, guest), `exp` (expiration time = current_time + 24 jam), `iat` (issued at), `iss` (issuer = "iws"). Implementasi `validate_jwt(token: &str) -> Result<Claims>`: verify signature (menggunakan HS256 algorithm), check expiration (exp > current_time), check issuer (iss == "iws"), extract claims. Implementasi `api_key_middleware()`: check `X-API-Key` header, validate against stored keys in database (table `api_keys` dengan key, user_id, permissions, expires_at). Implementasi `permission_check(role: &str, required_role: &str) -> bool`: admin dapat melakukan semua actions, user dapat melakukan actions untuk scan sendiri, guest hanya dapat read-only. Implementasi `rate_limit_by_user()`: user dengan role admin memiliki limit lebih tinggi (1000 requests/hour), user (100/hour), guest (10/hour). Implementasi `refresh_token()`: generate new JWT from refresh token untuk memperpanjang session tanpa login ulang. Implementasi `blacklist_token()` untuk menambahkan token ke blacklist (Redis) jika logout atau token compromised.

---

#### **4. `rate_limiter_middleware.rs` (Rust)**

**Tugas:** Memberikan rate limiting pada API untuk mencegah abuse dan DoS attacks.

**Tujuan:** File ini mengimplementasikan middleware untuk rate limiting pada API. Struct `RateLimiterMiddleware` dengan fields: `limiter: Arc<RateLimiter>` (shared rate limiter), `config: RateLimiterConfig` (konfigurasi rate limit). Implementasi `call(request: Request) -> Result<Response>`: extract IP address dari `X-Forwarded-For` header (jika ada) atau socket address (`req.remote_addr()`). Check rate limit untuk IP: max 100 requests per minute (default). Jika exceeded, return HTTP 429 (Too Many Requests) dengan `Retry-After` header (waktu dalam detik sampai rate limit reset). Implementasi `per_api_key_limit`: limit per API key berdasarkan tier (admin: 1000/hour, user: 100/hour, guest: 10/hour). Implementasi `per_route_limit`: different limits untuk different endpoints (`/api/v1/scan` stricter (10/minute) daripada `/api/v1/status` (1000/minute)). Implementasi `sliding_window_counter()` untuk rate limiting yang lebih akurat: store request timestamps in Redis sorted set, check count in last N seconds. Implementasi `token_bucket()` untuk burst handling: allow burst up to capacity (contoh: 100 requests in 1 minute, burst up to 20). Implementasi `get_rate_limit_status()` untuk mengembalikan remaining requests dan reset time dalam response headers (`X-RateLimit-Limit`, `X-RateLimit-Remaining`, `X-RateLimit-Reset`).

---

#### **5. `endpoint_routes.rs` (Rust)**

**Tugas:** Mendefinisikan semua endpoint routes dan handler functions secara terorganisir.

**Tujuan:** File ini mendefinisikan route definitions dengan `axum::Router` nesting. Groups: `/api/v1` root yang memiliki sub-routes: `scan_routes()`: `/scan` (POST untuk start, GET untuk list, DELETE untuk cancel), `analysis_routes()`: `/analyze` (POST untuk run analysis, GET untuk get results), `report_routes()`: `/report` (GET untuk get report, POST untuk generate), `export_routes()`: `/export` (GET untuk export data), `history_routes()`: `/history` (GET untuk history scanning), `monitor_routes()`: `/monitor` (POST untuk start monitoring, DELETE untuk stop). Implementasi handler functions dengan async `fn handler(State(state): State<AppState>, Query(params): Query<Params>) -> impl IntoResponse`. Setiap handler menggunakan `axum::Json` untuk JSON serialization/deserialization. Implementasi `extractors`: `Path<Uuid>` untuk extract scan_id dari URL, `Query<HashMap<String, String>>` untuk query parameters, `Json<T>` untuk JSON body. Implementasi `error_handling()`: convert internal errors ke HTTP status codes (404 for Not Found, 400 for Bad Request, 500 for Internal Server Error) dengan JSON error response format: `{"error": {"code": "E1001", "message": "Scan not found", "details": {...}}}`. Implementasi `openapi_docs()` dengan `utoipa` library untuk generate OpenAPI specification dari route definitions. Implementasi `validate_request()` untuk memvalidasi request body menggunakan `validator` crate sebelum diproses.

---

**reports folder explanation:**

---

## 📂 **`reports/` - REPORTING**

---

#### **1. `report_factory.rs` (Rust)**

**Task:** Implement factory pattern to create various types of reports in different formats.

**Purpose:** This file serves as a report factory that generates various report formats based on user needs. Struct `ReportFactory` with fields: `reporters: HashMap<String, Box<dyn Reporter>>` (map from format to reporter implementation). Implements `create_report(data: &ReportData, format: &str) -> Result<Vec<u8>>`: selects reporter based on requested format. Available reporters: `JsonReporter` -> uses `serde_json::to_vec_pretty()` to generate structured JSON, `TxtReporter` -> uses `txt_generator` for plain text, `DocsReporter` -> uses `docs_builder` for DOCX/ODT, `CsvReporter` -> uses `csv_exporter` for spreadsheet, `HtmlReporter` -> uses `html_reporter` for interactive HTML, `PdfReporter` -> uses `pdf_generator` for professional PDF. Implements `get_supported_formats() -> Vec<String>` to return list of supported formats. Implements `get_recommended_format()` to recommend format based on data type and user (e.g., technical team -> JSON/HTML, management -> PDF/DOCS). Implements `validate_data()` to ensure data is complete before creating report. Implements `add_custom_reporter()` to allow users to add custom reporters.

---

#### **2. `executive_summary_builder.rs` (Rust)**

**Task:** Build concise and informative executive summary for management and non-technical stakeholders.

**Purpose:** This file generates an executive summary in natural language that is easy for management to understand. Struct `ExecutiveSummaryBuilder` with fields: `summarizer: Box<dyn Summarizer>` (AI summarizer), `template: String` (template for output). Implements `build(data: &ReportData) -> String`: generates natural language summary using AI model (LLama3/Mistral) with prompt engineering. Prompt template: `"Based on the following security findings, generate an executive summary for business stakeholders. Focus on business impact, critical issues requiring immediate attention, and recommended actions. Keep it concise (max 300 words) and avoid technical jargon.\n\nFindings: {findings_json}\n\nTarget: {target_url}\nScan Date: {scan_date}"`. Implements `extract_key_findings()` to extract the most important findings (critical and high severity) from all findings. Implements `generate_recommendations()`: based on vulnerability list, creates prioritized action items with estimated effort and impact. Implements `calculate_business_risk()`: translates technical risk to business risk (financial impact, reputational impact, regulatory impact). Implements `format_for_stakeholder()`: adjusts language for various stakeholders (CISO, CEO, Developer, Legal). Implements `add_executive_highlights()`: adds brief highlights (3-5 bullet points) at the beginning of the summary.

---

#### **3. `technical_deep_dive.rs` (Rust)**

**Task:** Generate detailed technical reports for security teams and developers.

**Purpose:** This file produces comprehensive technical reports with all technical details. Struct `TechnicalDeepDive` with fields: `data: ReportData`, `format: TechnicalFormat` (output format). Implements `generate_document()`: organizes findings by severity (critical, high, medium, low), then by type (network, content, security, infrastructure). Each finding includes: `vulnerability_description` (technical description of the vulnerability), `affected_systems` (systems impacted), `proof_of_concept` (code or steps to reproduce), `impact_analysis` (impact if exploited), `remediation_steps` (fix steps with detailed instructions). Adds technical references: CVE links (nvd.nist.gov), OWASP references (owasp.org), vendor security advisories. Implements `generate_network_diagram()` to create network diagram showing target architecture. Implements `generate_code_snippets()` to provide example fix code (e.g., how to fix XSS with encoding, how to add CSRF token). Implements `generate_testing_steps()` to provide verification steps for fixes. Implements `add_technical_glossary()` to explain technical terms used.

---

#### **4. `vulnerability_tracker.rs` (Rust)**

**Task:** Track all discovered vulnerabilities and their remediation status.

**Purpose:** This file manages the lifecycle of each discovered vulnerability. Struct `VulnerabilityTracker` with fields: `tracker: HashMap<String, VulnerabilityStatus>` (map from vulnerability ID to status), `history: Vec<StatusChange>` (status change history). Implements `track(vulnerability: &Vulnerability)`: assigns unique ID to vulnerability (format: `VULN-{timestamp}-{counter}`), initial status `"open"` with timestamp. Implements `update_status(vuln_id: &str, new_status: &str)`: updates status to (open, in_progress, fixed, won_fix, false_positive), records change in history with timestamp, user, and comment. Implements `get_status_summary()`: calculates total vulnerabilities per status (open: 5, in_progress: 3, fixed: 10, won_fix: 2). Implements `calculate_sla_compliance()`: calculates percentage of vulnerabilities fixed within SLA (Service Level Agreement) - e.g., critical must be fixed within 24 hours, high within 7 days, medium within 30 days, low within 90 days. Implements `generate_tracker_report()`: creates complete tracking report with all vulnerabilities, current status, time open, and SLA compliance. Implements `get_vulnerability_timeline()`: gets complete timeline of each vulnerability (discovered, analyzed, assigned, fixed, verified). Implements `assign_owner(vuln_id: &str, owner: &str)` to assign responsible owner for remediation.

---

#### **5. `timeline_generator.rs` (Rust)**

**Task:** Generate timeline of scanning activity and findings in chronological format.

**Purpose:** This file generates a visual timeline showing the sequence of events during scanning and analysis. Struct `TimelineGenerator` with fields: `events: Vec<TimelineEvent>` (list of events that occurred). Implements `add_event(timestamp: DateTime, event_type: &str, details: &str)` to add event to timeline. Event types: `ScanStarted` (scanning started), `ModuleStarted` (scanning module started), `ModuleCompleted` (scanning module completed), `FindingFound` (vulnerability found), `ScanCompleted` (scanning completed), `AnalysisStarted` (analysis started), `AnalysisCompleted` (analysis completed), `ReportGenerated` (report generated), `ChangeDetected` (change detected by monitoring). Implements `generate()`: sorts events by timestamp, formats with ASCII timeline or visual chart. ASCII timeline format:
```
[2024-01-15 10:00:00] Scan Started for example.com
[2024-01-15 10:00:05] Network Module Started - DNS Enumeration
[2024-01-15 10:00:30] Network Module Completed - 5 records found
[2024-01-15 10:00:35] Security Module Started - XSS Detection
[2024-01-15 10:01:00] Finding Found - XSS vulnerability in /search?q= parameter
[2024-01-15 10:01:30] Security Module Completed - 2 vulnerabilities found
[2024-01-15 10:05:00] Scan Completed - Duration: 5m 0s
[2024-01-15 10:05:10] Analysis Started
[2024-01-15 10:10:00] Analysis Completed - 5 findings total
[2024-01-15 10:10:05] Report Generated - PDF format
```
Implements `generate_chart()` for visual chart using Chart.js or matplotlib: bar chart showing events by time. Implements `export_timeline(format: TimelineFormat)` to export timeline in various formats (text, HTML, JSON). Implements `correlate_events()` to correlate related events (e.g., FindingFound with ModuleStarted that led to the discovery).

---

#### **6. `graph_visualizer.py` (Python)**

**Task:** Create data visualizations in the form of graphs and charts for reports.

**Purpose:** This file generates various data visualizations using matplotlib and plotly. Class `GraphVisualizer` with fields: `plotter: matplotlib.pyplot`, `style: str` (style for charts). Implements `create_bar_chart(data: Dict[str, int], title: str) -> bytes`: matplotlib bar chart for severity distribution (critical, high, medium, low, info), returns PNG bytes. Implements `create_pie_chart(data: Dict[str, int], title: str) -> bytes`: pie chart for vulnerability type percentages (XSS, SQLi, CSRF, Header Issues, etc). Implements `create_line_chart(dates: List[datetime], values: List[float], title: str) -> bytes`: line chart for timeline activity (vulnerabilities over time, risk score over time). Implements `create_network_graph(nodes: List[Node], edges: List[Edge]) -> bytes`: uses networkx for graph visualization of link graph (shows relationships between pages, internal/external links). Implements `create_heat_map(data: List[List[float]], labels: List[str]) -> bytes`: heat map for threat geolocation or port distribution. Implements `create_radar_chart(categories: List[str], values: List[float], title: str) -> bytes`: radar chart for comparing multiple metrics (security score across various categories). Implements `create_donut_chart()` for donut chart (pie chart with hole in the center). Implements `apply_style()` to apply consistent style to all charts (colors, fonts, sizes). Implements `export_charts()` to export all charts as images (PNG, SVG) for insertion into reports.

---

**indonesian:**

---

## 📂 **`reports/` - PELAPORAN**

---

#### **1. `report_factory.rs` (Rust)**

**Tugas:** Mengimplementasikan factory pattern untuk membuat berbagai jenis laporan dalam berbagai format.

**Tujuan:** File ini berfungsi sebagai pabrik laporan yang menghasilkan berbagai format laporan berdasarkan kebutuhan pengguna. Struct `ReportFactory` dengan fields: `reporters: HashMap<String, Box<dyn Reporter>>` (map dari format ke reporter implementation). Implementasi `create_report(data: &ReportData, format: &str) -> Result<Vec<u8>>`: pilih reporter berdasarkan format yang diminta. Reporter yang tersedia: `JsonReporter` -> menggunakan `serde_json::to_vec_pretty()` untuk menghasilkan JSON terstruktur, `TxtReporter` -> menggunakan `txt_generator` untuk plain text, `DocsReporter` -> menggunakan `docs_builder` untuk DOCX/ODT, `CsvReporter` -> menggunakan `csv_exporter` untuk spreadsheet, `HtmlReporter` -> menggunakan `html_reporter` untuk HTML interaktif, `PdfReporter` -> menggunakan `pdf_generator` untuk PDF profesional. Implementasi `get_supported_formats() -> Vec<String>` untuk mengembalikan list format yang didukung. Implementasi `get_recommended_format()` untuk merekomendasikan format berdasarkan jenis data dan pengguna (contoh: technical team -> JSON/HTML, management -> PDF/DOCS). Implementasi `validate_data()` untuk memastikan data lengkap sebelum membuat laporan. Implementasi `add_custom_reporter()` untuk memungkinkan pengguna menambahkan reporter kustom.

---

#### **2. `executive_summary_builder.rs` (Rust)**

**Tugas:** Membangun executive summary yang ringkas dan informatif untuk manajemen dan stakeholder non-teknis.

**Tujuan:** File ini menghasilkan ringkasan eksekutif dalam bahasa alami yang mudah dipahami oleh manajemen. Struct `ExecutiveSummaryBuilder` dengan fields: `summarizer: Box<dyn Summarizer>` (AI summarizer), `template: String` (template untuk output). Implementasi `build(data: &ReportData) -> String`: generate natural language summary menggunakan AI model (LLama3/Mistral) dengan prompt engineering. Prompt template: `"Based on the following security findings, generate an executive summary for business stakeholders. Focus on business impact, critical issues requiring immediate attention, and recommended actions. Keep it concise (max 300 words) and avoid technical jargon.\n\nFindings: {findings_json}\n\nTarget: {target_url}\nScan Date: {scan_date}"`. Implementasi `extract_key_findings()` untuk mengekstrak temuan paling penting (critical dan high severity) dari semua findings. Implementasi `generate_recommendations()`: berdasarkan vulnerability list, buat prioritized action items dengan estimated effort dan impact. Implementasi `calculate_business_risk()`: terjemahkan technical risk ke business risk (financial impact, reputational impact, regulatory impact). Implementasi `format_for_stakeholder()`: sesuaikan bahasa untuk berbagai stakeholder (CISO, CEO, Developer, Legal). Implementasi `add_executive_highlights()`: tambahkan highlight singkat (3-5 bullet points) di awal summary.

---

#### **3. `technical_deep_dive.rs` (Rust)**

**Tugas:** Menghasilkan laporan teknis yang mendetail untuk tim keamanan dan developer.

**Tujuan:** File ini menghasilkan laporan teknis yang komprehensif dengan semua detail teknis. Struct `TechnicalDeepDive` dengan fields: `data: ReportData`, `format: TechnicalFormat` (format output). Implementasi `generate_document()`: organize findings by severity (critical, high, medium, low), kemudian by type (network, content, security, infrastructure). Setiap finding mencakup: `vulnerability_description` (deskripsi teknis kerentanan), `affected_systems` (sistem yang terkena dampak), `proof_of_concept` (kode atau langkah-langkah untuk mereproduksi), `impact_analysis` (dampak jika dieksploitasi), `remediation_steps` (langkah-langkah perbaikan dengan instruksi detail). Tambahkan technical references: CVE links (nvd.nist.gov), OWASP references (owasp.org), vendor security advisories. Implementasi `generate_network_diagram()` untuk membuat diagram jaringan yang menunjukkan arsitektur target. Implementasi `generate_code_snippets()` untuk memberikan contoh kode perbaikan (misal: cara memperbaiki XSS dengan encoding, cara menambahkan CSRF token). Implementasi `generate_testing_steps()` untuk memberikan langkah-langkah verifikasi perbaikan. Implementasi `add_technical_glossary()` untuk menjelaskan istilah-istilah teknis yang digunakan.

---

#### **4. `vulnerability_tracker.rs` (Rust)**

**Tugas:** Melacak semua kerentanan yang ditemukan dan status perbaikannya.

**Tujuan:** File ini mengelola lifecycle dari setiap vulnerability yang ditemukan. Struct `VulnerabilityTracker` dengan fields: `tracker: HashMap<String, VulnerabilityStatus>` (map dari vulnerability ID ke status), `history: Vec<StatusChange>` (riwayat perubahan status). Implementasi `track(vulnerability: &Vulnerability)`: assign unique ID ke vulnerability (format: `VULN-{timestamp}-{counter}`), initial status `"open"` dengan timestamp. Implementasi `update_status(vuln_id: &str, new_status: &str)`: update status ke (open, in_progress, fixed, won_fix, false_positive), record change in history dengan timestamp, user, dan comment. Implementasi `get_status_summary()`: menghitung total vulnerabilities per status (open: 5, in_progress: 3, fixed: 10, won_fix: 2). Implementasi `calculate_sla_compliance()`: menghitung persentase vulnerabilities yang diperbaiki dalam SLA (Service Level Agreement) - contoh: critical harus diperbaiki dalam 24 jam, high dalam 7 hari, medium dalam 30 hari, low dalam 90 hari. Implementasi `generate_tracker_report()`: membuat laporan tracking lengkap dengan semua vulnerabilities, current status, time open, dan SLA compliance. Implementasi `get_vulnerability_timeline()`: mendapatkan timeline lengkap dari setiap vulnerability (discovered, analyzed, assigned, fixed, verified). Implementasi `assign_owner(vuln_id: &str, owner: &str)` untuk menetapkan pemilik yang bertanggung jawab atas perbaikan.

---

#### **5. `timeline_generator.rs` (Rust)**

**Tugas:** Generate timeline aktivitas scanning dan findings dalam format kronologis.

**Tujuan:** File ini menghasilkan timeline visual yang menunjukkan urutan kejadian selama scanning dan analisis. Struct `TimelineGenerator` dengan fields: `events: Vec<TimelineEvent>` (daftar event yang terjadi). Implementasi `add_event(timestamp: DateTime, event_type: &str, details: &str)` untuk menambahkan event ke timeline. Event types: `ScanStarted` (scanning dimulai), `ModuleStarted` (module scanning dimulai), `ModuleCompleted` (module scanning selesai), `FindingFound` (kerentanan ditemukan), `ScanCompleted` (scanning selesai), `AnalysisStarted` (analisis dimulai), `AnalysisCompleted` (analisis selesai), `ReportGenerated` (laporan dihasilkan), `ChangeDetected` (perubahan terdeteksi oleh monitoring). Implementasi `generate()`: sort events by timestamp, format dengan ASCII timeline atau visual chart. ASCII timeline format: 
```
[2024-01-15 10:00:00] Scan Started for example.com
[2024-01-15 10:00:05] Network Module Started - DNS Enumeration
[2024-01-15 10:00:30] Network Module Completed - 5 records found
[2024-01-15 10:00:35] Security Module Started - XSS Detection
[2024-01-15 10:01:00] Finding Found - XSS vulnerability in /search?q= parameter
[2024-01-15 10:01:30] Security Module Completed - 2 vulnerabilities found
[2024-01-15 10:05:00] Scan Completed - Duration: 5m 0s
[2024-01-15 10:05:10] Analysis Started
[2024-01-15 10:10:00] Analysis Completed - 5 findings total
[2024-01-15 10:10:05] Report Generated - PDF format
```
Implementasi `generate_chart()` untuk visual chart menggunakan Chart.js atau matplotlib: bar chart showing events by time. Implementasi `export_timeline(format: TimelineFormat)` untuk mengekspor timeline dalam berbagai format (text, HTML, JSON). Implementasi `correlate_events()` untuk menghubungkan event yang terkait (misal: FindingFound dengan ModuleStarted yang menyebabkan penemuan tersebut).

---

#### **6. `graph_visualizer.py` (Python)**

**Tugas:** Membuat visualisasi data dalam bentuk grafik dan charts untuk laporan.

**Tujuan:** File ini menghasilkan berbagai visualisasi data menggunakan matplotlib dan plotly. Class `GraphVisualizer` dengan fields: `plotter: matplotlib.pyplot`, `style: str` (style untuk chart). Implementasi `create_bar_chart(data: Dict[str, int], title: str) -> bytes`: matplotlib bar chart untuk distribusi severity (critical, high, medium, low, info), return PNG bytes. Implementasi `create_pie_chart(data: Dict[str, int], title: str) -> bytes`: pie chart untuk persentase jenis vulnerability (XSS, SQLi, CSRF, Header Issues, etc). Implementasi `create_line_chart(dates: List[datetime], values: List[float], title: str) -> bytes`: line chart untuk timeline activity (vulnerabilities over time, risk score over time). Implementasi `create_network_graph(nodes: List[Node], edges: List[Edge]) -> bytes`: menggunakan networkx untuk graph visualization dari link graph (menunjukkan hubungan antar halaman, internal/external links). Implementasi `create_heat_map(data: List[List[float]], labels: List[str]) -> bytes`: heat map untuk geolokasi ancaman atau port distribution. Implementasi `create_radar_chart(categories: List[str], values: List[float], title: str) -> bytes`: radar chart untuk perbandingan multiple metrics (security score di berbagai kategori). Implementasi `create_donut_chart()` untuk donut chart (pie chart dengan hole di tengah). Implementasi `apply_style()` untuk menerapkan style konsisten ke semua charts (colors, fonts, sizes). Implementasi `export_charts()` untuk mengekspor semua charts sebagai gambar (PNG, SVG) untuk disisipkan ke laporan.

---

**terminal folder explanation:**

---

## 📂 **`terminal/` - TERMINAL INTERFACE**

---

#### **1. `cli_interface.rs` (Rust)**

**Task:** Provide the main Command Line Interface (CLI) for user interaction via terminal.

**Purpose:** This file is the main gateway for user interaction through the command line. Struct `CliInterface` with fields: `parser: Parser` (clap parser for command line arguments), `config: CliConfig` (CLI configuration). Implements `run()`: parses command line arguments using `clap` with structured subcommands. Subcommands and their functions: `scan` (with args: `--url`, `--profile`, `--output`, `--max-pages`) to start new scanning, `analyze` (with args: `--scan-id`) to run analysis on completed scan, `report` (with args: `--scan-id`, `--format`, `--output`) to generate report, `export` (with args: `--scan-id`, `--format`, `--output`) to export data, `monitor` (with args: `--url`, `--schedule`, `--alert`) to start scheduled monitoring, `status` (with args: `--scan-id` or `--all`) to check scan status, `config` (with args: `--get`, `--set`, `--list`) to manage configuration, `history` (with args: `--limit`, `--filter`) to view scan history. Implements `show_help()` to display comprehensive usage information with examples. Implements `validate_args()` to validate arguments before execution (e.g., URL must be valid, scan_id must be UUID, format must be supported). Implements `handle_completion()` to support shell completion (bash, zsh, fish). Implements `format_output()` to format output with colors (success = green, warning = yellow, error = red) using `colored` crate.

---

#### **2. `termux_support.rs` (Rust)**

**Task:** Provide specific optimizations for Termux environment (Android terminal emulator).

**Purpose:** This file optimizes user experience in Termux (Android). Struct `TermuxSupport` with fields: `terminal_size: (usize, usize)` (terminal width and height in characters), `color_support: bool` (whether terminal supports color). Implements `detect_terminal()`: uses `term_size` crate to get terminal size (width and height). If undetectable, falls back to default (80x24). Implements `supports_color()`: checks `TERM` environment variable for values that support color (xterm, xterm-256color, screen, tmux), and `tput colors` command to get number of supported colors. Implements `format_progress(progress: f32, width: usize) -> String`: generates progress bar compatible with Termux with format `[#######---] 70%`. Adjusts width to terminal size (using width-10 for progress bar). Implements `get_input()`: uses `crossterm` or `termion` for non-blocking input compatible with Android touch keyboard. Implements `format_table()`: formats table with column widths adjusting to terminal width (truncates if too long). Implements `scrollable_output()`: for long output, supports scrolling with arrow keys. Implements `detect_termux()`: checks whether `TERMUX_VERSION` environment variable exists to detect Termux environment.

---

#### **3. `desktop_support.go` (Go)**

**Task:** Provide support for desktop environments (Windows, Linux, macOS).

**Purpose:** This file handles integration with desktop environments. Struct `DesktopSupport` with fields: `gui_enabled: bool` (whether GUI is available), `notifications_enabled: bool` (whether desktop notifications are available). Implements `ShowNotification(title string, message string)`: displays desktop notifications using platform-specific methods: Windows uses `toast` (via `github.com/go-toast/toast`), Linux uses `notify-send` (DBus), macOS uses `NSUserNotification` (via Objective-C bridge). Implements `SystemTray()`: creates system tray icon with menu options: Start Scan (starts scanning), View Reports (opens reports in browser), Monitor Status (displays status), Configuration (opens settings), Exit (exits application). Uses `fyne` or `gioui` for GUI components. Implements `OpenInBrowser(url string)`: opens URL in default system browser (Windows: `cmd /c start`, Linux: `xdg-open`, macOS: `open`). Implements `GetSystemInfo()`: gets system information (OS, version, architecture, CPU cores, memory) for logging and debugging. Implements `FilePicker()`: opens file picker dialog to select files (export location, config file). Implements `DesktopShortcut()`: creates shortcut on desktop for easy access.

---

#### **4. `progress_display.rs` (Rust)**

**Task:** Display progress bars and status during scanning in an interactive and informative manner.

**Purpose:** This file displays informative visual progress during operations. Struct `ProgressDisplay` with fields: `progress: f32` (progress 0-100), `status: String` (status message), `tasks: Vec<TaskProgress>` (list of tasks and their progress), `start_time: Instant` (start time). Implements `render()`: draws progress bar with width terminal_width - 20 (uses 20 characters for label). Format: `[#######---] 70% - Scanning DNS records...`. Shows current status, ETA (estimated time arrival) based on progress and elapsed time. Shows tasks completed/total (e.g., "Tasks: 3/10 completed"). Shows detailed status for each task (if available): `- DNS Enumeration: Complete (5 records)`, `- Port Scanning: In Progress (50/100 ports)`, `- XSS Detection: Pending`. Implements `update(progress: f32, status: &str, tasks: &[TaskProgress])`: updates fields and re-renders in place (using `\r` to overwrite). Implements `multi_progress()` for parallel tasks: shows multiple progress bars simultaneously using `tui-rs`/`ratatui` with split screen. Implements `color_coding()`: based on status (success = green, warning = yellow, error = red, info = blue). Implements `animation()`: progress bar with smooth animation (spinning indicator when idle). Implements `render_timeline()`: displays timeline events (e.g., "10:00:05 - DNS Enumeration Started", "10:00:30 - DNS Enumeration Completed").

---

#### **5. `interactive_shell.rs` (Rust)**

**Task:** Provide interactive shell mode where users can run commands interactively.

**Purpose:** This file provides a powerful interactive shell experience. Struct `InteractiveShell` with fields: `history: Vec<String>` (command history), `history_index: usize` (position in history), `completions: Vec<String>` (completion list), `current_dir: PathBuf` (current directory). Implements `run()`: loop with `readline` library (rustyline) displaying prompt `iws> `. Each command: parse, execute, show output. Built-in commands: `scan` (start scanning), `analyze` (run analysis), `report` (generate report), `export` (export data), `monitor` (start monitoring), `status` (check status), `config` (manage config), `history` (show command history), `clear` (clear screen), `exit` (exit shell), `help` (show help). Implements `command_history()`: arrow up/down to navigate history (like in bash). Implements `tab_completion()`: suggests commands (scan, analyze, report, etc), suggests options (--url, --profile, --format), suggests arguments (scan_id, url, file_path). Implements `command_piping()`: supports piping between commands (e.g., `scan example.com | analyze | report -f pdf`). Implements `persistent_session()`: saves state (history, current_dir, variables) to file for restoration in next session. Implements `alias()`: users can create aliases (e.g., `s` for scan, `a` for analyze, `r` for report). Implements `variables()`: users can define variables (e.g., `set PROFILE aggressive`, then use `scan example.com --profile $PROFILE`). Implements `help()` command that displays all available commands with syntax and examples.

---

**indonesian:**

---

## 📂 **`terminal/` - TERMINAL INTERFACE**

---

#### **1. `cli_interface.rs` (Rust)**

**Tugas:** Menyediakan antarmuka Command Line Interface (CLI) utama untuk interaksi pengguna melalui terminal.

**Tujuan:** File ini adalah pintu gerbang utama untuk interaksi pengguna melalui command line. Struct `CliInterface` dengan fields: `parser: Parser` (clap parser untuk command line arguments), `config: CliConfig` (konfigurasi CLI). Implementasi `run()`: parse command line arguments menggunakan `clap` dengan subcommands yang terstruktur. Subcommands dan fungsinya: `scan` (with args: `--url`, `--profile`, `--output`, `--max-pages`) untuk memulai scanning baru, `analyze` (with args: `--scan-id`) untuk menjalankan analisis pada scan yang sudah selesai, `report` (with args: `--scan-id`, `--format`, `--output`) untuk generate laporan, `export` (with args: `--scan-id`, `--format`, `--output`) untuk mengekspor data, `monitor` (with args: `--url`, `--schedule`, `--alert`) untuk memulai monitoring berjadwal, `status` (with args: `--scan-id` atau `--all`) untuk mengecek status scanning, `config` (with args: `--get`, `--set`, `--list`) untuk mengelola konfigurasi, `history` (with args: `--limit`, `--filter`) untuk melihat riwayat scanning. Implementasi `show_help()` untuk menampilkan usage information yang lengkap dengan contoh. Implementasi `validate_args()` untuk memvalidasi argumen sebelum eksekusi (contoh: url harus valid, scan_id harus UUID, format harus didukung). Implementasi `handle_completion()` untuk mendukung shell completion (bash, zsh, fish). Implementasi `format_output()` untuk memformat output dengan warna (success = green, warning = yellow, error = red) menggunakan `colored` crate.

---

#### **2. `termux_support.rs` (Rust)**

**Tugas:** Menyediakan optimasi khusus untuk environment Termux (Android terminal emulator).

**Tujuan:** File ini mengoptimalkan pengalaman pengguna di Termux (Android). Struct `TermuxSupport` dengan fields: `terminal_size: (usize, usize)` (lebar dan tinggi terminal dalam karakter), `color_support: bool` (apakah terminal mendukung warna). Implementasi `detect_terminal()`: use `term_size` crate untuk mendapatkan ukuran terminal (lebar dan tinggi). Jika tidak bisa dideteksi, fallback ke default (80x24). Implementasi `supports_color()`: check `TERM` environment variable untuk nilai yang support warna (xterm, xterm-256color, screen, tmux), dan `tput colors` command untuk mendapatkan jumlah warna yang didukung. Implementasi `format_progress(progress: f32, width: usize) -> String`: generate progress bar yang kompatibel dengan Termux dengan format `[#######---] 70%`. Menyesuaikan width dengan terminal size (menggunakan width-10 untuk progress bar). Implementasi `get_input()`: use `crossterm` atau `termion` untuk non-blocking input yang kompatibel dengan touch keyboard di Android. Implementasi `format_table()`: format tabel dengan lebar kolom yang menyesuaikan dengan terminal width (truncate jika terlalu panjang). Implementasi `scrollable_output()`: untuk output yang panjang, mendukung scrolling dengan arrow keys. Implementasi `detect_termux()`: check apakah `TERMUX_VERSION` environment variable exists untuk mendeteksi Termux environment.

---

#### **3. `desktop_support.go` (Go)**

**Tugas:** Menyediakan support untuk desktop environment (Windows, Linux, macOS).

**Tujuan:** File ini menangani integrasi dengan desktop environment. Struct `DesktopSupport` dengan fields: `gui_enabled: bool` (apakah GUI tersedia), `notifications_enabled: bool` (apakah notifikasi desktop tersedia). Implementasi `ShowNotification(title string, message string)`: menampilkan notifikasi desktop menggunakan platform-specific methods: Windows menggunakan `toast` (via `github.com/go-toast/toast`), Linux menggunakan `notify-send` (DBus), macOS menggunakan `NSUserNotification` (via Objective-C bridge). Implementasi `SystemTray()`: create system tray icon dengan menu options: Start Scan (memulai scanning), View Reports (membuka laporan di browser), Monitor Status (menampilkan status), Configuration (buka pengaturan), Exit (keluar aplikasi). Menggunakan `fyne` atau `gioui` untuk GUI components. Implementasi `OpenInBrowser(url string)`: membuka URL di browser default sistem (Windows: `cmd /c start`, Linux: `xdg-open`, macOS: `open`). Implementasi `GetSystemInfo()`: mendapatkan informasi sistem (OS, version, architecture, CPU cores, memory) untuk logging dan debugging. Implementasi `FilePicker()`: membuka file picker dialog untuk memilih file (export location, config file). Implementasi `DesktopShortcut()`: membuat shortcut di desktop untuk memudahkan akses.

---

#### **4. `progress_display.rs` (Rust)**

**Tugas:** Menampilkan progress bar dan status selama scanning secara interaktif dan informatif.

**Tujuan:** File ini menampilkan visual progress yang informatif selama operasi berlangsung. Struct `ProgressDisplay` dengan fields: `progress: f32` (progress 0-100), `status: String` (status message), `tasks: Vec<TaskProgress>` (daftar task dan progress masing-masing), `start_time: Instant` (waktu mulai). Implementasi `render()`: draw progress bar dengan lebar terminal_width - 20 (gunakan 20 karakter untuk label). Format: `[#######---] 70% - Scanning DNS records...`. Tampilkan current status, ETA (estimated time arrival) berdasarkan progress dan elapsed time. Tampilkan tasks completed/total (contoh: "Tasks: 3/10 completed"). Tampilkan detailed status untuk setiap task (jika ada): `- DNS Enumeration: Complete (5 records)`, `- Port Scanning: In Progress (50/100 ports)`, `- XSS Detection: Pending`. Implementasi `update(progress: f32, status: &str, tasks: &[TaskProgress])`: update fields dan re-render di tempat (menggunakan `\r` untuk overwrite). Implementasi `multi_progress()` untuk parallel tasks: show multiple progress bars simultaneously menggunakan `tui-rs`/`ratatui` dengan split screen. Implementasi `color_coding()`: berdasarkan status (success = green, warning = yellow, error = red, info = blue). Implementasi `animation()`: progress bar dengan animasi yang smooth (spinning indicator saat idle). Implementasi `render_timeline()`: tampilkan timeline events (contoh: "10:00:05 - DNS Enumeration Started", "10:00:30 - DNS Enumeration Completed").

---

#### **5. `interactive_shell.rs` (Rust)**

**Tugas:** Menyediakan interactive shell mode di mana user dapat menjalankan command secara interaktif.

**Tujuan:** File ini memberikan pengalaman shell interaktif yang powerful. Struct `InteractiveShell` dengan fields: `history: Vec<String>` (riwayat perintah), `history_index: usize` (posisi dalam history), `completions: Vec<String>` (daftar completions), `current_dir: PathBuf` (direktori saat ini). Implementasi `run()`: loop dengan `readline` library (rustyline) yang menampilkan prompt `iws> `. Setiap command: parse, execute, show output. Built-in commands: `scan` (start scanning), `analyze` (run analysis), `report` (generate report), `export` (export data), `monitor` (start monitoring), `status` (check status), `config` (manage config), `history` (show command history), `clear` (clear screen), `exit` (exit shell), `help` (show help). Implementasi `command_history()`: arrow up/down untuk navigate history (seperti di bash). Implementasi `tab_completion()`: suggest commands (scan, analyze, report, etc), suggest options (--url, --profile, --format), suggest arguments (scan_id, url, file_path). Implementasi `command_piping()`: support pipe antara commands (contoh: `scan example.com | analyze | report -f pdf`). Implementasi `persistent_session()`: menyimpan state (history, current_dir, variables) ke file untuk di-restore di session berikutnya. Implementasi `alias()`: user dapat membuat alias (contoh: `s` untuk scan, `a` untuk analyze, `r` untuk report). Implementasi `variables()`: user dapat mendefinisikan variabel (contoh: `set PROFILE aggressive`, kemudian gunakan `scan example.com --profile $PROFILE`). Implementasi `help()` command yang menampilkan semua available commands dengan syntax dan contoh.

---

**integration folder explanation:**

---

## 📂 **`integration/` - SERVICE INTEGRATION**

---

#### **1. `shodan_wrapper.rs` (Rust)**

**Task:** Connect the system with Shodan API to obtain network intelligence.

**Purpose:** This file is a wrapper for the Shodan API that allows the system to access network intelligence data from the Shodan search engine. Struct `ShodanWrapper` with fields: `client: Client` (HTTP client with connection pooling), `api_key: String` (API key for authentication), `base_url: String` (Shodan API base URL). Implements `search_ip(ip: &str) -> Result<ShodanHost>`: performs GET request to `/shodan/host/{ip}` to get detailed information about the target IP. Parses response: `hostnames` (list of associated hostnames), `ports` (list of open ports), `os` (detected operating system), `data` (complete banners from running services). Implements `search_domain(domain: &str) -> Result<Vec<ShodanHost>>`: performs GET request to `/shodan/host/search?query=hostname:{domain}` to search for all hosts associated with the domain. Implements `search(query: &str) -> Result<Vec<ShodanResult>>`: performs GET request to `/shodan/host/search?query={query}` to perform free search. Rate limit: 1 request per second for free tier, implements token bucket to respect rate limit. Implements `get_service_banner(ip: &str, port: u16)` to get specific service banner from a given port. Implements `get_geo_location(ip: &str)` to get IP geolocation information.

---

#### **2. `censys_connector.rs` (Rust)**

**Task:** Connect the system with Censys API to obtain internet-wide scanning data.

**Purpose:** This file is a connector for the Censys API that provides internet-wide scanning data. Struct `CensysConnector` with fields: `client: Client`, `api_id: String` (API ID for authentication), `api_secret: String` (API secret), `base_url: String`. Implements `get_ip(ip: &str) -> Result<CensysIp>`: performs GET request to `/v2/hosts/{ip}` to get complete information about the IP. Parses response: `location` (country, city, coordinates), `services` (list of running services with banners), `autonomous_system` (ASN and organization). Implements `get_certificate(cert_hash: &str) -> Result<CensysCert>`: performs GET request to `/v2/certificates/{cert_hash}` to get information about SSL/TLS certificate. Parses response: `fingerprint_sha256`, `subject_dn`, `issuer_dn`, `validity` (start, end), `subject_alt_names`. Implements `search(query: &str) -> Result<Vec<CensysHost>>`: performs POST request to `/v2/hosts/search` with body `{"q": query}` to search for hosts based on query. Implements `get_certificate_history(domain: &str)` to get certificate history for a domain. Implements `get_domain_report(domain: &str)` to get complete domain report.

---

#### **3. `virustotal_adapter.rs` (Rust)**

**Task:** Connect the system with VirusTotal API for malware scanning and URL analysis.

**Purpose:** This file is an adapter for the VirusTotal API that provides malware scanning capabilities. Struct `VirusTotalAdapter` with fields: `client: Client`, `api_key: String`, `base_url: String`. Implements `scan_url(url: &str) -> Result<VtUrlScan>`: performs POST request to `/urls` with URL to submit URL scanning, then GET request to `/analyses/{id}` to get scan results. Parses response: `malicious` (number of malicious detections), `suspicious`, `harmless`, `undetected`. Implements `get_domain_report(domain: &str) -> Result<VtDomain>`: performs GET request to `/domains/{domain}` to get complete information about the domain. Parses response: `last_analysis_stats` (malicious, suspicious, harmless, undetected), `whois` (WHOIS information), `last_http_response` (last response), `categories` (domain categorization). Implements `get_ip_report(ip: &str) -> Result<VtIp>`: performs GET request to `/ip_addresses/{ip}` to get information about the IP. Implements `scan_file(file_data: &[u8]) -> Result<VtFileScan>`: uploads file (max 32MB) for malware scanning. Implements `get_file_report(hash: &str)` to get report from file hash.

---

#### **4. `alienvault_otx.rs` (Rust)**

**Task:** Connect the system with AlienVault OTX (Open Threat Exchange) for threat intelligence.

**Purpose:** This file is an integration with AlienVault OTX that provides open threat intelligence exchange. Struct `AlienvaultOtx` with fields: `client: Client`, `api_key: String`, `base_url: String`. Implements `get_domain_pulses(domain: &str) -> Result<Vec<Pulse>>`: performs GET request to `/api/v1/indicators/domain/{domain}/pulses` to get pulses (threat intelligence reports) associated with the domain. Parses response: `pulse_id`, `name`, `description`, `created`, `modified`, `tags`, `references`. Implements `get_ip_pulses(ip: &str) -> Result<Vec<Pulse>>`: performs GET request to `/api/v1/indicators/IPv4/{ip}/pulses` to get pulses associated with the IP. Implements `get_domain_general(domain: &str) -> Result<OtxIndicator>`: performs GET request to `/api/v1/indicators/domain/{domain}/general` to get general information about the domain. Parses response: `whois` (structured WHOIS information), `geo` (geolocation), `url_list` (list of associated URLs), `passive_dns` (passive DNS records). Implements `get_indicator_by_type(indicator_type: &str, value: &str)` to get indicator by type (domain, IP, URL, hash). Implements `get_pulse_details(pulse_id: &str)` to get complete details of a specific pulse.

---

#### **5. `urlscan_integration.rs` (Rust)**

**Task:** Connect the system with URLScan API for website behavior analysis.

**Purpose:** This file is an integration with URLScan API that analyzes website behavior. Struct `UrlscanIntegration` with fields: `client: Client`, `api_key: Option<String>` (optional for higher rate limit), `base_url: String`. Implements `submit_scan(url: &str) -> Result<UrlscanSubmission>`: performs POST request to `/scan` with body `{"url": url, "public": "off"}` to submit URL scanning privately (results only viewable with API key). Parses response: `uuid` (submission ID), `api` (URL for results), `visibility` (public/private). Implements `get_result(uuid: &str) -> Result<UrlscanResult>`: performs GET request to `/result/{uuid}` to get scan results. Parses response: `screenshot_url` (screenshot URL), `dom` (DOM snapshot), `verdicts` (malicious, phishing, suspicious), `links` (external/internal links), `pages` (pages found). Implements `search(query: &str) -> Result<Vec<UrlscanSubmission>>`: performs GET request to `/search/?q={query}` to search previous submissions. Implements `get_domain_report(domain: &str)` to get complete domain report. Implements `get_ip_report(ip: &str)` to get report about the IP.

---

#### **6. `securitytrails_client.rs` (Rust)**

**Task:** Connect the system with SecurityTrails API for DNS history and domain intelligence.

**Purpose:** This file is a client for the SecurityTrails API that provides DNS history and domain intelligence. Struct `SecurityTrailsClient` with fields: `client: Client`, `api_key: String`, `base_url: String`. Implements `get_domain_details(domain: &str) -> Result<StDomain>`: performs GET request to `/v1/domain/{domain}` to get detailed information about the domain. Parses response: `registrant`, `creation_date`, `expiry_date`, `nameservers`, `whois_email`. Implements `get_subdomains(domain: &str) -> Result<Vec<String>>`: performs GET request to `/v1/domain/{domain}/subdomains` to get all known subdomains. Implements `get_dns_history(domain: &str) -> Result<StDnsHistory>`: performs GET request to `/v1/domain/{domain}/history/{record_type}` (for A, AAAA, MX, NS, TXT records). Parses response: `records` (list of records with timestamps), `total` (total records). Implements `get_ssl_history(domain: &str)` to get SSL certificate history. Implements `get_whois_history(domain: &str)` to get WHOIS ownership change history. Implements `get_related_domains(domain: &str)` to find related domains (same owner, similar name, same IP).

---

#### **7. `crtsh_wrapper.rs` (Rust)**

**Task:** Connect the system with crt.sh (Certificate Transparency Log search) for certificate discovery.

**Purpose:** This file is a wrapper for the crt.sh API that provides certificate transparency log searching. Struct `CrtshWrapper` with fields: `client: Client`, `base_url: String`. Implements `search(domain: &str) -> Result<Vec<CrtshCert>>`: performs GET request to `/?q={domain}&output=json` to search all certificates issued for the domain. Parses response: `name_value` (domain/subdomain), `issuer_name` (CA issuer name), `not_before` (validity start date), `not_after` (expiry date), `serial_number`. Implements `search_by_hash(hash: &str) -> Result<Vec<CrtshCert>>`: performs GET request to `/?q={hash}` to search certificates by hash. Implements `search_by_wildcard(domain: &str) -> Result<Vec<CrtshCert>>`: performs GET request to `/?q=*.{domain}` to search wildcard certificates for the domain. Implements `get_certificate_details(cert_hash: &str)` to get complete certificate details from hash. Implements `get_domain_certificates(domain: &str)` to get all certificates associated with the domain (including subdomains and wildcards). Implements `find_subdomains_by_cert(domain: &str)` to find subdomains from certificate transparency logs (DNS names in certificates).

---

#### **8. `dnsdb_client.rs` (Rust)**

**Task:** Connect the system with DNSDB (DNS Database) for historical DNS data.

**Purpose:** This file is a client for the DNSDB API that provides historical DNS data. Struct `DnsdbClient` with fields: `client: Client`, `api_key: String`, `base_url: String`. Implements `get_rrsets(domain: &str) -> Result<Vec<DnsdbRrset>>`: performs GET request to `/dnsdb/v2/rrsets?name={domain}` to get DNS records for the domain. Implements `get_rrset_history(domain: &str, rr_type: &str) -> Result<Vec<DnsdbRrset>>`: performs GET request to `/dnsdb/v2/rrsets?name={domain}&rrtype={rr_type}&history=1` to get historical records of a specific type. Parses response: `rrname` (record name), `rrtype` (record type), `rdata` (record data), `count` (observation count), `time_first` (first seen), `time_last` (last seen). Implements `get_zone(zone: &str) -> Result<Vec<DnsdbRrset>>`: performs GET request to `/dnsdb/v2/zones/{zone}` to get all records in a zone. Implements `get_passive_dns(domain: &str)` to get passive DNS data (DNS observations from various sources). Implements `find_related_domains(ip: &str)` to find domains associated with an IP based on DNS records.

---

#### **9. `greyhat_warfare.rs` (Rust)**

**Task:** Connect the system with GreyHat Warfare for fast threat intelligence access.

**Purpose:** This file is an integration with GreyHat Warfare that provides fast threat intelligence access. Struct `GreyhatWarfare` with fields: `client: Client`, `api_key: String`, `base_url: String`. Implements `search(query: &str) -> Result<Vec<GhResult>>`: performs GET request to `/search?q={query}` to perform fast search. Parses response: `domain`, `ip`, `registrant`, `creation_date`, `expiry_date`, `nameservers`, `tags`. Implements `get_domain(domain: &str) -> Result<GhDomain>`: performs GET request to `/domain/{domain}` to get detailed information about the domain. Parses response: `registrant` (registrant name), `creation_date` (registration date), `expiry_date` (expiry date), `nameservers` (nameservers), `ip` (IP address), `asn` (ASN). Implements `get_ip(ip: &str) -> Result<GhIp>`: performs GET request to `/ip/{ip}` to get information about the IP. Parses response: `domains` (list of domains on IP), `asn` (ASN), `organization` (organization), `location` (geolocation). Implements `get_bulk_domains(domains: Vec<String>)` to get bulk information for multiple domains. Implements `get_recent_domains()` to get recently discovered domains.

---

**indonesian:**

---

## 📂 **`integration/` - INTEGRASI LAYANAN**

---

#### **1. `shodan_wrapper.rs` (Rust)**

**Tugas:** Menghubungkan sistem dengan Shodan API untuk mendapatkan network intelligence.

**Tujuan:** File ini adalah wrapper untuk Shodan API yang memungkinkan sistem mengakses data network intelligence dari Shodan search engine. Struct `ShodanWrapper` dengan fields: `client: Client` (HTTP client dengan connection pooling), `api_key: String` (API key untuk autentikasi), `base_url: String` (base URL Shodan API). Implementasi `search_ip(ip: &str) -> Result<ShodanHost>`: melakukan GET request ke `/shodan/host/{ip}` untuk mendapatkan informasi detail tentang IP target. Parse response: `hostnames` (daftar hostname yang terkait), `ports` (daftar port yang terbuka), `os` (operating system yang terdeteksi), `data` (banner lengkap dari service yang berjalan). Implementasi `search_domain(domain: &str) -> Result<Vec<ShodanHost>>`: melakukan GET request ke `/shodan/host/search?query=hostname:{domain}` untuk mencari semua host yang terkait dengan domain. Implementasi `search(query: &str) -> Result<Vec<ShodanResult>>`: melakukan GET request ke `/shodan/host/search?query={query}` untuk melakukan pencarian bebas. Rate limit: 1 request per second untuk free tier, implementasi token bucket untuk menghormati rate limit. Implementasi `get_service_banner(ip: &str, port: u16)` untuk mendapatkan banner spesifik dari service di port tertentu. Implementasi `get_geo_location(ip: &str)` untuk mendapatkan informasi geolokasi IP.

---

#### **2. `censys_connector.rs` (Rust)**

**Tugas:** Menghubungkan sistem dengan Censys API untuk mendapatkan internet-wide scanning data.

**Tujuan:** File ini adalah konektor untuk Censys API yang menyediakan data scanning internet-wide. Struct `CensysConnector` dengan fields: `client: Client`, `api_id: String` (API ID untuk autentikasi), `api_secret: String` (API secret), `base_url: String`. Implementasi `get_ip(ip: &str) -> Result<CensysIp>`: melakukan GET request ke `/v2/hosts/{ip}` untuk mendapatkan informasi lengkap tentang IP. Parse response: `location` (negara, kota, koordinat), `services` (daftar service yang berjalan dengan banner), `autonomous_system` (ASN dan organisasi). Implementasi `get_certificate(cert_hash: &str) -> Result<CensysCert>`: melakukan GET request ke `/v2/certificates/{cert_hash}` untuk mendapatkan informasi tentang sertifikat SSL/TLS. Parse response: `fingerprint_sha256`, `subject_dn`, `issuer_dn`, `validity` (start, end), `subject_alt_names`. Implementasi `search(query: &str) -> Result<Vec<CensysHost>>`: melakukan POST request ke `/v2/hosts/search` dengan body `{"q": query}` untuk mencari host berdasarkan query. Implementasi `get_certificate_history(domain: &str)` untuk mendapatkan history sertifikat dari domain. Implementasi `get_domain_report(domain: &str)` untuk mendapatkan laporan lengkap tentang domain.

---

#### **3. `virustotal_adapter.rs` (Rust)**

**Tugas:** Menghubungkan sistem dengan VirusTotal API untuk malware scanning dan URL analysis.

**Tujuan:** File ini adalah adapter untuk VirusTotal API yang menyediakan malware scanning capabilities. Struct `VirusTotalAdapter` dengan fields: `client: Client`, `api_key: String`, `base_url: String`. Implementasi `scan_url(url: &str) -> Result<VtUrlScan>`: melakukan POST request ke `/urls` dengan URL untuk submit URL scanning, kemudian GET request ke `/analyses/{id}` untuk mendapatkan hasil scanning. Parse response: `malicious` (jumlah deteksi malicious), `suspicious`, `harmless`, `undetected`. Implementasi `get_domain_report(domain: &str) -> Result<VtDomain>`: melakukan GET request ke `/domains/{domain}` untuk mendapatkan informasi lengkap tentang domain. Parse response: `last_analysis_stats` (malicious, suspicious, harmless, undetected), `whois` (informasi WHOIS), `last_http_response` (response terakhir), `categories` (kategorisasi domain). Implementasi `get_ip_report(ip: &str) -> Result<VtIp>`: melakukan GET request ke `/ip_addresses/{ip}` untuk mendapatkan informasi tentang IP. Implementasi `scan_file(file_data: &[u8]) -> Result<VtFileScan>`: upload file (max 32MB) untuk scanning malware. Implementasi `get_file_report(hash: &str)` untuk mendapatkan report dari file hash.

---

#### **4. `alienvault_otx.rs` (Rust)**

**Tugas:** Menghubungkan sistem dengan AlienVault OTX (Open Threat Exchange) untuk threat intelligence.

**Tujuan:** File ini adalah integration dengan AlienVault OTX yang menyediakan open threat intelligence exchange. Struct `AlienvaultOtx` dengan fields: `client: Client`, `api_key: String`, `base_url: String`. Implementasi `get_domain_pulses(domain: &str) -> Result<Vec<Pulse>>`: melakukan GET request ke `/api/v1/indicators/domain/{domain}/pulses` untuk mendapatkan pulses (threat intelligence reports) yang terkait dengan domain. Parse response: `pulse_id`, `name`, `description`, `created`, `modified`, `tags`, `references`. Implementasi `get_ip_pulses(ip: &str) -> Result<Vec<Pulse>>`: melakukan GET request ke `/api/v1/indicators/IPv4/{ip}/pulses` untuk mendapatkan pulses yang terkait dengan IP. Implementasi `get_domain_general(domain: &str) -> Result<OtxIndicator>`: melakukan GET request ke `/api/v1/indicators/domain/{domain}/general` untuk mendapatkan informasi umum tentang domain. Parse response: `whois` (informasi WHOIS terstruktur), `geo` (geolokasi), `url_list` (daftar URL yang terkait), `passive_dns` (passive DNS records). Implementasi `get_indicator_by_type(indicator_type: &str, value: &str)` untuk mendapatkan indicator berdasarkan tipe (domain, IP, URL, hash). Implementasi `get_pulse_details(pulse_id: &str)` untuk mendapatkan detail lengkap dari pulse tertentu.

---

#### **5. `urlscan_integration.rs` (Rust)**

**Tugas:** Menghubungkan sistem dengan URLScan API untuk website behavior analysis.

**Tujuan:** File ini adalah integration dengan URLScan API yang menganalisis perilaku website. Struct `UrlscanIntegration` dengan fields: `client: Client`, `api_key: Option<String>` (optional untuk rate limit yang lebih tinggi), `base_url: String`. Implementasi `submit_scan(url: &str) -> Result<UrlscanSubmission>`: melakukan POST request ke `/scan` dengan body `{"url": url, "public": "off"}` untuk submit URL scanning private (hasil hanya bisa dilihat dengan API key). Parse response: `uuid` (ID submission), `api` (URL untuk hasil), `visibility` (public/private). Implementasi `get_result(uuid: &str) -> Result<UrlscanResult>`: melakukan GET request ke `/result/{uuid}` untuk mendapatkan hasil scanning. Parse response: `screenshot_url` (URL screenshot), `dom` (DOM snapshot), `verdicts` (malicious, phishing, suspicious), `links` (external/internal links), `pages` (halaman yang ditemukan). Implementasi `search(query: &str) -> Result<Vec<UrlscanSubmission>>`: melakukan GET request ke `/search/?q={query}` untuk mencari submission sebelumnya. Implementasi `get_domain_report(domain: &str)` untuk mendapatkan laporan lengkap tentang domain. Implementasi `get_ip_report(ip: &str)` untuk mendapatkan laporan tentang IP.

---

#### **6. `securitytrails_client.rs` (Rust)**

**Tugas:** Menghubungkan sistem dengan SecurityTrails API untuk DNS history dan domain intelligence.

**Tujuan:** File ini adalah client untuk SecurityTrails API yang menyediakan DNS history dan domain intelligence. Struct `SecurityTrailsClient` dengan fields: `client: Client`, `api_key: String`, `base_url: String`. Implementasi `get_domain_details(domain: &str) -> Result<StDomain>`: melakukan GET request ke `/v1/domain/{domain}` untuk mendapatkan informasi detail tentang domain. Parse response: `registrant`, `creation_date`, `expiry_date`, `nameservers`, `whois_email`. Implementasi `get_subdomains(domain: &str) -> Result<Vec<String>>`: melakukan GET request ke `/v1/domain/{domain}/subdomains` untuk mendapatkan semua subdomain yang diketahui. Implementasi `get_dns_history(domain: &str) -> Result<StDnsHistory>`: melakukan GET request ke `/v1/domain/{domain}/history/{record_type}` (untuk A, AAAA, MX, NS, TXT records). Parse response: `records` (list records dengan timestamp), `total` (total records). Implementasi `get_ssl_history(domain: &str)` untuk mendapatkan history sertifikat SSL. Implementasi `get_whois_history(domain: &str)` untuk mendapatkan history WHOIS perubahan kepemilikan. Implementasi `get_related_domains(domain: &str)` untuk menemukan domain yang terkait (sama pemilik, similar name, same IP).

---

#### **7. `crtsh_wrapper.rs` (Rust)**

**Tugas:** Menghubungkan sistem dengan crt.sh (Certificate Transparency Log search) untuk certificate discovery.

**Tujuan:** File ini adalah wrapper untuk crt.sh API yang menyediakan pencarian certificate transparency logs. Struct `CrtshWrapper` dengan fields: `client: Client`, `base_url: String`. Implementasi `search(domain: &str) -> Result<Vec<CrtshCert>>`: melakukan GET request ke `/?q={domain}&output=json` untuk mencari semua sertifikat yang diterbitkan untuk domain. Parse response: `name_value` (domain/subdomain), `issuer_name` (nama issuer CA), `not_before` (tanggal mulai valid), `not_after` (tanggal kadaluarsa), `serial_number`. Implementasi `search_by_hash(hash: &str) -> Result<Vec<CrtshCert>>`: melakukan GET request ke `/?q={hash}` untuk mencari sertifikat berdasarkan hash. Implementasi `search_by_wildcard(domain: &str) -> Result<Vec<CrtshCert>>`: melakukan GET request ke `/?q=*.{domain}` untuk mencari wildcard certificate untuk domain. Implementasi `get_certificate_details(cert_hash: &str)` untuk mendapatkan detail lengkap sertifikat dari hash. Implementasi `get_domain_certificates(domain: &str)` untuk mendapatkan semua sertifikat yang terkait dengan domain (termasuk subdomain dan wildcard). Implementasi `find_subdomains_by_cert(domain: &str)` untuk menemukan subdomain dari certificate transparency logs (DNS names dalam certificate).

---

#### **8. `dnsdb_client.rs` (Rust)**

**Tugas:** Menghubungkan sistem dengan DNSDB (DNS Database) untuk historical DNS data.

**Tujuan:** File ini adalah client untuk DNSDB API yang menyediakan historical DNS data. Struct `DnsdbClient` dengan fields: `client: Client`, `api_key: String`, `base_url: String`. Implementasi `get_rrsets(domain: &str) -> Result<Vec<DnsdbRrset>>`: melakukan GET request ke `/dnsdb/v2/rrsets?name={domain}` untuk mendapatkan DNS records dari domain. Implementasi `get_rrset_history(domain: &str, rr_type: &str) -> Result<Vec<DnsdbRrset>>`: melakukan GET request ke `/dnsdb/v2/rrsets?name={domain}&rrtype={rr_type}&history=1` untuk mendapatkan historical records dari tipe tertentu. Parse response: `rrname` (nama record), `rrtype` (tipe record), `rdata` (data record), `count` (jumlah observasi), `time_first` (first seen), `time_last` (last seen). Implementasi `get_zone(zone: &str) -> Result<Vec<DnsdbRrset>>`: melakukan GET request ke `/dnsdb/v2/zones/{zone}` untuk mendapatkan semua records dalam zone. Implementasi `get_passive_dns(domain: &str)` untuk mendapatkan passive DNS data (observasi DNS dari berbagai sumber). Implementasi `find_related_domains(ip: &str)` untuk menemukan domain yang terkait dengan IP berdasarkan DNS records.

---

#### **9. `greyhat_warfare.rs` (Rust)**

**Tugas:** Menghubungkan sistem dengan GreyHat Warfare untuk fast threat intelligence access.

**Tujuan:** File ini adalah integration dengan GreyHat Warfare yang menyediakan fast threat intelligence access. Struct `GreyhatWarfare` dengan fields: `client: Client`, `api_key: String`, `base_url: String`. Implementasi `search(query: &str) -> Result<Vec<GhResult>>`: melakukan GET request ke `/search?q={query}` untuk melakukan pencarian cepat. Parse response: `domain`, `ip`, `registrant`, `creation_date`, `expiry_date`, `nameservers`, `tags`. Implementasi `get_domain(domain: &str) -> Result<GhDomain>`: melakukan GET request ke `/domain/{domain}` untuk mendapatkan informasi detail tentang domain. Parse response: `registrant` (nama registrant), `creation_date` (tanggal registrasi), `expiry_date` (tanggal kadaluarsa), `nameservers` (nameserver), `ip` (IP address), `asn` (ASN). Implementasi `get_ip(ip: &str) -> Result<GhIp>`: melakukan GET request ke `/ip/{ip}` untuk mendapatkan informasi tentang IP. Parse response: `domains` (daftar domain di IP), `asn` (ASN), `organization` (organisasi), `location` (geolokasi). Implementasi `get_bulk_domains(domains: Vec<String>)` untuk mendapatkan informasi bulk dari multiple domains. Implementasi `get_recent_domains()` untuk mendapatkan domain terbaru yang ditemukan.

---

**deployment folder explanation:**

---

## 📂 **`deployment/` - DEPLOYMENT**

---

#### **1. `dockerfile`**

**Task:** Build Docker image for containerizing the IWS application with multi-stage build.

**Purpose:** This file defines the Docker image build process with a multi-stage approach to optimize size and security. Stage 1 - Builder: FROM `rust:1.75-slim` to compile Rust components with release optimizations, FROM `golang:1.21-alpine` to compile Go components, FROM `python:3.11-slim` to install Python dependencies. Copies `Cargo.toml`, `go.mod`, `requirements.txt` into the container. Runs `cargo build --release` to build Rust components, `go build -o iws` to build Go components, `pip install -r requirements.txt` for Python dependency installation. Stage 2 - Runtime: FROM `python:3.11-slim` as the final smaller base image. Copies all binaries from builder stage (`/app/target/release/`, `/app/iws`, `/app/.venv/`). Copies all Python files (`iws.py`, `main.py`, `core/`, `modules/`, `agents/`, `models/`, `storage/`, `utils/`, `config/`, `database/`, `api/`, `reports/`, `terminal/`, `integration/`). Exposes port 8080 for HTTP API. Healthcheck: `CMD ["curl", "-f", "http://localhost:8080/health"]` for monitoring container health. CMD `["python", "iws.py", "--mode", "production"]` to run the application. Implements non-root user (`useradd -m -u 1000 iws`) for container security.

---

#### **2. `docker-compose.yml`**

**Task:** Orchestrate multiple containers to run the entire IWS stack.

**Purpose:** This file defines all services needed to run IWS in a containerized environment. Services: `iws` (build: .) - main application with port mapping 8080:8080, depends_on `postgres` and `redis`. `postgres` (image: postgres:15) - PostgreSQL database with environment: `POSTGRES_USER=iws`, `POSTGRES_PASSWORD=${DB_PASSWORD}`, `POSTGRES_DB=iws`, volume `postgres_data:/var/lib/postgresql/data`, healthcheck with `pg_isready -U iws`. `redis` (image: redis:7-alpine) - Redis cache with command `redis-server --appendonly yes`, volume `redis_data:/data`, healthcheck with `redis-cli ping`. `nginx` (image: nginx:alpine) - reverse proxy with volume `./nginx_config.conf:/etc/nginx/conf.d/default.conf`, depends_on `iws`, ports 80:80 and 443:443. Networks: `iws-network` (bridge network for inter-container communication). Volumes: `postgres_data`, `redis_data`, `iws_data` (for persistent data). Environment variables from `.env` file using `env_file: .env`. Restart policy: `unless-stopped` for all services.

---

#### **3. `kubernetes_deployment.yaml`**

**Task:** Define manifests for deploying IWS in a Kubernetes cluster.

**Purpose:** This file contains all Kubernetes manifests for production-ready deployment. `Deployment` for IWS application: `replicas=3` for high availability, container image `iws:latest` (pull policy `Always`), env from `ConfigMap` (for non-sensitive config) and `Secrets` (for sensitive data). Resources: requests `cpu=100m, memory=256Mi`, limits `cpu=500m, memory=1Gi`. `livenessProbe`: `httpGet /health` every 10s, `initialDelaySeconds=30`, `failureThreshold=3`. `readinessProbe`: `httpGet /ready` every 5s, `initialDelaySeconds=10`, `failureThreshold=3`. `Service`: type=ClusterIP (internal), port=8080, selector `app=iws`. `Ingress`: hostname `iws.example.com`, tls secret `iws-tls` for HTTPS. `HorizontalPodAutoscaler (HPA)`: minReplicas=3, maxReplicas=10, targetCPUUtilizationPercentage=70, targetMemoryUtilizationPercentage=80. `ConfigMap`: for non-sensitive configuration (`LOG_LEVEL=info`, `MAX_THREADS=50`, `SCANNING_PROFILE=moderate`). `Secret`: for sensitive data (database password, API keys). `PersistentVolumeClaim`: for persistent data storage (10Gi). `ServiceAccount`: with minimal permissions for operation.

---

#### **4. `nginx_config.conf`**

**Task:** Configure Nginx as reverse proxy, load balancer, and SSL terminator.

**Purpose:** This file defines Nginx configuration for managing HTTP/HTTPS traffic. `upstream` block: `upstream iws_backend { server iws:8080; }` for load balancing to backend. `server` block port 80: `listen 80`, `return 301 https://$host$request_uri` to redirect HTTP to HTTPS. `server` block port 443: `listen 443 ssl http2`, `ssl_certificate /etc/nginx/ssl/tls.crt`, `ssl_certificate_key /etc/nginx/ssl/tls.key` for SSL termination. `ssl_protocols TLSv1.2 TLSv1.3`, `ssl_ciphers HIGH:!aNULL:!MD5` for secure cipher suites. `location /`: `proxy_pass http://iws_backend`, `proxy_set_header Host $host`, `proxy_set_header X-Real-IP $remote_addr`, `proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for`, `proxy_set_header X-Forwarded-Proto $scheme`. `location /ws/`: for WebSocket support, `proxy_http_version 1.1`, `proxy_set_header Upgrade $http_upgrade`, `proxy_set_header Connection "upgrade"`. `location /static/`: `alias /usr/share/nginx/html/static/` for static files. Rate limiting: `limit_req_zone $binary_remote_addr zone=api_limit:10m rate=10r/s`, `limit_req zone=api_limit burst=20 nodelay`. `client_max_body_size 50M` to allow large file uploads.

---

#### **5. `systemd_service.rs` (Rust)**

**Task:** Generate systemd service unit to run IWS as a service on Linux.

**Purpose:** This file implements a systemd service unit generator. Template unit: `[Unit]` Description="IWS Intelligence Website Scanning System", After="network.target" (ensure network is available), Wants="network-online.target". `[Service]` Type="simple" (main process), User="iws" (dedicated user), Group="iws", WorkingDirectory="/opt/iws" (working directory), ExecStart="/usr/bin/python3 /opt/iws/iws.py --mode production" (command to run), Restart="always" (restart if crash), RestartSec="10" (10 second delay before restart), TimeoutStopSec="30" (30 second shutdown timeout), EnvironmentFile="/etc/iws/env" (environment variables), LimitNOFILE="65535" (maximum file descriptors). `[Install]` WantedBy="multi-user.target" (start at multi-user runlevel). Implements `generate_service_file()` to write unit file to `/etc/systemd/system/iws.service`. Implements `enable_service()` to run `systemctl enable iws`. Implements `start_service()` to run `systemctl start iws`. Implements `status_service()` to check status with `systemctl status iws`. Implements `logs_service()` to view logs with `journalctl -u iws -f`.

---

#### **6. `install.sh`**

**Task:** Automate the installation process of IWS on various operating systems.

**Purpose:** This file is an installation script that automates the entire installation process. Step 1: Check prerequisites - checks for `curl`, `wget`, `git`, `python3` (>=3.11), `pip3`, `cargo` (>=1.75), `go` (>=1.21). Installs missing ones using package manager (apt, yum, pacman). Step 2: Clone repository from Git (`git clone https://github.com/iws/iws.git /opt/iws`). Step 3: Create virtual environment (`python3 -m venv /opt/iws/.venv`). Step 4: Install Python dependencies (`source /opt/iws/.venv/bin/activate && pip install -r requirements.txt`). Step 5: Build Rust components (`cd /opt/iws && cargo build --release`). Step 6: Build Go components (`cd /opt/iws && go build -o iws`). Step 7: Setup configuration (copy `.env_template` to `.env`, prompt user to fill API keys). Step 8: Initialize database (`python3 scripts/init_database.py`). Step 9: Create systemd service (run `systemd_service.rs` generator). Step 10: Start service (`systemctl start iws` and `systemctl enable iws`). Step 11: Verify installation (`curl http://localhost:8080/health`). Supports uninstall (`--remove` flag): stop and disable service, delete directory, remove user. Supports upgrade (`--upgrade` flag): pull latest code, rebuild, restart service. Supports `--help` option to display all options.

---

**indonesian:**

---

## 📂 **`deployment/` - DEPLOYMENT**

---

#### **1. `dockerfile`**

**Tugas:** Membangun Docker image untuk containerisasi aplikasi IWS dengan multi-stage build.

**Tujuan:** File ini mendefinisikan proses build Docker image dengan pendekatan multi-stage untuk mengoptimalkan ukuran dan keamanan. Stage 1 - Builder: FROM `rust:1.75-slim` untuk mengkompilasi komponen Rust dengan optimasi release, FROM `golang:1.21-alpine` untuk mengkompilasi komponen Go, FROM `python:3.11-slim` untuk menginstal dependensi Python. Copy `Cargo.toml`, `go.mod`, `requirements.txt` ke dalam container. Jalankan `cargo build --release` untuk build Rust components, `go build -o iws` untuk build Go components, `pip install -r requirements.txt` untuk instalasi dependensi Python. Stage 2 - Runtime: FROM `python:3.11-slim` sebagai base image final yang lebih kecil. Copy semua binary dari stage builder (`/app/target/release/`, `/app/iws`, `/app/.venv/`). Copy semua file Python (`iws.py`, `main.py`, `core/`, `modules/`, `agents/`, `models/`, `storage/`, `utils/`, `config/`, `database/`, `api/`, `reports/`, `terminal/`, `integration/`). Expose port 8080 untuk HTTP API. Healthcheck: `CMD ["curl", "-f", "http://localhost:8080/health"]` untuk monitoring container health. CMD `["python", "iws.py", "--mode", "production"]` untuk menjalankan aplikasi. Implementasi non-root user (`useradd -m -u 1000 iws`) untuk keamanan container.

---

#### **2. `docker-compose.yml`**

**Tugas:** Mengorkestrasi multiple containers untuk menjalankan seluruh stack IWS.

**Tujuan:** File ini mendefinisikan semua services yang dibutuhkan untuk menjalankan IWS dalam environment containerized. Services: `iws` (build: .) - aplikasi utama dengan port mapping 8080:8080, depends_on `postgres` dan `redis`. `postgres` (image: postgres:15) - database PostgreSQL dengan environment: `POSTGRES_USER=iws`, `POSTGRES_PASSWORD=${DB_PASSWORD}`, `POSTGRES_DB=iws`, volume `postgres_data:/var/lib/postgresql/data`, healthcheck dengan `pg_isready -U iws`. `redis` (image: redis:7-alpine) - Redis cache dengan command `redis-server --appendonly yes`, volume `redis_data:/data`, healthcheck dengan `redis-cli ping`. `nginx` (image: nginx:alpine) - reverse proxy dengan volume `./nginx_config.conf:/etc/nginx/conf.d/default.conf`, depends_on `iws`, ports 80:80 dan 443:443. Networks: `iws-network` (bridge network untuk komunikasi antar container). Volumes: `postgres_data`, `redis_data`, `iws_data` (untuk persistent data). Environment variables dari file `.env` menggunakan `env_file: .env`. Restart policy: `unless-stopped` untuk semua services.

---

#### **3. `kubernetes_deployment.yaml`**

**Tugas:** Mendefinisikan manifest untuk deployment IWS di Kubernetes cluster.

**Tujuan:** File ini berisi semua manifest Kubernetes untuk deployment production-ready. `Deployment` untuk aplikasi IWS: `replicas=3` untuk high availability, container image `iws:latest` (pull policy `Always`), env from `ConfigMap` (untuk non-sensitive config) dan `Secrets` (untuk sensitive data). Resources: requests `cpu=100m, memory=256Mi`, limits `cpu=500m, memory=1Gi`. `livenessProbe`: `httpGet /health` setiap 10s, `initialDelaySeconds=30`, `failureThreshold=3`. `readinessProbe`: `httpGet /ready` setiap 5s, `initialDelaySeconds=10`, `failureThreshold=3`. `Service`: type=ClusterIP (internal), port=8080, selector `app=iws`. `Ingress`: hostname `iws.example.com`, tls secret `iws-tls` untuk HTTPS. `HorizontalPodAutoscaler (HPA)`: minReplicas=3, maxReplicas=10, targetCPUUtilizationPercentage=70, targetMemoryUtilizationPercentage=80. `ConfigMap`: untuk konfigurasi non-sensitive (`LOG_LEVEL=info`, `MAX_THREADS=50`, `SCANNING_PROFILE=moderate`). `Secret`: untuk sensitive data (database password, API keys). `PersistentVolumeClaim`: untuk persistent data storage (10Gi). `ServiceAccount`: dengan minimal permissions untuk operasi.

---

#### **4. `nginx_config.conf`**

**Tugas:** Mengkonfigurasi Nginx sebagai reverse proxy, load balancer, dan SSL terminator.

**Tujuan:** File ini mendefinisikan konfigurasi Nginx untuk mengelola traffic HTTP/HTTPS. `upstream` block: `upstream iws_backend { server iws:8080; }` untuk load balancing ke backend. `server` block port 80: `listen 80`, `return 301 https://$host$request_uri` untuk redirect HTTP ke HTTPS. `server` block port 443: `listen 443 ssl http2`, `ssl_certificate /etc/nginx/ssl/tls.crt`, `ssl_certificate_key /etc/nginx/ssl/tls.key` untuk SSL termination. `ssl_protocols TLSv1.2 TLSv1.3`, `ssl_ciphers HIGH:!aNULL:!MD5` untuk cipher suite yang aman. `location /`: `proxy_pass http://iws_backend`, `proxy_set_header Host $host`, `proxy_set_header X-Real-IP $remote_addr`, `proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for`, `proxy_set_header X-Forwarded-Proto $scheme`. `location /ws/`: untuk WebSocket support, `proxy_http_version 1.1`, `proxy_set_header Upgrade $http_upgrade`, `proxy_set_header Connection "upgrade"`. `location /static/`: `alias /usr/share/nginx/html/static/` untuk static files. Rate limiting: `limit_req_zone $binary_remote_addr zone=api_limit:10m rate=10r/s`, `limit_req zone=api_limit burst=20 nodelay`. `client_max_body_size 50M` untuk mengizinkan upload file besar.

---

#### **5. `systemd_service.rs` (Rust)**

**Tugas:** Menghasilkan systemd service unit untuk menjalankan IWS sebagai service di Linux.

**Tujuan:** File ini mengimplementasikan generator systemd service unit. Template unit: `[Unit]` Description="IWS Intelligence Website Scanning System", After="network.target" (pastikan network tersedia), Wants="network-online.target". `[Service]` Type="simple" (proses utama), User="iws" (user khusus), Group="iws", WorkingDirectory="/opt/iws" (directory kerja), ExecStart="/usr/bin/python3 /opt/iws/iws.py --mode production" (command untuk menjalankan), Restart="always" (restart jika crash), RestartSec="10" (delay 10 detik sebelum restart), TimeoutStopSec="30" (timeout shutdown 30 detik), EnvironmentFile="/etc/iws/env" (environment variables), LimitNOFILE="65535" (maksimum file descriptors). `[Install]` WantedBy="multi-user.target" (start pada multi-user runlevel). Implementasi `generate_service_file()` untuk menulis file unit ke `/etc/systemd/system/iws.service`. Implementasi `enable_service()` untuk menjalankan `systemctl enable iws`. Implementasi `start_service()` untuk menjalankan `systemctl start iws`. Implementasi `status_service()` untuk mengecek status dengan `systemctl status iws`. Implementasi `logs_service()` untuk melihat logs dengan `journalctl -u iws -f`.

---

#### **6. `install.sh`**

**Tugas:** Mengotomatiskan proses instalasi IWS di berbagai sistem operasi.

**Tujuan:** File ini adalah installation script yang mengotomatiskan seluruh proses instalasi. Step 1: Check prerequisites - periksa keberadaan `curl`, `wget`, `git`, `python3` (>=3.11), `pip3`, `cargo` (>=1.75), `go` (>=1.21). Instal yang kurang menggunakan package manager (apt, yum, pacman). Step 2: Clone repository dari Git (`git clone https://github.com/iws/iws.git /opt/iws`). Step 3: Create virtual environment (`python3 -m venv /opt/iws/.venv`). Step 4: Install Python dependencies (`source /opt/iws/.venv/bin/activate && pip install -r requirements.txt`). Step 5: Build Rust components (`cd /opt/iws && cargo build --release`). Step 6: Build Go components (`cd /opt/iws && go build -o iws`). Step 7: Setup configuration (copy `.env_template` to `.env`, prompt user untuk mengisi API keys). Step 8: Initialize database (`python3 scripts/init_database.py`). Step 9: Create systemd service (jalankan `systemd_service.rs` generator). Step 10: Start service (`systemctl start iws` dan `systemctl enable iws`). Step 11: Verify installation (`curl http://localhost:8080/health`). Support untuk uninstall (`--remove` flag): stop dan disable service, hapus direktori, remove user. Support upgrade (`--upgrade` flag): pull latest code, rebuild, restart service. Support untuk opsi `--help` untuk menampilkan semua opsi.

---

**tests folder explanation:**

---

## 📂 **`tests/` - TESTING**

---

#### **1. `test_scanner.rs` (Rust)**

**Task:** Test all scanner module functionality to ensure accuracy and reliability.

**Purpose:** This file contains unit tests for the scanner module that verify every aspect of scanning. Implements `#[cfg(test)]` to mark test-only code. Mock HTTP client using `wiremock` to simulate server responses without making real requests. Test `DNS enumeration`: mock DNS resolver with known response records, verify that `dns_enum.rs` returns correct records (A, AAAA, CNAME, MX, TXT, NS, SOA, SRV). Test `port scanning`: mock TCP connections with simulated responses, verify that open ports are correctly detected and service detection works. Test `SSL analysis`: mock certificate responses (self-signed, expired, valid, with weak ciphers), verify that analyzer correctly detects issues. Test `timeout handling`: mock slow responses to ensure timeout mechanism works and does not hang. Test `error handling`: simulate connection failures, DNS resolution failures, invalid URLs, verify that errors are returned correctly and do not crash. Test `concurrent scanning`: run multiple scans concurrently, verify that there are no race conditions and all scans complete correctly. Test `cancel_scan()`: start a scan then cancel it, verify that the scan stops with graceful shutdown.

---

#### **2. `test_analyzer.rs` (Rust)**

**Task:** Test all analyzer module functionality to ensure analysis accuracy.

**Purpose:** This file contains unit tests for the analyzer module with sample data that has known results. Test `CVE matching`: provide sample data with vulnerable software versions (Apache 2.4.49), verify that CVE-2021-41773 is found with correct CVSS score. Test `risk scoring`: provide sample vulnerabilities with known CVSS vectors, verify that risk score is correctly calculated using CVSS v3.1 formula. Test `pattern detection`: provide text containing malicious patterns (XSS payloads, SQL injection patterns), verify that pattern recognizer finds all patterns. Test `correlation analysis`: provide data from multiple modules (port 3306 open + MySQL version 5.7 + CVE database), verify that cross-reference correlates findings and produces appropriate warnings. Test `anomaly detection`: provide dataset with outliers, verify that anomaly detector correctly identifies outliers. Test `performance`: verify that analyzer completes analysis within 5 minutes for a website with 100 pages (meeting performance contract).

---

#### **3. `test_integration.rs` (Rust)**

**Task:** Test integration of all modules working together properly in an end-to-end manner.

**Purpose:** This file contains integration tests that test the entire system from start to finish. Implements `testcontainers` to spin up test environment automatically (PostgreSQL container, Redis container, Nginx container). Database is populated with sample data (100 test domains, 1000 vulnerabilities) to ensure data availability. Test `end-to-end scanning`: run full scan on test website (`httpbin` local or mock server), verify that all modules run successfully and produce complete data. Test `scan -> analyze -> report -> export` pipeline: after scan completes, run analysis, generate reports in all formats (JSON, TXT, DOCS, CSV, HTML, PDF), verify that all reports are created correctly with no errors. Test `API integration`: run REST API server, send requests to all endpoints (POST /scan, GET /status, GET /report, POST /analyze, GET /export), verify correct responses and appropriate status codes. Test `error recovery`: simulate network failures (using mock that fails), verify that system retries with exponential backoff and does not crash. Test `data persistence`: verify that data is correctly stored in database and can be retrieved after restart. Verify output against **golden files** (files containing expected output that has been approved) to ensure result consistency.

---

#### **4. `test_security_modules.rs` (Rust)**

**Task:** Specifically test all security modules to ensure accurate vulnerability detection.

**Purpose:** This file contains specific tests for security modules with sample data containing vulnerabilities and non-vulnerable data. Test `XSS detection`: create sample pages with XSS vulnerabilities (reflected, stored, DOM-based), verify that XSS detector finds all vulnerabilities with appropriate confidence levels. Test with XSS payload list (100+ payloads) to ensure comprehensive detection. Test `SQL injection detection`: create sample forms with SQL injection vulnerabilities (boolean-based, time-based, error-based, union-based), verify that SQL injection detector finds all vulnerabilities with correct types. Test `CSRF detection`: create sample forms with and without CSRF tokens, with various SameSite cookie configurations, verify that CSRF analyzer correctly identifies vulnerable forms. Test `header analysis`: create sample responses with correct and incorrect security headers (HSTS, CSP, X-Frame-Options), verify that header analyzer gives appropriate scores. Test `cookie scanner`: create sample cookies with various configurations (Secure, HttpOnly, SameSite), verify that cookie scanner identifies insecure configurations.

---

#### **5. `test_agents.rs` (Rust)**

**Task:** Test all agent system functionality to ensure coordination and reliability.

**Purpose:** This file contains tests for the agent system that verify all aspects of agent lifecycle and communication. Test `agent lifecycle`: create agent, run `init()` -> `run()` -> `pause()` -> `resume()` -> `shutdown()`, verify that state transitions work correctly according to contract. Test `inter-agent communication`: create multiple agents, send messages between agents via message bus, verify that messages are received correctly and payload is not corrupted. Test `state persistence`: save agent state to storage, restart agent, verify that state is restored correctly. Test `error recovery`: simulate agent crash (panic), verify that supervisor detects missing heartbeat and restarts agent automatically. Test `monitoring agent`: create monitoring agent with simulated changes (new vulnerability, port change), verify that agent detects changes and generates alert with correct severity. Test `reconnaissance agent`: run reconnaissance agent on sample domain, verify that all stages (WHOIS, DNS, subdomains, technologies) complete with valid results. Test `reporting agent`: run reporting agent with sample data, verify that reports are generated in all supported formats. Test `concurrent agents`: run 10 agents simultaneously, verify that there are no race conditions and all agents complete successfully.

---

**indonesian:**

---

## 📂 **`tests/` - TESTING**

---

#### **1. `test_scanner.rs` (Rust)**

**Tugas:** Menguji semua fungsionalitas scanner module untuk memastikan akurasi dan reliability.

**Tujuan:** File ini berisi unit tests untuk scanner module yang memverifikasi setiap aspek scanning. Implementasi `#[cfg(test)]` untuk menandai test-only code. Mock HTTP client menggunakan `wiremock` untuk mensimulasikan response server tanpa melakukan request nyata. Test `DNS enumeration`: mock DNS resolver dengan response records yang telah diketahui, verifikasi bahwa `dns_enum.rs` mengembalikan records yang benar (A, AAAA, CNAME, MX, TXT, NS, SOA, SRV). Test `port scanning`: mock TCP connections dengan simulated responses, verifikasi bahwa port yang terbuka terdeteksi dengan benar dan service detection bekerja. Test `SSL analysis`: mock certificate responses (self-signed, expired, valid, with weak ciphers), verifikasi bahwa analyzer mendeteksi masalah dengan benar. Test `timeout handling`: mock slow responses untuk memastikan timeout mechanism bekerja dan tidak menggantung. Test `error handling`: simulate connection failures, DNS resolution failures, invalid URLs, verifikasi bahwa error dikembalikan dengan benar dan tidak crash. Test `concurrent scanning`: jalankan multiple scans secara concurrent, verifikasi bahwa tidak ada race condition dan semua scan selesai dengan benar. Test `cancel_scan()`: memulai scan lalu membatalkannya, verifikasi bahwa scan berhenti dengan graceful shutdown.

---

#### **2. `test_analyzer.rs` (Rust)**

**Tugas:** Menguji semua fungsionalitas analyzer module untuk memastikan akurasi analisis.

**Tujuan:** File ini berisi unit tests untuk analyzer module dengan sample data yang sudah diketahui hasilnya. Test `CVE matching`: berikan sample data dengan versi software yang vulnerable (Apache 2.4.49), verifikasi bahwa CVE-2021-41773 ditemukan dengan CVSS score yang benar. Test `risk scoring`: berikan sample vulnerabilities dengan CVSS vectors yang diketahui, verifikasi bahwa risk score dihitung dengan benar menggunakan formula CVSS v3.1. Test `pattern detection`: berikan text yang mengandung malicious patterns (XSS payloads, SQL injection patterns), verifikasi bahwa pattern recognizer menemukan semua patterns. Test `correlation analysis`: berikan data dari multiple modules (port 3306 terbuka + MySQL version 5.7 + CVE database), verifikasi bahwa cross-reference menghubungkan temuan dan menghasilkan warning yang sesuai. Test `anomaly detection`: berikan dataset dengan outliers, verifikasi bahwa anomaly detector mengidentifikasi outliers dengan benar. Test `performance`: verifikasi bahwa analyzer menyelesaikan analisis dalam 5 menit untuk website dengan 100 halaman (sesuai performance contract).

---

#### **3. `test_integration.rs` (Rust)**

**Tugas:** Menguji integrasi semua modules bekerja sama dengan baik secara end-to-end.

**Tujuan:** File ini berisi integration tests yang menguji seluruh sistem dari awal hingga akhir. Implementasi `testcontainers` untuk menspin-up test environment (PostgreSQL container, Redis container, Nginx container) secara otomatis. Test database di-populate dengan sample data (100 test domains, 1000 vulnerabilities) untuk memastikan data tersedia. Test `end-to-end scanning`: jalankan full scan pada test website (`httpbin` local atau mock server), verifikasi bahwa semua modules berjalan dengan sukses dan menghasilkan data yang lengkap. Test `scan -> analyze -> report -> export` pipeline: setelah scan selesai, jalankan analysis, generate report dalam semua format (JSON, TXT, DOCS, CSV, HTML, PDF), verifikasi bahwa semua report terbuat dengan benar dan tidak ada error. Test `API integration`: jalankan REST API server, kirim request ke semua endpoints (POST /scan, GET /status, GET /report, POST /analyze, GET /export), verifikasi response yang benar dan status codes yang sesuai. Test `error recovery`: simulasi kegagalan network (gunakan mock yang gagal), verifikasi bahwa sistem melakukan retry dengan exponential backoff dan tidak crash. Test `data persistence`: verifikasi bahwa data tersimpan dengan benar di database dan dapat di-retrieve setelah restart. Verifikasi output dengan **golden files** (files berisi expected output yang telah disetujui) untuk memastikan konsistensi hasil.

---

#### **4. `test_security_modules.rs` (Rust)**

**Tugas:** Menguji secara khusus semua security modules untuk memastikan deteksi kerentanan akurat.

**Tujuan:** File ini berisi tests khusus untuk security modules dengan sample data yang mengandung vulnerabilities dan yang tidak. Test `XSS detection`: buat sample halaman dengan XSS vulnerabilities (reflected, stored, DOM-based), verifikasi bahwa XSS detector menemukan semua vulnerabilities dengan confidence level yang sesuai. Test dengan XSS payload list (100+ payloads) untuk memastikan deteksi komprehensif. Test `SQL injection detection`: buat sample form dengan SQL injection vulnerabilities (boolean-based, time-based, error-based, union-based), verifikasi bahwa SQL injection detector menemukan semua vulnerabilities dengan tipe yang benar. Test `CSRF detection`: buat sample form dengan dan tanpa CSRF token, dengan berbagai konfigurasi SameSite cookie, verifikasi bahwa CSRF analyzer mengidentifikasi form yang vulnerable dengan benar. Test `header analysis`: buat sample responses dengan security headers (HSTS, CSP, X-Frame-Options) yang benar dan yang salah, verifikasi bahwa header analyzer memberi score yang sesuai. Test `cookie scanner`: buat sample cookies dengan berbagai konfigurasi (Secure, HttpOnly, SameSite), verifikasi bahwa cookie scanner mengidentifikasi konfigurasi yang tidak aman.

---

#### **5. `test_agents.rs` (Rust)**

**Tugas:** Menguji semua fungsionalitas agent system untuk memastikan koordinasi dan reliability.

**Tujuan:** File ini berisi tests untuk agent system yang memverifikasi semua aspek agent lifecycle dan komunikasi. Test `agent lifecycle`: buat agent, jalankan `init()` -> `run()` -> `pause()` -> `resume()` -> `shutdown()`, verifikasi bahwa state transition berjalan dengan benar sesuai kontrak. Test `inter-agent communication`: buat multiple agents, kirim message antar agents via message bus, verifikasi bahwa message diterima dengan benar dan payload tidak corrupt. Test `state persistence`: simpan state agent ke storage, restart agent, verifikasi bahwa state di-restore dengan benar. Test `error recovery`: simulasi agent crash (panic), verifikasi bahwa supervisor mendeteksi heartbeat missing dan merestart agent secara otomatis. Test `monitoring agent`: buat monitoring agent dengan simulated changes (new vulnerability, port change), verifikasi bahwa agent mendeteksi perubahan dan generate alert dengan severity yang benar. Test `reconnaissance agent`: jalankan reconnaissance agent pada sample domain, verifikasi bahwa semua stage (WHOIS, DNS, subdomains, technologies) selesai dengan hasil yang valid. Test `reporting agent`: jalankan reporting agent dengan sample data, verifikasi bahwa report dihasilkan dalam semua format yang didukung. Test `concurrent agents`: jalankan 10 agents secara bersamaan, verifikasi bahwa tidak ada race condition dan semua agents selesai dengan sukses.

---

**docs folder explanation:**

---

## 📂 **`docs/` - DOCUMENTATION**

---

#### **1. `architecture.md`**

**Task:** Document the system architecture in detail with diagrams and comprehensive explanations.

**Purpose:** This file is the main architecture documentation explaining the entire IWS system design. Uses C4 model for visualization: **System Context diagram** showing IWS as the main system interacting with users (CLI, API), databases (PostgreSQL, Redis), and external services (Shodan, VirusTotal, etc.). **Container diagram** showing containers within the system: Core Engine (Rust), Orchestrator (Go), AI Models (Python), Database, Cache, API Gateway, and Reverse Proxy. **Component diagram** for each container: Core Engine has Scanner, Analyzer, Extractor, Validator, and Engine components. **Decision records (ADR)** for important design decisions: why Rust for core engine (performance, memory safety), Go for orchestrator (concurrency, simplicity), Python for AI (library ecosystem, flexibility). **Data flow diagrams** showing data flow from user input through scan, analysis, storage, to reporting. **Scalability considerations**: horizontal scaling, load balancing, connection pooling, caching strategy, and database sharding. **Security considerations**: encryption at rest, encryption in transit, authentication, authorization, rate limiting, and secure coding practices.

---

#### **2. `deployment_guide.md`**

**Task:** Provide complete deployment guide for all environments and platforms.

**Purpose:** This file is a step-by-step guide for installing and running IWS on various platforms. **Prerequisites**: list of required software (Python 3.11+, Rust 1.75+, Go 1.21+, PostgreSQL 15+, Redis 7+, Docker, kubectl) with installation commands for each OS. **Installation on Termux**: specific steps for Android using Termux (pkg install, setup storage, install dependencies, run). **Installation on Linux**: for Ubuntu/Debian (apt), CentOS/RHEL (yum), Arch (pacman), with complete commands. **Installation on Windows**: using WSL2 or native with environment setup guide. **Docker deployment**: build image, run container with docker-compose, environment variables configuration. **Kubernetes deployment**: apply manifests, configure secrets, setup ingress with TLS, HPA configuration. **Configuration guide**: explanation of each environment variable in .env, required API keys, and how to obtain them. **Database setup**: initial database initialization guide, migrations, and backup. **Monitoring and logging setup**: how to configure Prometheus metrics, Grafana dashboards, and ELK stack for log aggregation. **Troubleshooting common deployment issues**: port conflicts, database connection failures, permission errors, memory issues.

---

#### **3. `scanning_profiles.md`**

**Task:** Document all available scanning profiles and usage guidelines.

**Purpose:** This file explains in detail each available scanning profile. **Aggressive Profile**: for fast scanning with high resources (Threads=100, Timeout=10s, Delay=0, MaxPages=1000). When to use: for quick testing, internal scanning, when speed is more important than stealth. **Moderate Profile**: for balanced scanning (Threads=50, Timeout=15s, Delay=100ms, MaxPages=500). When to use: default profile, for standard scanning, balance between speed and stealth. **Stealth Profile**: for stealth scanning (Threads=10, Timeout=30s, Delay=1000ms, MaxPages=100). When to use: when not wanting to be detected, for production environment, when target has WAF or rate limiting. **Comprehensive Profile**: for in-depth scanning (Threads=30, Timeout=20s, Delay=200ms, MaxPages=2000). When to use: for compliance audit, thorough security assessment, when coverage is more important than speed. **Custom Profile**: guide for creating custom profiles with parameters: threads (concurrent requests), timeout (per request timeout), delay (between requests), max_pages (maximum pages), follow_redirects (whether to follow redirects), respect_robots (whether to respect robots.txt). Explanation of parameter impact: threads affect speed and resource usage, timeout affects reliability, delay affects stealth, max_pages affects coverage. Best practices for each scenario: testing (aggressive), production (stealth), compliance (comprehensive), standard (moderate).

---

#### **4. `api_documentation.md`**

**Task:** Provide complete API endpoints documentation with OpenAPI/Swagger specification.

**Purpose:** This file is the OpenAPI/Swagger specification documenting all API endpoints. **Authentication**: how to get JWT token, using API key in header `X-API-Key`, role-based access (admin, user, guest). **Endpoints**: `POST /api/v1/scan` - body: `{"url": "example.com", "profile": "moderate"}`, response: `{"scan_id": "uuid", "status": "pending"}`. `GET /api/v1/scan/:id/status` - response: `{"scan_id": "uuid", "state": "scanning", "progress": 75, "current_step": "XSS Detection"}`. `GET /api/v1/scan/:id/report` - response: `{"format": "json", "data": {...}}` (with query param `?format=pdf`). `POST /api/v1/analyze/:id` - triggers analysis on completed scan. `GET /api/v1/export/:id/:format` - download report in specified format (json, txt, docs, csv, html, pdf). `GET /api/v1/history` - with pagination (`?page=1&limit=20&filter=completed`). `POST /api/v1/monitor` - body: `{"url": "example.com", "schedule": "daily"}`. **Error codes**: list of all possible error codes (E1001: Scan not found, E1002: Invalid URL, E1003: Rate limit exceeded, E1004: Authentication failed, E1005: Permission denied, E1006: Invalid format, E1007: Analysis failed). **Rate limiting**: policy per IP (100/minute), per API key tier (admin: 1000/hour, user: 100/hour, guest: 10/hour). **Example requests and responses** for each endpoint with curl commands.

---

#### **5. `troubleshooting.md`**

**Task:** Provide troubleshooting guide for common issues that may be encountered.

**Purpose:** This file is a guide for diagnosing and fixing common issues. **Installation issues**: "Command not found: cargo" -> install Rust with `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`. "Python version too old" -> install Python 3.11+ using `apt install python3.11` or `brew install python@3.11`. **Connection issues**: "Connection refused" -> check firewall (`ufw status`), check service running (`systemctl status iws`), check port (`netstat -tlnp | grep 8080`). "Rate limit exceeded" -> wait or upgrade API key tier, check `Retry-After` header. **Database issues**: "Database connection failed" -> check credentials in `.env`, check PostgreSQL running (`systemctl status postgresql`), check network connectivity. "Migration failed" -> run `python3 scripts/migrate_schema.py --rollback`, check migration logs, verify database integrity. **Scanning issues**: "Scan stuck at 0%" -> check network connectivity, check target URL accessible, check DNS resolution. "No vulnerabilities found" -> check scanning profile (stealth may have limited coverage), check target is actually vulnerable, verify scanner configuration. **Performance issues**: "High memory usage" -> reduce threads in profile, enable memory limits, increase swap space. "Slow scanning" -> increase threads, reduce delay, check network bandwidth. **Diagnostic commands**: list of commands for diagnostic (`iws status --details`, `iws config --list`, `journalctl -u iws -n 100`, `ps aux | grep iws`, `netstat -an | grep 8080`). **Getting help**: how to collect logs, create issue on GitHub, contact support.

---

#### **6. `contributions.md`**

**Task:** Provide contributor guide for developers who want to contribute to the IWS project.

**Purpose:** This file is a guide for developers who want to contribute to the IWS project. **Development setup**: how to set up development environment: clone repo, setup virtual environment, install dependencies, setup pre-commit hooks. **Coding guidelines**: Rust (rustfmt for formatting, clippy for linting, documentation with `///`), Go (gofmt for formatting, golint for linting, documentation with comments), Python (black for formatting, flake8 for linting, isort for import sorting, mypy for type checking). **Testing requirements**: all tests must pass (`cargo test`, `go test`, `pytest`), coverage must be > 80% (`cargo tarpaulin`, `go test -cover`, `pytest --cov`). **PR process**: fork repository, create branch (`feature/feature-name` or `fix/bug-name`), commit with descriptive messages (conventional commits), push and create Pull Request. **PR review process**: minimum 2 reviewers, all comments resolved, CI/CD tests must pass, documentation updated. **Code of Conduct**: standards for contributor interaction (respectful, inclusive, constructive feedback). **Project structure**: brief explanation of project structure and where to add new features. **Release process**: versioning with semantic versioning, changelog maintenance, release notes. **Security reporting**: procedure for reporting security vulnerabilities privately.

---

**indonesian:**

---

## 📂 **`docs/` - DOKUMENTASI**

---

#### **1. `architecture.md`**

**Tugas:** Mendokumentasikan arsitektur sistem secara mendetail dengan diagram dan penjelasan komprehensif.

**Tujuan:** File ini adalah dokumentasi arsitektur utama yang menjelaskan seluruh desain sistem IWS. Menggunakan C4 model untuk visualisasi: **System Context diagram** yang menunjukkan IWS sebagai sistem utama yang berinteraksi dengan pengguna (CLI, API), database (PostgreSQL, Redis), dan layanan eksternal (Shodan, VirusTotal, dll). **Container diagram** yang menunjukkan container-container dalam sistem: Core Engine (Rust), Orchestrator (Go), AI Models (Python), Database, Cache, API Gateway, dan Reverse Proxy. **Component diagram** untuk setiap container: Core Engine memiliki komponen Scanner, Analyzer, Extractor, Validator, dan Engine. **Decision records (ADR)** untuk keputusan desain penting: mengapa Rust untuk core engine (performance, memory safety), Go untuk orchestrator (concurrency, simplicity), Python untuk AI (library ecosystem, flexibility). **Data flow diagrams** yang menunjukkan alur data dari user input melalui scan, analysis, storage, hingga reporting. **Scalability considerations**: horizontal scaling, load balancing, connection pooling, caching strategy, dan database sharding. **Security considerations**: encryption at rest, encryption in transit, authentication, authorization, rate limiting, dan secure coding practices.

---

#### **2. `deployment_guide.md`**

**Tugas:** Menyediakan panduan deployment lengkap untuk semua environment dan platform.

**Tujuan:** File ini adalah panduan langkah-demi-langkah untuk menginstal dan menjalankan IWS di berbagai platform. **Prerequisites**: daftar software yang dibutuhkan (Python 3.11+, Rust 1.75+, Go 1.21+, PostgreSQL 15+, Redis 7+, Docker, kubectl) dengan perintah instalasi untuk setiap OS. **Installation on Termux**: langkah-langkah spesifik untuk Android menggunakan Termux (pkg install, setup storage, install dependencies, run). **Installation on Linux**: untuk Ubuntu/Debian (apt), CentOS/RHEL (yum), Arch (pacman), dengan perintah lengkap. **Installation on Windows**: menggunakan WSL2 atau native dengan panduan setup environment. **Docker deployment**: build image, run container dengan docker-compose, environment variables configuration. **Kubernetes deployment**: apply manifests, configure secrets, setup ingress dengan TLS, HPA configuration. **Configuration guide**: penjelasan setiap environment variable di .env, API keys yang diperlukan, dan cara mendapatkannya. **Database setup**: panduan inisialisasi database pertama kali, migrasi, dan backup. **Monitoring and logging setup**: cara mengkonfigurasi Prometheus metrics, Grafana dashboards, dan ELK stack untuk log aggregation. **Troubleshooting common deployment issues**: port conflicts, database connection failures, permission errors, memory issues.

---

#### **3. `scanning_profiles.md`**

**Tugas:** Mendokumentasikan semua profil scanning yang tersedia dan panduan penggunaannya.

**Tujuan:** File ini menjelaskan secara detail setiap profil scanning yang tersedia. **Aggressive Profile**: untuk scanning cepat dengan resource tinggi (Threads=100, Timeout=10s, Delay=0, MaxPages=1000). Kapan menggunakan: untuk testing cepat, internal scanning, saat kecepatan lebih penting daripada stealth. **Moderate Profile**: untuk scanning balanced (Threads=50, Timeout=15s, Delay=100ms, MaxPages=500). Kapan menggunakan: default profile, untuk scanning standar, keseimbangan antara speed dan stealth. **Stealth Profile**: untuk scanning stealth (Threads=10, Timeout=30s, Delay=1000ms, MaxPages=100). Kapan menggunakan: saat tidak ingin terdeteksi, untuk production environment, saat target memiliki WAF atau rate limiting. **Comprehensive Profile**: untuk scanning mendalam (Threads=30, Timeout=20s, Delay=200ms, MaxPages=2000). Kapan menggunakan: untuk compliance audit, security assessment menyeluruh, saat coverage lebih penting daripada speed. **Custom Profile**: panduan membuat profile kustom dengan parameter: threads (jumlah concurrent requests), timeout (timeout per request), delay (delay antar request), max_pages (maksimum halaman), follow_redirects (apakah mengikuti redirect), respect_robots (apakah menghormati robots.txt). Penjelasan impact setiap parameter: threads mempengaruhi speed dan resource usage, timeout mempengaruhi reliability, delay mempengaruhi stealth, max_pages mempengaruhi coverage. Best practices untuk setiap skenario: testing (aggressive), production (stealth), compliance (comprehensive), standard (moderate).

---

#### **4. `api_documentation.md`**

**Tugas:** Menyediakan dokumentasi lengkap API endpoints dengan OpenAPI/Swagger specification.

**Tujuan:** File ini adalah OpenAPI/Swagger specification yang mendokumentasikan semua API endpoints. **Authentication**: cara mendapatkan JWT token, menggunakan API key di header `X-API-Key`, role-based access (admin, user, guest). **Endpoints**: `POST /api/v1/scan` - body: `{"url": "example.com", "profile": "moderate"}`, response: `{"scan_id": "uuid", "status": "pending"}`. `GET /api/v1/scan/:id/status` - response: `{"scan_id": "uuid", "state": "scanning", "progress": 75, "current_step": "XSS Detection"}`. `GET /api/v1/scan/:id/report` - response: `{"format": "json", "data": {...}}` (bisa dengan query param `?format=pdf`). `POST /api/v1/analyze/:id` - triggers analysis on completed scan. `GET /api/v1/export/:id/:format` - download report in specified format (json, txt, docs, csv, html, pdf). `GET /api/v1/history` - dengan pagination (`?page=1&limit=20&filter=completed`). `POST /api/v1/monitor` - body: `{"url": "example.com", "schedule": "daily"}`. **Error codes**: daftar semua error codes yang mungkin (E1001: Scan not found, E1002: Invalid URL, E1003: Rate limit exceeded, E1004: Authentication failed, E1005: Permission denied, E1006: Invalid format, E1007: Analysis failed). **Rate limiting**: policy per IP (100/minute), per API key tier (admin: 1000/hour, user: 100/hour, guest: 10/hour). **Example requests and responses** untuk setiap endpoint dengan curl commands.

---

#### **5. `troubleshooting.md`**

**Tugas:** Menyediakan panduan troubleshooting untuk masalah umum yang mungkin dihadapi.

**Tujuan:** File ini adalah panduan untuk mendiagnosis dan memperbaiki masalah umum. **Installation issues**: "Command not found: cargo" -> install Rust with `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`. "Python version too old" -> install Python 3.11+ menggunakan `apt install python3.11` atau `brew install python@3.11`. **Connection issues**: "Connection refused" -> check firewall (`ufw status`), check service running (`systemctl status iws`), check port (`netstat -tlnp | grep 8080`). "Rate limit exceeded" -> wait or upgrade API key tier, check `Retry-After` header. **Database issues**: "Database connection failed" -> check credentials in `.env`, check PostgreSQL running (`systemctl status postgresql`), check network connectivity. "Migration failed" -> run `python3 scripts/migrate_schema.py --rollback`, check migration logs, verify database integrity. **Scanning issues**: "Scan stuck at 0%" -> check network connectivity, check target URL accessible, check DNS resolution. "No vulnerabilities found" -> check scanning profile (stealth may have limited coverage), check target is actually vulnerable, verify scanner configuration. **Performance issues**: "High memory usage" -> reduce threads in profile, enable memory limits, increase swap space. "Slow scanning" -> increase threads, reduce delay, check network bandwidth. **Diagnostic commands**: list of commands untuk diagnostic (`iws status --details`, `iws config --list`, `journalctl -u iws -n 100`, `ps aux | grep iws`, `netstat -an | grep 8080`). **Getting help**: how to collect logs, create issue on GitHub, contact support.

---

#### **6. `contributions.md`**

**Tugas:** Menyediakan panduan untuk kontributor yang ingin mengembangkan proyek IWS.

**Tujuan:** File ini adalah panduan bagi developer yang ingin berkontribusi ke proyek IWS. **Development setup**: cara setup environment development: clone repo, setup virtual environment, install dependencies, setup pre-commit hooks. **Coding guidelines**: Rust (rustfmt untuk formatting, clippy untuk linting, dokumentasi dengan `///`), Go (gofmt untuk formatting, golint untuk linting, dokumentasi dengan comments), Python (black untuk formatting, flake8 untuk linting, isort untuk import sorting, mypy untuk type checking). **Testing requirements**: semua tests harus pass (`cargo test`, `go test`, `pytest`), coverage harus > 80% (`cargo tarpaulin`, `go test -cover`, `pytest --cov`). **PR process**: fork repository, create branch (`feature/feature-name` atau `fix/bug-name`), commit dengan descriptive messages (conventional commits), push dan create Pull Request. **PR review process**: minimal 2 reviewers, semua comments resolved, CI/CD tests must pass, documentation updated. **Code of Conduct**: standar untuk interaksi antar kontributor (respectful, inclusive, constructive feedback). **Project structure**: penjelasan singkat tentang struktur proyek dan di mana menambahkan fitur baru. **Release process**: versioning dengan semantic versioning, changelog maintenance, release notes. **Security reporting**: prosedur untuk melaporkan security vulnerabilities secara private.

---

**scripts folder explanation:**

---

## 📂 **`scripts/` - UTILITY SCRIPTS**

---

#### **1. `init_database.py` (Python)**

**Task:** Initialize the database for the first time with all tables, indexes, and default data.

**Purpose:** This file is the first setup script that must be run after installation to prepare the database. Implements `init_database()` which: connects to the database using `DATABASE_URL` from environment variables (`os.environ.get('DATABASE_URL')`). Reads `schema.sql` file from `database/` directory and executes all SQL statements sequentially. Creates all tables defined in the schema (users, scan_results, vulnerabilities, agent_states, configuration). Creates indexes for query optimization (scan_results(user_id, started_at), vulnerabilities(scan_id, severity), agent_states(agent_name)). Creates foreign key constraints with ON DELETE CASCADE to maintain referential integrity. Inserts default data: admin user (`username='admin', password_hash=bcrypt.hash('admin123'), role='admin'`) for first access. Default scanning profiles (aggressive, moderate, stealth, comprehensive) into `configuration` table with key 'scanning_profiles'. Default configuration (max_threads=50, default_timeout=30, default_profile='moderate') into `configuration` table. Checks if database already exists (check table exists), if so skip initialization to prevent data loss. Implements `--force` flag to force re-initialization (drop all tables and recreate). Implements `--sample-data` flag to insert sample data for testing (100 domains, 1000 vulnerabilities). Logs each step with timestamp for process tracking.

---

#### **2. `migrate_schema.py` (Python)**

**Task:** Manage database schema migrations to update structure without losing data.

**Purpose:** This file manages schema changes over time. Implements using `alembic` for migration management. `alembic init` for initial setup (creates `migrations/` directory and `alembic.ini`). `alembic revision --autogenerate -m "description"` to automatically generate migration script based on SQLAlchemy model changes. `alembic upgrade head` to apply all pending migrations to the database. `alembic downgrade -1` to rollback one migration if issues occur. Implements `create_migration(name: str)` to create new migration file with timestamp and template (upgrade and downgrade sections). Automatic database backup before running migrations using `pg_dump` to prevent data loss if migration fails. Implements `validate_schema()` to verify schema integrity after migration (check foreign keys, indexes, constraints). Implements `--dry-run` flag to preview migration without executing. Implements `--sql` flag to generate SQL statements without executing. Implements `--version` flag to display current schema version. Implements `--history` flag to display migration history already executed.

---

#### **3. `backup_storage.rs` (Rust)**

**Task:** Perform automatic and encrypted backups of all storage data.

**Purpose:** This file manages system data backup for disaster recovery. Implements `backup_storage()` which: dumps database using `pg_dump` with custom format (`pg_dump -Fc -f backup.dump`) for compression and efficiency. Backs up file data (`data/scans/`, `data/reports/`, `data/exports/`) using `tar` with compression (`tar -czf data_backup.tar.gz data/`). Compresses all backups with `gzip` to save space. Encrypts all backups with AES-256 using `encryption.rs` utility. Uploads to cloud storage (AWS S3 using `aws-sdk-s3`, Google Cloud Storage, or Azure Blob) for offsite backup. Retention policy: keep last 7 daily backups, 4 weekly backups, 12 monthly backups (total 23 backup sets). Implements `restore_backup(backup_id: &str)` to restore from backup: download from cloud, decrypt, decompress, restore database using `pg_restore`, restore file data. Implements `list_backups()` to display all available backups with timestamp and size. Implements `verify_backup()` to verify backup integrity (checksum verification). Implements `--schedule` flag to run backups automatically with cron expression (e.g., daily at 2 AM). Implements `--cleanup` flag to delete old backups based on retention policy. Logs all backup activity to `logs/backup.log` for audit trail.

---

#### **4. `cleanup_cache.rs` (Rust)**

**Task:** Clean up cache and temporary files to save disk space and maintain performance.

**Purpose:** This file manages cleanup of temporary data and cache. Implements `cleanup_cache()` which: iterates `data/cache/` directory and deletes files older than 7 days (using `std::fs::metadata()` and `SystemTime::elapsed()`). Cleans `data/temp/` directory by deleting files older than 24 hours. Rotates log files: compresses log files if size > 100MB (`access.log`, `error.log`, `scanner_activity.log`, `agent_trace.log`), keeps 5 archives, compresses with gzip. Cache invalidation: removes entries from `dns_cache.db` that have expired (TTL exceeded), removes entries from `html_cache.db` that have expired, removes entries from `entity_cache.db` that have expired. Implements `--dry-run` flag to preview files to be deleted without actual deletion. Implements `--verbose` flag to display details of each file deleted. Implements `--age` flag to specify maximum file age (default 7 days). Implements `--size` flag to specify size threshold for log rotation (default 100MB). Implements `--include` and `--exclude` patterns for file selection. Logs all cleanup activity to `logs/cleanup.log` for audit trail. Implements `schedule_cleanup()` to run cleanup automatically (daily at 3 AM) using cron.

---

#### **5. `generate_analysis_report.rs` (Rust)**

**Task:** Automatically generate analysis reports based on schedule or trigger.

**Purpose:** This file automates analysis report generation. Implements `generate_report()` which: queries database for scans that are completed (`SELECT * FROM scan_results WHERE status = 'completed'`). For each scan that has not been analyzed or report not generated, runs analysis using `analysis_agent`. Generates reports in all formats (JSON, TXT, DOCS, CSV, HTML, PDF) and saves to `data/reports/{format}/`. Saves report metadata to database with timestamp for tracking. Implements `--scan-id` flag to generate report for specific scan ID. Implements `--format` flag to generate only specific format (default: all). Implements `--email` flag to send report via email to configured recipients (from `settings.rs`). Implements `--schedule` flag to run automatically with cron expression (e.g., daily at 9 AM for executive summary, weekly at 5 PM for technical report). Implements `--period` flag to generate report for specific period (`daily`, `weekly`, `monthly`) - queries scans within that period and aggregates results. Implements `--output` flag to specify output directory (default: `data/reports/`). Implements notification after report completion: sends notification to configured webhooks (Slack, Discord, Telegram). Implements `archive_old_reports()` to archive reports older than 90 days to cold storage. Logs all report generation activity to `logs/report.log` for audit trail.

---

**indonesian:**

---

## 📂 **`scripts/` - SCRIPT UTILITY**

---

#### **1. `init_database.py` (Python)**

**Tugas:** Menginisialisasi database untuk pertama kali dengan semua tabel, indeks, dan data default.

**Tujuan:** File ini adalah script setup pertama yang harus dijalankan setelah instalasi untuk menyiapkan database. Implementasi `init_database()` yang melakukan: koneksi ke database menggunakan `DATABASE_URL` dari environment variables (`os.environ.get('DATABASE_URL')`). Membaca file `schema.sql` dari direktori `database/` dan mengeksekusi semua SQL statements secara sequential. Membuat semua tabel yang didefinisikan di schema (users, scan_results, vulnerabilities, agent_states, configuration). Membuat indeks untuk optimasi query (scan_results(user_id, started_at), vulnerabilities(scan_id, severity), agent_states(agent_name)). Membuat foreign key constraints dengan ON DELETE CASCADE untuk maintain referential integrity. Menginsert data default: admin user (`username='admin', password_hash=bcrypt.hash('admin123'), role='admin'`) untuk akses pertama. Profil scanning default (aggressive, moderate, stealth, comprehensive) ke dalam table `configuration` dengan key 'scanning_profiles'. Configuration default (max_threads=50, default_timeout=30, default_profile='moderate') ke dalam table `configuration`. Memeriksa apakah database sudah ada sebelumnya (check table exists), jika sudah ada maka skip inisialisasi untuk mencegah data loss. Implementasi `--force` flag untuk force re-initialization (drop semua tabel dan recreate). Implementasi `--sample-data` flag untuk menginsert sample data untuk testing (100 domain, 1000 vulnerabilities). Logging setiap langkah dengan timestamp untuk tracking proses.

---

#### **2. `migrate_schema.py` (Python)**

**Tugas:** Mengelola migrasi schema database untuk update struktur tanpa kehilangan data.

**Tujuan:** File ini mengelola perubahan schema database seiring waktu. Implementasi menggunakan `alembic` untuk migration management. `alembic init` untuk inisialisasi awal (membuat direktori `migrations/` dan `alembic.ini`). `alembic revision --autogenerate -m "description"` untuk generate migration script secara otomatis berdasarkan perubahan model SQLAlchemy. `alembic upgrade head` untuk mengaplikasikan semua migration yang pending ke database. `alembic downgrade -1` untuk rollback satu migration terakhir jika terjadi masalah. Implementasi `create_migration(name: str)` untuk membuat migration file baru dengan timestamp dan template (upgrade dan downgrade sections). Backup database otomatis sebelum menjalankan migrasi menggunakan `pg_dump` untuk mencegah data loss jika migrasi gagal. Implementasi `validate_schema()` untuk memverifikasi integritas schema setelah migrasi (check foreign keys, indexes, constraints). Implementasi `--dry-run` flag untuk preview migration tanpa mengeksekusi. Implementasi `--sql` flag untuk generate SQL statements tanpa execute. Implementasi `--version` flag untuk menampilkan versi schema saat ini. Implementasi `--history` flag untuk menampilkan riwayat migrasi yang sudah dijalankan.

---

#### **3. `backup_storage.rs` (Rust)**

**Tugas:** Melakukan backup semua data storage secara otomatis dan terenkripsi.

**Tujuan:** File ini mengelola backup data sistem untuk disaster recovery. Implementasi `backup_storage()` yang melakukan: dump database menggunakan `pg_dump` dengan format custom (`pg_dump -Fc -f backup.dump`) untuk kompresi dan efisiensi. Backup file data (`data/scans/`, `data/reports/`, `data/exports/`) menggunakan `tar` dengan compression (`tar -czf data_backup.tar.gz data/`). Compress semua backup dengan `gzip` untuk menghemat ruang. Encrypt semua backup dengan AES-256 menggunakan `encryption.rs` utility. Upload ke cloud storage (AWS S3 menggunakan `aws-sdk-s3`, Google Cloud Storage, atau Azure Blob) untuk offsite backup. Retensi policy: keep last 7 daily backups, 4 weekly backups, 12 monthly backups (total 23 backup sets). Implementasi `restore_backup(backup_id: &str)` untuk restore dari backup: download dari cloud, decrypt, decompress, restore database menggunakan `pg_restore`, restore file data. Implementasi `list_backups()` untuk menampilkan semua backup yang tersedia dengan timestamp dan size. Implementasi `verify_backup()` untuk memverifikasi integritas backup (checksum verification). Implementasi `--schedule` flag untuk menjalankan backup secara otomatis dengan cron expression (contoh: daily at 2 AM). Implementasi `--cleanup` flag untuk menghapus backup lama berdasarkan retensi policy. Logging semua aktivitas backup ke `logs/backup.log` untuk audit trail.

---

#### **4. `cleanup_cache.rs` (Rust)**

**Tugas:** Membersihkan cache dan temporary files untuk menghemat ruang disk dan menjaga performa.

**Tujuan:** File ini mengelola pembersihan data sementara dan cache. Implementasi `cleanup_cache()` yang melakukan: iterate direktori `data/cache/` dan menghapus file yang lebih tua dari 7 hari (menggunakan `std::fs::metadata()` dan `SystemTime::elapsed()`). Membersihkan direktori `data/temp/` dengan menghapus file yang lebih tua dari 24 jam. Rotasi log files: kompres log file jika size > 100MB (`access.log`, `error.log`, `scanner_activity.log`, `agent_trace.log`), keep 5 archives, compress dengan gzip. Cache invalidation: menghapus entry dari `dns_cache.db` yang sudah expired (TTL exceeded), menghapus entry dari `html_cache.db` yang sudah expired, menghapus entry dari `entity_cache.db` yang sudah kadaluarsa. Implementasi `--dry-run` flag untuk preview files yang akan dihapus tanpa melakukan penghapusan. Implementasi `--verbose` flag untuk menampilkan detail setiap file yang dihapus. Implementasi `--age` flag untuk menentukan umur maksimum file (default 7 hari). Implementasi `--size` flag untuk menentukan size threshold untuk log rotation (default 100MB). Implementasi `--include` dan `--exclude` pattern untuk seleksi file. Logging semua aktivitas cleanup ke `logs/cleanup.log` untuk audit trail. Implementasi `schedule_cleanup()` untuk menjalankan cleanup secara otomatis (daily at 3 AM) menggunakan cron.

---

#### **5. `generate_analysis_report.rs` (Rust)**

**Tugas:** Generate analysis report secara otomatis berdasarkan schedule atau trigger.

**Tujuan:** File ini mengotomatiskan pembuatan laporan analysis. Implementasi `generate_report()` yang melakukan: query database untuk mencari scan yang sudah completed (`SELECT * FROM scan_results WHERE status = 'completed'`). Untuk setiap scan yang belum dianalisis atau report belum digenerate, jalankan analysis menggunakan `analysis_agent`. Generate reports dalam semua format (JSON, TXT, DOCS, CSV, HTML, PDF) dan simpan ke `data/reports/{format}/`. Save report metadata ke database dengan timestamp untuk tracking. Implementasi `--scan-id` flag untuk generate report untuk scan ID tertentu. Implementasi `--format` flag untuk generate hanya format tertentu (default: all). Implementasi `--email` flag untuk mengirim report via email ke configured recipients (dari `settings.rs`). Implementasi `--schedule` flag untuk menjalankan secara otomatis dengan cron expression (contoh: daily at 9 AM untuk executive summary, weekly at 5 PM untuk technical report). Implementasi `--period` flag untuk generate report untuk period tertentu (`daily`, `weekly`, `monthly`) - query scan dalam periode tersebut dan aggregate hasilnya. Implementasi `--output` flag untuk menentukan direktori output (default: `data/reports/`). Implementasi `notification` setelah report selesai: kirim notifikasi ke webhook yang dikonfigurasi (Slack, Discord, Telegram). Implementasi `archive_old_reports()` untuk mengarsipkan report yang lebih tua dari 90 hari ke cold storage. Logging semua aktivitas report generation ke `logs/report.log` untuk audit trail.

---

**data folder explanation:**

---

## 📂 **`data/` - DATA STORAGE**

---

#### **1. `scans/` - Scan Results**

**Task:** Store all scan result data across various stages.

**Purpose:** This folder is the main storage location for all scanning data generated by the system. Divided into three subfolders with different purposes:

- **`active/`**: Stores currently running scan data. File naming: `{scan_id}.json` in JSON format containing current scan state. Data structure: `ScanData` with fields: `id` (scan UUID), `url` (target URL), `profile` (scanning profile used), `status` (active, paused, error), `progress` (percentage 0-100), `start_time` (start timestamp), `modules_completed` (list of completed modules), `modules_pending` (list of pending modules), `results` (partial results from completed modules), `errors` (errors that occurred). Lock file: `{scan_id}.lock` to prevent concurrent writes from multiple processes, using `flock` system call. Implements auto-save every 30 seconds to prevent data loss in case of crash.

- **`completed/`**: Stores completed scan data (status completed or failed). JSON format with complete structure: all module results, timing information, errors, summary statistics. File naming: `{scan_id}_{timestamp}.json` with completion timestamp. Data is stored permanently and used for analysis, reporting, and historical reference.

- **`archived/`**: Stores old scan data that has been archived to save space. Files are compressed with gzip (`.json.gz`) and automatically archived by `scripts/cleanup_cache.rs` for data older than 90 days. Metadata is stored in the database for reference (scan_id, url, timestamp, archive_location) to remain queryable even after data is archived.

---

#### **2. `reports/` - Reports**

**Task:** Store all reports generated in various formats.

**Purpose:** This folder stores reports generated by the reporting agent in 6 different formats, each with its own subfolder:

- **`json/`**: Reports in structured JSON format. File naming: `{scan_id}_{timestamp}.json`. Contains complete findings data, vulnerabilities, risk scores, recommendations in JSON format that can be processed by other systems. Suitable for API integration and data interchange.

- **`txt/`**: Reports in plain text format. File naming: `{scan_id}_{timestamp}.txt`. Contains executive summary, vulnerability list with severity, and recommendations in easy-to-read text format. Suitable for quick terminal review.

- **`docs/`**: Reports in DOCX format (Microsoft Word). File naming: `{scan_id}_{timestamp}.docx`. Contains complete documentation with natural language narrative, tables, and professional formatting. Created using `docs_builder.rs` with Jinja2 templates. Suitable for distribution to non-technical stakeholders.

- **`csv/`**: Reports in CSV format (Comma-Separated Values). File naming: `{scan_id}_{timestamp}.csv`. Contains tabular data of vulnerabilities and findings that can be opened in spreadsheets (Excel, Google Sheets). Suitable for statistical analysis and data processing.

- **`html/`**: Reports in interactive HTML format. File naming: `{scan_id}_{timestamp}.html`. Contains complete report with interactive JavaScript: sorting, filtering, charts, and visualizations. Created using `html_reporter.rs` with Chart.js and DataTable. Suitable for browser-based interactive review.

- **`pdf/`**: Reports in professional PDF format. File naming: `{scan_id}_{timestamp}.pdf`. Contains complete report with professional formatting, watermark, digital signature, and page numbering. Created using `pdf_generator.py` with WeasyPrint. Suitable for formal distribution and archiving.

Metadata file `report_metadata.json` tracks all generated reports with fields: report_id, scan_id, format, timestamp, size, status (generated/failed), and link to file.

---

#### **3. `cache/` - Cache**

**Task:** Store cached data to improve system performance.

**Purpose:** This folder stores various caches to avoid expensive repeated operations. Consists of three SQLite database files:

- **`dns_cache.db`**: SQLite database with `dns_cache` table storing DNS lookup results. Schema: `domain` (TEXT, PRIMARY KEY), `record_type` (TEXT), `records` (TEXT, JSON array of records), `ttl` (INTEGER), `expires_at` (INTEGER, timestamp). Implements TTL-based invalidation: records are valid if `expires_at > current_time`. Cache is populated on each DNS query and used to avoid repeated DNS queries for the same domain in a short period. Cache size: maximum 10,000 entries with LRU eviction.

- **`html_cache.db`**: SQLite database with `html_cache` table storing fetched HTML pages. Schema: `url` (TEXT, PRIMARY KEY), `content_hash` (TEXT), `content` (TEXT, HTML content), `fetch_time` (INTEGER, timestamp). Implements content-based deduplication: before fetch, checks if URL is in cache and content_hash matches, uses cache. Cache is invalidated after 24 hours or if content_hash changes. Cache size: maximum 5,000 entries with LRU eviction.

- **`entity_cache.db`**: SQLite database with `entity_cache` table storing entity recognition results. Schema: `entity_type` (TEXT), `entity_value` (TEXT, PRIMARY KEY), `context` (TEXT, JSON), `discovered_at` (INTEGER, timestamp). Stores already extracted entities (emails, phones, technologies, API keys) to avoid re-processing. Cache is invalidated after 7 days. Cache size: maximum 20,000 entries with LRU eviction.

Implements cache invalidation: TTL-based (expires_at) and LRU eviction (max entries per table) to prevent uncontrolled cache growth.

---

#### **4. `logs/` - Logs**

**Task:** Store all system logs for debugging, monitoring, and auditing.

**Purpose:** This folder stores logs from various system components in JSONL format (JSON Lines) for easy parsing and analysis. Consists of four log files:

- **`access.log`**: JSONL format with fields: `timestamp` (ISO 8601 datetime), `client_ip` (IP address), `method` (HTTP method), `path` (request path), `status` (HTTP status code), `response_time` (duration in ms), `user_agent` (User-Agent string), `request_id` (UUID for tracing). Records all requests to the API server. Rotation: 10MB per file, keep 5 archives.

- **`error.log`**: JSONL format with fields: `timestamp`, `level` (ERROR/FATAL), `component` (component with error), `message` (error message), `stack_trace` (stack trace if available), `context` (request_id, user_id, scan_id). Records all errors occurring in the system. Rotation: 10MB per file, keep 5 archives.

- **`scanner_activity.log`**: JSONL format with fields: `timestamp`, `scan_id`, `module` (module name), `event` (started, progress, completed, failed), `details` (JSON with event details), `duration` (duration in ms). Records all scanning activity: module start, progress updates, completion, errors. Used for debugging and performance analysis. Rotation: 10MB per file, keep 5 archives.

- **`agent_trace.log`**: JSONL format with fields: `timestamp`, `agent_name`, `agent_state` (state transition), `message` (agent activity), `details` (JSON with details), `duration` (duration in ms). Records all agent activity: state transitions, messages sent/received, task execution, errors. Used for debugging agent system and inter-agent communication. Rotation: 10MB per file, keep 5 archives.

Implements log rotation: each file has size threshold 10MB, when threshold is reached, file is rotated and compressed with gzip, keep maximum 5 archives (total 50MB per log type). Logs can be forwarded to remote server (ELK stack, Graylog) via `utils/logging_system.rs`.

---

#### **5. `exports/` - Data Exports**

**Task:** Store data exported by users in various formats.

**Purpose:** This folder stores export results requested by users via API or CLI. Each format has its own subfolder:

- **`json_exports/`**: Exports in JSON format. File naming: `{scan_id}_export_{timestamp}.json`. Used when users request raw data export in JSON format.

- **`txt_exports/`**: Exports in TXT format. File naming: `{scan_id}_export_{timestamp}.txt`. Used when users request summary export in plain text.

- **`docs_exports/`**: Exports in DOCX format. File naming: `{scan_id}_export_{timestamp}.docx`. Used when users request complete documentation export.

- **`csv_exports/`**: Exports in CSV format. File naming: `{scan_id}_export_{timestamp}.csv`. Used when users request tabular data export.

- **`html_exports/`**: Exports in HTML format. File naming: `{scan_id}_export_{timestamp}.html`. Used when users request interactive report export.

- **`pdf_exports/`**: Exports in PDF format. File naming: `{scan_id}_export_{timestamp}.pdf`. Used when users request professional report export.

Exports are performed asynchronously via background tasks, progress is displayed in UI (CLI or WebSocket). Users can download export files via API endpoint `GET /api/v1/export/{scan_id}/{format}`. Export files have 7-day TTL, after which they are deleted by `cleanup_cache.rs` to save space.

---

#### **6. `temp/` - Temporary Files**

**Task:** Store temporary files used during processing.

**Purpose:** This folder stores temporary files used for various processing operations. Use cases include: intermediate files during large data processing (CSV files being processed), temporary downloads (files downloaded from the internet for analysis), unpacked archives (unzipped files for content extraction), image processing (screenshots generated for reports, then embedded into PDF/HTML). Files here are temporary with short lifecycle (typically < 24 hours). Periodically cleaned by `scripts/cleanup_cache.rs` which deletes files older than 24 hours. This folder is not version controlled and is ignored by `.gitignore`. Lock files are used to prevent conflicts if multiple processes try to use the same file.

---

**indonesian:**

---

## 📂 **`data/` - DATA STORAGE**

---

#### **1. `scans/` - Hasil Scanning**

**Tugas:** Menyimpan semua data hasil scanning dalam berbagai tahap.

**Tujuan:** Folder ini adalah tempat penyimpanan utama untuk semua data scanning yang dihasilkan sistem. Terbagi menjadi tiga subfolder dengan tujuan berbeda:

- **`active/`**: Menyimpan data scanning yang sedang berjalan. File naming: `{scan_id}.json` dengan format JSON yang berisi state scanning saat ini. Struktur data: `ScanData` dengan fields: `id` (UUID scan), `url` (target URL), `profile` (scanning profile yang digunakan), `status` (active, paused, error), `progress` (persentase 0-100), `start_time` (timestamp mulai), `modules_completed` (daftar module yang sudah selesai), `modules_pending` (daftar module yang belum dijalankan), `results` (partial results dari module yang sudah selesai), `errors` (error yang terjadi). Lock file: `{scan_id}.lock` untuk mencegah concurrent writes dari multiple processes, menggunakan `flock` system call. Implementasi auto-save setiap 30 detik untuk mencegah data loss jika terjadi crash.

- **`completed/`**: Menyimpan data scanning yang sudah selesai (status completed atau failed). Format JSON dengan struktur lengkap: semua module results, timing information, errors, summary statistics. File naming: `{scan_id}_{timestamp}.json` dengan timestamp selesai. Data disimpan secara permanen dan digunakan untuk analysis, reporting, dan historical reference.

- **`archived/`**: Menyimpan data scanning lama yang sudah diarsipkan untuk menghemat ruang. File di-compress dengan gzip (`.json.gz`) dan diarsipkan secara otomatis oleh script `scripts/cleanup_cache.rs` untuk data yang lebih tua dari 90 hari. Metadata disimpan di database untuk referensi (scan_id, url, timestamp, archive_location) agar tetap dapat di-query meskipun data sudah di-archive.

---

#### **2. `reports/` - Laporan**

**Tugas:** Menyimpan semua laporan yang dihasilkan dalam berbagai format.

**Tujuan:** Folder ini menyimpan laporan yang dihasilkan oleh reporting agent dalam 6 format berbeda, masing-masing memiliki subfolder tersendiri:

- **`json/`**: Laporan dalam format JSON terstruktur. File naming: `{scan_id}_{timestamp}.json`. Berisi data lengkap findings, vulnerabilities, risk scores, recommendations dalam format JSON yang dapat diproses oleh sistem lain. Cocok untuk integrasi API dan data interchange.

- **`txt/`**: Laporan dalam plain text format. File naming: `{scan_id}_{timestamp}.txt`. Berisi ringkasan eksekutif, daftar vulnerabilities dengan severity, dan rekomendasi dalam format teks yang mudah dibaca. Cocok untuk quick review di terminal.

- **`docs/`**: Laporan dalam format DOCX (Microsoft Word). File naming: `{scan_id}_{timestamp}.docx`. Berisi dokumentasi lengkap dengan narasi natural language, tabel, dan formatting profesional. Dibuat menggunakan `docs_builder.rs` dengan template Jinja2. Cocok untuk distribusi ke stakeholder non-teknis.

- **`csv/`**: Laporan dalam format CSV (Comma-Separated Values). File naming: `{scan_id}_{timestamp}.csv`. Berisi data tabular dari vulnerabilities dan findings yang dapat dibuka di spreadsheet (Excel, Google Sheets). Cocok untuk analisis statistik dan data processing.

- **`html/`**: Laporan dalam format HTML interaktif. File naming: `{scan_id}_{timestamp}.html`. Berisi laporan lengkap dengan JavaScript interaktif: sorting, filtering, charts, dan visualisasi. Dibuat menggunakan `html_reporter.rs` dengan Chart.js dan DataTable. Cocok untuk dibuka di browser untuk review interaktif.

- **`pdf/`**: Laporan dalam format PDF profesional. File naming: `{scan_id}_{timestamp}.pdf`. Berisi laporan lengkap dengan formatting profesional, watermark, digital signature, dan page numbering. Dibuat menggunakan `pdf_generator.py` dengan WeasyPrint. Cocok untuk distribusi formal dan archiving.

Metadata file `report_metadata.json` yang melacak semua reports yang di-generate dengan fields: report_id, scan_id, format, timestamp, size, status (generated/failed), dan link ke file.

---

#### **3. `cache/` - Cache**

**Tugas:** Menyimpan data cache untuk meningkatkan performa sistem.

**Tujuan:** Folder ini menyimpan berbagai cache untuk menghindari operasi berulang yang mahal. Terdiri dari tiga file SQLite database:

- **`dns_cache.db`**: SQLite database dengan table `dns_cache` yang menyimpan hasil DNS lookup. Schema: `domain` (TEXT, PRIMARY KEY), `record_type` (TEXT), `records` (TEXT, JSON array of records), `ttl` (INTEGER), `expires_at` (INTEGER, timestamp). Implementasi TTL-based invalidation: records dianggap valid jika `expires_at > current_time`. Cache diisi setiap kali DNS query dilakukan, dan digunakan untuk menghindari query DNS berulang untuk domain yang sama dalam waktu singkat. Ukuran cache: maksimum 10,000 entries dengan LRU eviction.

- **`html_cache.db`**: SQLite database dengan table `html_cache` yang menyimpan halaman HTML yang sudah di-fetch. Schema: `url` (TEXT, PRIMARY KEY), `content_hash` (TEXT), `content` (TEXT, HTML content), `fetch_time` (INTEGER, timestamp). Implementasi content-based deduplication: sebelum fetch, check jika URL sudah ada di cache dan content_hash sama, gunakan cache. Cache di-invalidate setelah 24 jam atau jika content_hash berubah. Ukuran cache: maksimum 5,000 entries dengan LRU eviction.

- **`entity_cache.db`**: SQLite database dengan table `entity_cache` yang menyimpan hasil entity recognition. Schema: `entity_type` (TEXT), `entity_value` (TEXT, PRIMARY KEY), `context` (TEXT, JSON), `discovered_at` (INTEGER, timestamp). Menyimpan entities yang sudah diekstrak (emails, phones, technologies, API keys) untuk menghindari re-processing. Cache di-invalidate setelah 7 hari. Ukuran cache: maksimum 20,000 entries dengan LRU eviction.

Implementasi cache invalidation: TTL-based (expires_at) dan LRU eviction (max entries per table) untuk mencegah cache growth tidak terkendali.

---

#### **4. `logs/` - Logs**

**Tugas:** Menyimpan semua log sistem untuk debugging, monitoring, dan audit.

**Tujuan:** Folder ini menyimpan log dari berbagai komponen sistem dalam format JSONL (JSON Lines) untuk memudahkan parsing dan analysis. Terdiri dari empat file log:

- **`access.log`**: JSONL format dengan fields: `timestamp` (ISO 8601 datetime), `client_ip` (IP address), `method` (HTTP method), `path` (request path), `status` (HTTP status code), `response_time` (duration in ms), `user_agent` (User-Agent string), `request_id` (UUID untuk tracing). Mencatat semua request yang masuk ke API server. Rotasi: 10MB per file, keep 5 archives.

- **`error.log`**: JSONL format dengan fields: `timestamp`, `level` (ERROR/FATAL), `component` (komponen yang error), `message` (error message), `stack_trace` (stack trace jika ada), `context` (request_id, user_id, scan_id). Mencatat semua error yang terjadi di sistem. Rotasi: 10MB per file, keep 5 archives.

- **`scanner_activity.log`**: JSONL format dengan fields: `timestamp`, `scan_id`, `module` (nama module), `event` (started, progress, completed, failed), `details` (JSON dengan detail event), `duration` (duration in ms). Mencatat semua aktivitas scanning: module start, progress updates, completion, errors. Digunakan untuk debugging dan performance analysis. Rotasi: 10MB per file, keep 5 archives.

- **`agent_trace.log`**: JSONL format dengan fields: `timestamp`, `agent_name`, `agent_state` (state transition), `message` (agent activity), `details` (JSON dengan detail), `duration` (duration in ms). Mencatat semua aktivitas agents: state transitions, messages sent/received, task execution, errors. Digunakan untuk debugging agent system dan inter-agent communication. Rotasi: 10MB per file, keep 5 archives.

Implementasi log rotation: setiap file memiliki size threshold 10MB, ketika mencapai threshold, file di-rotate dan di-compress dengan gzip, keep maksimum 5 archives (total 50MB per log type). Logs dapat di-forward ke remote server (ELK stack, Graylog) melalui `utils/logging_system.rs`.

---

#### **5. `exports/` - Ekspor Data**

**Tugas:** Menyimpan data yang diekspor oleh user dalam berbagai format.

**Tujuan:** Folder ini menyimpan hasil export yang diminta oleh user melalui API atau CLI. Setiap format memiliki subfolder sendiri:

- **`json_exports/`**: Ekspor dalam format JSON. File naming: `{scan_id}_export_{timestamp}.json`. Digunakan ketika user meminta ekspor data mentah dalam format JSON.

- **`txt_exports/`**: Ekspor dalam format TXT. File naming: `{scan_id}_export_{timestamp}.txt`. Digunakan ketika user meminta ekspor ringkasan dalam plain text.

- **`docs_exports/`**: Ekspor dalam format DOCX. File naming: `{scan_id}_export_{timestamp}.docx`. Digunakan ketika user meminta ekspor dokumentasi lengkap.

- **`csv_exports/`**: Ekspor dalam format CSV. File naming: `{scan_id}_export_{timestamp}.csv`. Digunakan ketika user meminta ekspor data tabular.

- **`html_exports/`**: Ekspor dalam format HTML. File naming: `{scan_id}_export_{timestamp}.html`. Digunakan ketika user meminta ekspor laporan interaktif.

- **`pdf_exports/`**: Ekspor dalam format PDF. File naming: `{scan_id}_export_{timestamp}.pdf`. Digunakan ketika user meminta ekspor laporan profesional.

Export dilakukan secara async melalui background task, progress ditampilkan di UI (CLI atau WebSocket). User dapat mengunduh file export melalui API endpoint `GET /api/v1/export/{scan_id}/{format}`. File export memiliki TTL 7 hari, setelah itu dihapus oleh script `cleanup_cache.rs` untuk menghemat ruang.

---

#### **6. `temp/` - Temporary Files**

**Tugas:** Menyimpan file sementara yang digunakan selama processing.

**Tujuan:** Folder ini menyimpan file temporary yang digunakan untuk berbagai operasi processing. Contoh penggunaan: intermediate files saat processing data besar (file CSV yang sedang diproses), temporary downloads (file yang di-download dari internet untuk dianalisis), unpacked archives (file yang di-unzip untuk ekstraksi konten), image processing (screenshot yang di-generate untuk report, kemudian disisipkan ke PDF/HTML). File di sini bersifat sementara dan memiliki lifecycle pendek (biasanya < 24 jam). Dibersihkan secara periodik oleh script `scripts/cleanup_cache.rs` yang menghapus file yang lebih tua dari 24 jam. Folder ini tidak di-version control dan di-ignore oleh `.gitignore`. Lock file digunakan untuk mencegah conflict jika multiple processes mencoba menggunakan file yang sama.

---

**shared folder explanation:**

---

## 📂 **`shared/` - SHARED CONTRACTS & TYPES**

---

### 📂 **`shared/contracts/` - SERVICE CONTRACTS**

#### **1. `scanner_contract.rs`**

**Task:** Define formal contracts for all scanner modules to ensure implementation consistency.

**Purpose:** This file defines the `ScannerContract` trait with method signatures that all scanners must implement: `fn scan_website(&self, url: Url) -> Result<ScanResult, ScannerError>`, `fn scan_with_profile(&self, url: Url, profile: Profile) -> Result<ScanResult, ScannerError>`, `fn cancel_scan(&self, scan_id: Uuid) -> Result<(), ScannerError>`, `fn get_scan_status(&self, scan_id: Uuid) -> Result<ScanStatus, ScannerError>`. Defines **preconditions** with `debug_assert!` to validate input before execution: `debug_assert!(!url.as_str().is_empty())`, `debug_assert!(url.scheme() == "http" || url.scheme() == "https")`, `debug_assert!(profile.threads > 0 && profile.threads <= 200)`. Defines **postconditions**: `assert!(result.start_time <= result.end_time)`, `assert!(result.modules_results.len() > 0)`, `assert!(result.status != ScanStatus::Unknown)`. Defines **invariants**: `scan_id must be unique`, `start_time < end_time`, `progress between 0-100`. Defines error types: `ScannerError::InvalidUrl`, `ScannerError::DomainNotFound`, `ScannerError::ConnectionTimeout`, `ScannerError::ScanCancelled`, `ScannerError::RateLimited`. Adds documentation attributes `#[doc = "..."]` for each method.

---

#### **2. `analyzer_contract.rs`**

**Task:** Define formal contracts for all analyzer components.

**Purpose:** This file defines the `AnalyzerContract` trait with methods: `fn analyze_scan_data(&self, data: ScanResult) -> Result<AnalysisResult, AnalysisError>`, `fn cross_reference_analysis(&self, result: AnalysisResult) -> Result<CrossReferenceResult, AnalysisError>`, `fn calculate_risk(&self, result: AnalysisResult) -> Result<RiskScore, AnalysisError>`. Defines **preconditions**: `debug_assert!(!data.url.is_empty())`, `debug_assert!(data.modules_results.len() > 0)`, `debug_assert!(data.status == ScanStatus::Completed)`. Defines **performance contract**: `const ANALYSIS_TIMEOUT_MINUTES: u64 = 5` - analysis must complete within 5 minutes for standard website (100 pages). If exceeded, considered timeout. Defines **error conditions**: `AnalysisError::IncompleteData` if data incomplete, `AnalysisError::ModelLoadFailed` if AI model fails to load, `AnalysisError::CorrelationFailed` if cross-reference fails, `AnalysisError::TimeoutExceeded` if exceeds 5 minutes. Defines **output guarantees**: `result.findings.len() > 0` (at least one finding), `result.risk_score >= 0.0 && result.risk_score <= 10.0` (CVSS score range).

---

#### **3. `orchestrator_contract.go`**

**Task:** Define formal contracts for orchestrator workflow management.

**Purpose:** This file defines the `OrchestratorContract` interface with methods: `StartWorkflow(req WorkflowRequest) (WorkflowID, error)`, `GetWorkflowStatus(id WorkflowID) (WorkflowStatus, error)`, `PauseWorkflow(id WorkflowID) error`, `ResumeWorkflow(id WorkflowID) error`, `CancelWorkflow(id WorkflowID) error`. Defines **state machine contract** with constant state definitions: `StateIdle = "IDLE"`, `StatePreparing = "PREPARING"`, `StateScanning = "SCANNING"`, `StateAnalyzing = "ANALYZING"`, `StateReporting = "REPORTING"`, `StateComplete = "COMPLETE"`, `StateError = "ERROR"`. Defines **valid state transitions** with `map[State][]State` for validation: `IDLE -> PREPARING`, `PREPARING -> SCANNING`, `SCANNING -> ANALYZING`, `ANALYZING -> REPORTING`, `REPORTING -> COMPLETE`, `ANY -> ERROR`, `ERROR -> IDLE`. Defines **workflow guarantees**: workflow must be idempotent (can be re-run without side effects), workflow must be transactional (all-or-nothing), workflow must be durable (state stored in persistent storage). Defines error types: `ErrWorkflowNotFound`, `ErrWorkflowAlreadyRunning`, `ErrWorkflowPaused`, `ErrWorkflowCancelled`, `ErrInvalidTransition`.

---

#### **4. `storage_contract.rs`**

**Task:** Define formal contracts for all storage operations.

**Purpose:** This file defines the `StorageContract` trait with methods: `fn store_data(&self, key: StorageKey, data: StorageValue) -> Result<(), StorageError>`, `fn retrieve_data(&self, key: StorageKey) -> Result<Option<StorageValue>, StorageError>`, `fn delete_data(&self, key: StorageKey) -> Result<(), StorageError>`, `fn query_data(&self, query: DataQuery) -> Result<Vec<StorageValue>, StorageError>`. Defines **consistency guarantees**: write operations must be strongly consistent (immediately visible to all readers), read operations can be eventually consistent (cached) with `ConsistencyLevel` enum: `StrongConsistency`, `EventualConsistency`, `ReadAfterWriteConsistency`. Defines **durability guarantee**: `async fn durability_check() -> Result<bool>` that verifies data persists across system restarts. Defines **error types**: `StorageError::KeyNotFound`, `StorageError::KeyAlreadyExists`, `StorageError::ConnectionFailed`, `StorageError::IntegrityError`, `StorageError::SerializationError`. Defines **performance SLAs**: store_data < 100ms, retrieve_data < 50ms, delete_data < 100ms, query_data < 500ms for 1000 records.

---

#### **5. `agent_contract.rs`**

**Task:** Define formal contracts for all agents in the system.

**Purpose:** This file defines the `AgentContract` trait with methods: `fn init(&mut self) -> Result<(), AgentError>`, `fn run(&mut self) -> Result<(), AgentError>`, `fn pause(&mut self) -> Result<(), AgentError>`, `fn resume(&mut self) -> Result<(), AgentError>`, `fn shutdown(&mut self) -> Result<(), AgentError>`, `fn get_state(&self) -> AgentState`, `fn send_message(&self, msg: AgentMessage) -> Result<(), AgentError>`. Defines **lifecycle validation** with method `fn validate_transition(from: AgentState, to: AgentState) -> bool` and state transition table: `Uninitialized -> Initialized`, `Initialized -> Running`, `Running -> Paused`, `Paused -> Running`, `Running -> ShuttingDown`, `Paused -> ShuttingDown`, `ShuttingDown -> Shutdown`. Defines **heartbeat contract**: `const HEARTBEAT_INTERVAL_SECONDS: u64 = 30`, `fn send_heartbeat(&self) -> Result<()>` - agent must send heartbeat every 30 seconds or supervisor will consider it dead and restart. Defines **error types**: `AgentError::AlreadyRunning`, `AgentError::AlreadyStopped`, `AgentError::HeartbeatMissed`, `AgentError::StateTransitionInvalid`, `AgentError::MessageDeliveryFailed`.

---

#### **6. `model_contract.py`**

**Task:** Define formal contracts for all AI/ML models.

**Purpose:** This file defines the abstract base class `ModelContract` with methods: `@abstractmethod def load_model(self, config: ModelConfig) -> Model`, `@abstractmethod def predict(self, features: FeatureVector) -> PredictionResult`, `@abstractmethod def explain(self, prediction: PredictionResult) -> Explanation`, `@abstractmethod def train(self, data: TrainingData) -> TrainingResult`, `@abstractmethod def evaluate(self, data: TrainingData) -> EvaluationMetrics`. Defines **input validation decorator**: `@validate_input` that checks feature vector length and data types (all values must be float, no NaN, length matches loaded model). Defines **output schema** with dataclass `PredictionResult` that must have fields: `label: str` (predicted label), `confidence: float` (0-1), `probabilities: Dict[str, float]` (label -> probability). Defines **performance requirements**: predict < 100ms for inference, train < 1 hour for 10,000 sample dataset, evaluate < 5 minutes for 2,000 sample test dataset. Defines error types: `ModelError::LoadFailed`, `ModelError::InvalidInput`, `ModelError::PredictionFailed`, `ModelError::TrainingFailed`.

---

#### **7. `module_contract.rs`**

**Task:** Define formal contracts for all functional modules.

**Purpose:** This file defines the `ModuleContract` trait with methods: `fn execute(&self, input: ModuleInput) -> Result<ModuleOutput, ModuleError>`, `fn validate_config(&self, config: Config) -> Result<ValidationResult, ModuleError>`, `fn get_capabilities(&self) -> Capabilities`, `fn get_version(&self) -> Version`. Defines **capability flags** with bitmask: `CAP_NETWORK_SCAN = 0x01`, `CAP_CONTENT_ANALYSIS = 0x02`, `CAP_SECURITY_CHECK = 0x04`, `CAP_INTELLIGENCE = 0x08`, `CAP_INFRASTRUCTURE = 0x10`. Defines **timeout behavior**: `fn execute_with_timeout(&self, input: ModuleInput, timeout: Duration) -> Result<ModuleOutput>` - every module must support timeout and cancellation. Defines **error handling contract**: every error must implement `ModuleError` trait with methods: `code() -> String` (error code), `message() -> String` (human-readable message), `details() -> Option<serde_json::Value>` (optional detail). Defines **versioning**: `const API_VERSION: &str = "v1.0"` - module must be compatible with API version.

---

#### **8. `integration_contract.rs`**

**Task:** Define formal contracts for all third-party integrations.

**Purpose:** This file defines the `IntegrationContract` trait with methods: `fn connect(&self, config: IntegrationConfig) -> Result<Connection, IntegrationError>`, `fn query(&self, request: Request) -> Result<Response, IntegrationError>`, `fn validate_credentials(&self, creds: Credentials) -> Result<(), IntegrationError>`, `fn disconnect(&self) -> Result<(), IntegrationError>`. Defines **retry policy** with struct `RetryPolicy { max_attempts: u32, initial_backoff: Duration, max_backoff: Duration, multiplier: f32 }` - default: max_attempts=3, initial_backoff=1s, max_backoff=30s, multiplier=2. Defines **rate limiting contract**: `fn check_rate_limit(&self) -> Result<bool>` that returns false if rate limit exceeded (based on tier: free = 1 req/s, paid = 5 req/s). Defines **error types**: `IntegrationError::AuthenticationFailed`, `IntegrationError::RateLimitExceeded`, `IntegrationError::Timeout`, `IntegrationError::InvalidResponse`, `IntegrationError::ServiceUnavailable`. Defines **connection lifecycle**: connect -> (query)* -> disconnect, connection must be reconnected if idle > 5 minutes.

---

#### **9. `api_contract.rs`**

**Task:** Define formal contracts for all API endpoints.

**Purpose:** This file defines the `ApiContract` trait with methods: `fn handle_request(&self, req: ApiRequest) -> ApiResponse`, `fn authenticate(&self, req: ApiRequest) -> Result<AuthResult, ApiError>`, `fn authorize(&self, req: ApiRequest, permission: Permission) -> Result<AuthResult, ApiError>`. Defines **API versioning** with enum `ApiVersion { V1, V2, V3 }` - API must support versioning for backward compatibility. Defines **response format** with struct `ApiResponse<T>`: `status: u16` (HTTP status code), `data: Option<T>` (response data), `error: Option<ApiError>` (error details), `timestamp: DateTime<Utc>` (response timestamp), `request_id: String` (unique request ID for tracing). Defines **error codes**: `AUTH_001` (Invalid token), `AUTH_002` (Token expired), `AUTH_003` (Permission denied), `API_001` (Invalid request), `API_002` (Rate limit exceeded), `API_003` (Internal server error), `API_004` (Resource not found). Defines **API guarantees**: all responses must have `request_id` for tracing, all errors must have `code` and `message`, response time must be < 1s for 95% of requests.

---

### 📂 **`shared/types/` - SHARED DATA TYPES**

#### **1. `common_types.rs`**

**Task:** Define basic data types used throughout the system.

**Purpose:** This file defines fundamental types used globally. `Url` with wrapper struct `#[derive(Debug, Clone, Serialize, Deserialize)]` and custom validation in `new()` method: if scheme is not http/https, returns `InvalidUrl` error. `Domain` with validation `is_ascii() && !is_empty() && !contains(' ')` and `ToAscii` for punycode. `IpAddress` with enum `#[derive(Debug, Clone, Copy, PartialEq, Eq)]` with variants `V4(Ipv4Addr)` and `V6(Ipv6Addr)`. `Timestamp` with `DateTime<Utc>` and `FromStr` impl for ISO 8601 parsing. `Duration` with `std::time::Duration` and methods `to_millis()`, `to_seconds()`. `ScanId` and `UserId` with `Uuid` wrapper. `Severity` enum `#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]` with variants `Critical`, `High`, `Medium`, `Low`, `Info` and `Display` impl for string representation. `Status` enum with variants `Active`, `Completed`, `Failed`, `Cancelled`, `Pending` and `From<&str>` impl for parsing. `Confidence` enum `#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]` with variants `High`, `Medium`, `Low`. `Priority` enum with variants `P0`, `P1`, `P2`, `P3`, `P4` and `rank()` method returning u8 (P0=0, P4=4). Implements `Serialize` and `Deserialize` for all types using `serde`.

---

#### **2. `network_types.rs`**

**Task:** Define data types for all network operations.

**Purpose:** This file defines data structures for network reconnaissance. `DnsRecord` with fields: `record_type: DnsRecordType` (enum A, AAAA, CNAME, MX, TXT, NS, SOA, SRV), `name: String`, `value: String`, `ttl: u32`, `priority: Option<u16>` (for MX and SRV). `PortScanResult` with fields: `port: u16`, `protocol: Protocol` (TCP, UDP), `state: PortState` (Open, Closed, Filtered), `service: Option<String>`, `version: Option<String>` (service version). `WhoisData` with fields: `registrar: String`, `creation_date: DateTime<Utc>`, `expiry_date: DateTime<Utc>`, `updated_date: Option<DateTime<Utc>>`, `nameservers: Vec<String>`, `registrant: Option<Contact>`, `tech_contact: Option<Contact>`, `admin_contact: Option<Contact>`, `status: Vec<String>`. `TracerouteHop` with fields: `hop_number: u32`, `ip: IpAddr`, `hostname: Option<String>`, `rtt_ms: Vec<u64>` (per probe, typically 3 probes), `location: Option<GeoLocation>` (country, city, lat, lon). `SslCertificate` with fields: `subject: String`, `issuer: String`, `valid_from: DateTime<Utc>`, `valid_to: DateTime<Utc>`, `san: Vec<String>` (Subject Alternative Names), `cipher_suites: Vec<CipherSuite>`, `key_type: KeyType` (RSA, ECDSA), `key_size: u16`, `signature_algorithm: String`. Implements `Display` for all types for logging.

---

#### **3. `security_types.rs`**

**Task:** Define data types for security analysis.

**Purpose:** This file defines data structures for security assessment. `HeaderAnalysis` with fields: `header_name: String`, `value: String`, `score: u8` (0-10), `issues: Vec<String>`, `recommendations: Vec<String>`, `is_present: bool`, `is_valid: bool`. `CookieAnalysis` with fields: `name: String`, `secure: bool`, `httponly: bool`, `samesite: SameSite` (Lax, Strict, None), `domain: Option<String>`, `path: Option<String>`, `expires: Option<DateTime<Utc>>`, `is_session: bool` (no expiration). `Vulnerability` with fields: `cve_id: String`, `title: String`, `description: String`, `severity: Severity`, `cvss_score: f32`, `cvss_vector: String`, `affected_software: Vec<SoftwareVersion>` (name, version, version_ranges), `references: Vec<String>`, `remediation: Option<String>`. `XssFinding` with fields: `type: XssType` (Reflected, Stored, DOM), `location: String` (URL or form), `payload: String`, `context: XssContext` (HTML, Attribute, JavaScript), `confidence: Confidence`. `SqlFinding` with fields: `type: SqlType` (Boolean, Time, Error, Union), `parameter: String`, `payload: String`, `error_message: Option<String>`, `confidence: Confidence`. `CsrfFinding` with fields: `form_action: String`, `method: String`, `has_token: bool`, `token_valid: bool`, `samesite_status: SameSite`, `vulnerable: bool`. Implements `to_json()` for all types using `serde_json`.

---

#### **4. `content_types.rs`**

**Task:** Define data types for website content.

**Purpose:** This file defines data structures for content analysis. `ParsedHtml` with fields: `title: Option<String>`, `headings: Vec<Heading>` (level, text, id), `forms: Vec<Form>` (action, method, inputs, id, name), `scripts: Vec<Script>` (src, type, content, async, defer), `images: Vec<Image>` (src, alt, title, width, height), `links: Vec<Link>` (href, rel, target, text). `JsAnalysis` with fields: `framework_detected: Vec<Framework>` (name, version), `api_keys: Vec<ApiKey>` (key, source, line, column), `dangerous_functions: Vec<DangerousFunction>` (name, line, column, context), `library_versions: Vec<LibraryVersion>` (name, version), `obfuscation_detected: bool`, `total_functions: usize`, `total_lines: usize`. `CssInfo` with fields: `total_rules: usize`, `total_selectors: usize`, `frameworks: Vec<Framework>`, `unused_percentage: f32`, `variables: Vec<CssVariable>` (name, value), `imports: Vec<String>`. `MetaData` with fields: `og_tags: HashMap<String, String>`, `twitter_cards: HashMap<String, String>`, `canonical_url: Option<Url>`, `robots: String`, `sitemap: Option<Url>`, `rss_feed: Option<Url>`, `description: Option<String>`, `keywords: Vec<String>`. `LinkGraph` with fields: `internal_links: Vec<Link>`, `external_links: Vec<Link>`, `broken_links: Vec<BrokenLink>` (url, status_code, error), `redirect_chains: Vec<RedirectChain>` (urls, status_codes). Implements `From<&str>` for parsing.

---

#### **5. `infrastructure_types.rs`**

**Task:** Define data types for infrastructure and hosting.

**Purpose:** This file defines data structures for infrastructure intelligence. `ServerInfo` with fields: `server_type: ServerType` (Apache, Nginx, IIS, Lighttpd, Caddy, Unknown), `version: Option<String>`, `os: Option<String>` (Linux, Windows, macOS), `powered_by: Option<String>` (X-Powered-By header), `framework: Option<String>` (backend framework), `platform: Option<String>` (PHP, .NET, Node.js, Python). `CloudProvider` with fields: `provider: CloudProviderType` (AWS, GCP, Azure, DO, Heroku, Unknown), `region: Option<String>` (us-east-1, eu-west-1, etc), `services: Vec<CloudService>` (EC2, S3, CloudFront, GCE, etc), `confidence: f32`. `CdnInfo` with fields: `provider: CdnProvider` (Cloudflare, Akamai, Fastly, CloudFront, GCP, Azure, Unknown), `edge_locations: Vec<String>` (data center codes), `cache_status: CacheStatus` (Hit, Miss, Bypass), `is_cdn: bool`. `LoadBalancerInfo` with fields: `type: LBType` (Hardware, Software, Cloud), `algorithm: LBAlgorithm` (RoundRobin, LeastConn, IPHash, Random), `backend_count: Option<usize>`, `sticky_session: bool`, `detected_via: DetectionMethod` (Header, Cookie, TTL, Pattern). `HostingInfo` with fields: `provider: String`, `isp: String`, `asn: u32`, `country: String`, `city: String`, `latitude: f64`, `longitude: f64`, `timezone: String`, `data_center: Option<String>`. Implements `Display` for logging.

---

#### **6. `intelligence_types.rs`**

**Task:** Define data types for threat intelligence.

**Purpose:** This file defines data structures for threat intelligence. `ThreatData` with fields: `indicators: Vec<Indicator>` (type, value, first_seen, last_seen), `families: Vec<String>` (malware families), `severity: Severity`, `timestamp: DateTime<Utc>`, `sources: Vec<ThreatSource>` (name, confidence, url). `ReputationScore` with fields: `overall_score: u8` (0-100), `categories: Vec<ReputationCategory>` (Malicious, Phishing, Suspicious, Benign, Unknown), `vendors: Vec<VendorResult>` (name, verdict, detection_rate), `total_vendors: usize`, `detections: usize`. `BlacklistStatus` with fields: `listed_in: Vec<BlacklistSource>` (Spamhaus, SURBL, URIBL, DNSBL, SORBS), `categories: Vec<BlacklistCategory>` (Spam, Malware, Phishing, Exploit, Abuse), `reasons: Vec<String>`. `HarvestedEmail` with fields: `email: String`, `category: EmailCategory` (Role, Personal, Generic), `source: EmailSource` (HTML, JS, Mailto, File, Comment), `validation_status: ValidationStatus` (Valid, Invalid, Disposable, RoleBased). Implements `From<serde_json::Value>` for parsing from API responses.

---

#### **7. `agent_types.rs`**

**Task:** Define data types for agent system.

**Purpose:** This file defines data structures for agent management. `AgentState` enum with variants: `Uninitialized`, `Initialized`, `Running`, `Paused`, `ShuttingDown`, `Shutdown`. `AgentMessage` with fields: `sender_id: String`, `recipient_id: String`, `message_type: MessageType` (Heartbeat, StatusRequest, StatusResponse, TaskAssignment, TaskComplete, ErrorReport, LogEntry), `payload: Vec<u8>`, `timestamp: DateTime<Utc>`. `WorkflowRequest` with fields: `scan_target: Url`, `profile: Profile`, `options: WorkflowOptions` (timeout, max_pages, follow_redirects, respect_robots). `WorkflowStatus` with fields: `id: WorkflowID` (UUID), `state: WorkflowState` (IDLE, PREPARING, SCANNING, ANALYZING, REPORTING, COMPLETE, ERROR), `progress: f32`, `start_time: DateTime<Utc>`, `end_time: Option<DateTime<Utc>>`, `error: Option<String>`, `current_step: String`. Implements `Serialize` and `Deserialize` for all types.

---

#### **8. `model_types.py`**

**Task:** Define data types for AI/ML models.

**Purpose:** This file defines dataclasses for model management. `FeatureVector` with `values: List[float]` and `validate()` method to check length and NaN values. `PredictionResult` with fields: `label: str`, `confidence: float` (0-1), `probabilities: Dict[str, float]` (label -> probability). `TrainingData` with fields: `features: List[FeatureVector]`, `labels: List[str]`, `weights: Optional[List[float]]`, `validate()` to check consistency (all features same length). `ModelConfig` with fields: `model_type: str` (RandomForest, SVM, GradientBoosting), `hyperparameters: Dict[str, Any]`, `path: Optional[str]`, `version: str`. `EvaluationMetrics` with fields: `accuracy: float`, `precision: float`, `recall: float`, `f1: float`, `auc_roc: float`, `confusion_matrix: List[List[int]]`. Implements `__post_init__` for validation.

---

#### **9. `storage_types.rs`**

**Task:** Define data types for storage operations.

**Purpose:** This file defines data structures for storage management. `StorageKey` with fields: `namespace: String` (scan, report, config), `id: String`, `to_string()` method for serialization. `StorageValue` with fields: `data: Vec<u8>`, `metadata: ValueMetadata` (content_type, size, created_at, updated_at, version). `DataQuery` with fields: `filters: Vec<QueryFilter>` (field, operator, value), `sort: Option<SortOrder>` (field, direction), `limit: Option<u32>`, `offset: Option<u32>`. `VersionMetadata` with fields: `version: u32`, `timestamp: DateTime<Utc>`, `author: String`, `message: String`. `BackupInfo` with fields: `backup_id: String`, `size: u64`, `created_at: DateTime<Utc>`, `location: BackupLocation` (Local, S3, GCS, Azure), `status: BackupStatus` (Pending, Running, Complete, Failed). Implements `Serialize` and `Deserialize`.

---

#### **10. `report_types.rs`**

**Task:** Define data types for reporting.

**Purpose:** This file defines data structures for report generation. `ReportData` with fields: `title: String`, `summary: Option<String>`, `findings: Vec<Finding>`, `recommendations: Vec<Recommendation>`, `metadata: ReportMetadata` (created_at, generated_by, version, language). `ExecutiveSummary` with fields: `overview: String`, `key_findings: Vec<KeyFinding>` (title, severity, impact), `risk_summary: RiskSummary` (total, critical, high, medium, low), `actions: Vec<ActionItem>` (priority, description, effort). `TechnicalDeepDive` with fields: `detailed_findings: Vec<DetailedFinding>` (description, evidence, poc, impact, remediation), `root_cause: Option<String>`, `affected_systems: Vec<String>`. `VulnerabilityTracker` with fields: `tracked_vulnerabilities: Vec<TrackedVulnerability>` (id, status, opened_at, closed_at, updated_at), `status_history: Vec<StatusChange>`, `sla_compliance: bool`. `TimelineEvent` with fields: `timestamp: DateTime<Utc>`, `event_type: TimelineEventType` (ScanStart, ScanComplete, FindingFound, StatusChange, AlertTriggered), `description: String`, `details: Option<serde_json::Value>`. Implements `ToHtml` and `ToJson`.

---

#### **11. `error_types.rs`**

**Task:** Define all error types used in the system.

**Purpose:** This file defines error types for all components. `ScannerError` enum with variants: `Timeout`, `ConnectionFailed(String)`, `ParseError(String)`, `InvalidTarget(String)`, `RateLimited`, `ScanCancelled`, `InternalError(String)`. `AnalyzerError` enum with variants: `InvalidData(String)`, `IncompleteData`, `ModelError(String)`, `CorrelationFailed(String)`, `TimeoutExceeded`. `StorageError` enum with variants: `NotFound`, `AlreadyExists`, `ConnectionFailed`, `IntegrityError(String)`, `SerializationError(String)`. `AgentError` enum with variants: `AlreadyRunning`, `AlreadyStopped`, `HeartbeatMissed`, `StateTransitionInvalid(String)`, `MessageDeliveryFailed`. `ApiError` enum with variants: `AuthenticationFailed`, `AuthorizationFailed`, `RateLimitExceeded`, `InvalidRequest(String)`, `InternalError(String)`. Each error implements `Display` and `Error` traits, and has `code()` method returning error code string (E1001-E9999) and `severity()` method returning `Severity`. Implements `From` for conversion between error types.

---

#### **12. `config_types.rs`**

**Task:** Define data types for system configuration.

**Purpose:** This file defines data structures for configuration management. `AppConfig` with fields: `env: Environment` (Development, Testing, Production), `debug: bool`, `log_level: LogLevel` (Debug, Info, Warn, Error, Fatal). `DatabaseConfig` with fields: `url: String`, `pool_size: usize` (default 20), `timeout: Duration` (default 30s), `max_lifetime: Duration` (default 10m). `ScanningConfig` with fields: `profiles: HashMap<String, Profile>`, `default_profile: String`, `max_threads: usize`, `default_timeout: Duration`. `ApiConfig` with fields: `host: String` (default "0.0.0.0"), `port: u16` (default 8080), `cors_origins: Vec<String>`, `rate_limit: RateLimitConfig` (requests, window). `IntegrationConfig` with fields: `service_name: String`, `api_key: String`, `endpoint: Url`, `timeout: Duration` (default 30s), `retry_policy: RetryPolicy`. Implements `FromEnv` for loading from environment variables with prefix `IWS_`. Implements `Validate` for configuration validation.

---

### 📂 **`shared/interfaces/` - INTERFACE DEFINITIONS**

#### **1. `scanner_interface.rs`**

**Task:** Define the Scanner trait as the base abstraction for all scanners.

**Purpose:** This file defines the `Scanner` trait with methods `scan(Url) -> ScanResult`, `scan_with_profile(Url, Profile) -> ScanResult`, `cancel(Uuid) -> Result`, `status(Uuid) -> ScanStatus`. Uses `async_trait` for async methods with `Send + Sync` bounds for thread safety. Defines associated type `Error = ScannerError`. Includes `#[must_use]` attribute for all methods returning Result (warns if result is ignored). Provides default method `scan_with_retry(url: Url, profile: Profile, max_retries: u8) -> ScanResult` that calls `scan_with_profile()` and retries on failure with exponential backoff. Includes `fn get_capabilities()` to get scanner capabilities.

---

#### **2. `analyzer_interface.rs`**

**Task:** Define the Analyzer trait for all analyzer components.

**Purpose:** This file defines the `Analyzer` trait with methods `analyze(ScanResult) -> AnalysisResult`, `cross_reference(AnalysisResult) -> CrossReferenceResult`, `calculate_risk(AnalysisResult) -> RiskScore`. Uses `async_trait` with `Send + Sync` bounds. Defines associated type `Error = AnalyzerError`. Adds method `get_analysis_progress() -> f32` for streaming progress (0-100) during analysis. Adds method `get_analysis_stages() -> Vec<AnalysisStage>` to get list of stages and their status. Provides `default` implementation for optional methods.

---

#### **3. `orchestrator_interface.go`**

**Task:** Define the Orchestrator interface for workflow management.

**Purpose:** This file defines the `Orchestrator` interface with methods `StartWorkflow(WorkflowRequest) (WorkflowID, error)`, `GetWorkflowStatus(WorkflowID) (WorkflowStatus, error)`, `PauseWorkflow(WorkflowID) error`, `ResumeWorkflow(WorkflowID) error`, `CancelWorkflow(WorkflowID) error`. Adds method `On(event string, callback func(WorkflowStatus))` for event registration (allows subscribers to listen to events). Implements `RegisterWorkflowCallback` with map of callbacks. Adds method `GetWorkflowHistory(WorkflowID) ([]WorkflowStatus, error)` to get status change history. Adds method `ListActiveWorkflows() ([]WorkflowID, error)` to get all active workflows.

---

#### **4. `storage_interface.rs`**

**Task:** Define the Storage trait for data persistence.

**Purpose:** This file defines the `Storage` trait with methods `store(StorageKey, StorageValue) -> Result`, `retrieve(StorageKey) -> Option<StorageValue>`, `delete(StorageKey) -> Result`, `query(DataQuery) -> Vec<StorageValue>`. Uses `async_trait` with `Send + Sync` bounds. Additional methods: `backup(BackupRequest) -> Result<BackupInfo>`, `restore(RestoreRequest) -> Result`, `health_check() -> bool`. Adds method `get_metrics()` to get storage metrics (size, entries, qps). Adds method `compact()` to perform compaction/optimization.

---

#### **5. `agent_interface.rs`**

**Task:** Define the Agent trait for all autonomous agents.

**Purpose:** This file defines the `Agent` trait with methods `init() -> Result`, `run() -> Result`, `pause() -> Result`, `resume() -> Result`, `shutdown() -> Result`, `get_state() -> AgentState`, `send_message(AgentMessage) -> Result`. Uses `async_trait` with `Send + Sync` bounds. Additional methods: `get_id() -> String` (unique agent ID), `get_type() -> AgentType` (Reconnaissance, Analysis, Reporting, Monitoring, ModelIntegration), `on_event(callback: fn(AgentEvent))` for event handling. Provides `default` implementation for optional methods.

---

#### **6. `module_interface.rs`**

**Task:** Define the Module trait for all functional modules.

**Purpose:** This file defines the `Module` trait with methods `execute(ModuleInput) -> ModuleOutput`, `validate_config(Config) -> ValidationResult`, `get_capabilities() -> Capabilities`, `get_version() -> Version`. Uses `async_trait` with `Send + Sync` bounds. Additional methods: `get_dependencies() -> Vec<String>` (dependencies on other modules), `get_requirements() -> Vec<Requirement>` (system requirements), `get_config_schema() -> JsonSchema` (schema for module configuration). Provides `default` implementation for optional methods.

---

#### **7. `integration_interface.rs`**

**Task:** Define the Integration trait for third-party integrations.

**Purpose:** This file defines the `Integration` trait with methods `connect(IntegrationConfig) -> Connection`, `query(Request) -> Response`, `validate_credentials(Credentials) -> Result`, `disconnect() -> Result`. Uses `async_trait` with `Send + Sync` bounds. Additional methods: `get_rate_limit_status() -> RateLimitStatus` (remaining, reset), `get_health_status() -> HealthStatus` (healthy, degraded, unhealthy), `get_capabilities() -> IntegrationCapabilities` (supported features). Provides `default` implementation for optional methods.

---

#### **8. `reporter_interface.rs`**

**Task:** Define the Reporter trait for report generation.

**Purpose:** This file defines the `Reporter` trait with methods `generate(ReportData) -> Vec<u8>`, `export(ReportData, Format) -> Vec<u8>`, `validate_template(String) -> bool`. Uses `async_trait` with `Send + Sync` bounds. Additional methods: `get_supported_formats() -> Vec<Format>` (JSON, TXT, DOCS, CSV, HTML, PDF), `set_template(String) -> Result`, `get_template() -> String` (get current template), `get_template_variables() -> Vec<String>` (variables available in template). Provides `default` implementation for optional methods.

---

### 📂 **`shared/proto/` - PROTOCOL BUFFERS**

#### **1. `agent_messages.proto`**

**Task:** Define communication protocol for inter-agent messaging.

**Purpose:** This file defines protobuf schema for agent communication. `message AgentMessage { string sender_id = 1; string recipient_id = 2; string message_type = 3; bytes payload = 4; uint64 timestamp = 5; }`. Defines enum `MessageType`: `HEARTBEAT = 0`, `STATUS_REQUEST = 1`, `STATUS_RESPONSE = 2`, `TASK_ASSIGNMENT = 3`, `TASK_COMPLETE = 4`, `ERROR_REPORT = 5`, `LOG_ENTRY = 6`. Adds `oneof` for type-specific payloads: `oneof message_payload { HeartbeatPayload heartbeat = 6; StatusResponse status = 7; TaskAssignment task = 8; TaskComplete complete = 9; ErrorReport error = 10; LogEntry log = 11; }`. Defines service `AgentService { rpc SendMessage(AgentMessage) returns (Ack); rpc StreamMessages(stream AgentMessage) returns (stream AgentMessage); }` for streaming communication.

---

#### **2. `scan_events.proto`**

**Task:** Define communication protocol for scan events.

**Purpose:** This file defines protobuf schema for scan events. `message ScanEvent { string scan_id = 1; string event_type = 2; bytes data = 3; uint64 timestamp = 4; }`. Defines enum `EventType`: `SCAN_STARTED = 0`, `SCAN_PROGRESS = 1`, `SCAN_PAUSED = 2`, `SCAN_RESUMED = 3`, `MODULE_STARTED = 4`, `MODULE_COMPLETED = 5`, `SCAN_COMPLETED = 6`, `SCAN_FAILED = 7`. Adds type-specific payloads: `message ScanProgress { float progress = 1; string status = 2; uint32 pages_done = 3; uint32 pages_total = 4; }`, `message ModuleEvent { string module_name = 1; string status = 2; uint32 duration = 3; }`. Defines service `ScanEventService { rpc SubscribeEvents(SubscribeRequest) returns (stream ScanEvent); rpc EmitEvent(ScanEvent) returns (Ack); }`.

---

#### **3. `analysis_results.proto`**

**Task:** Define communication protocol for analysis results.

**Purpose:** This file defines protobuf schema for analysis results. `message AnalysisResult { string scan_id = 1; repeated Finding findings = 2; RiskScore risk_score = 3; string summary = 4; uint64 timestamp = 5; }`. Defines `Finding` with `oneof` for finding types: `message Finding { string id = 1; string type = 2; Severity severity = 3; string description = 4; oneof detail { SecurityFinding security = 5; ContentFinding content = 6; InfrastructureFinding infrastructure = 7; IntelligenceFinding intelligence = 8; } }`. Defines `RiskScore`: `float base_score = 1; float temporal_score = 2; float environmental_score = 3; float business_score = 4; string priority = 5`. Defines `Severity` enum: `CRITICAL = 0`, `HIGH = 1`, `MEDIUM = 2`, `LOW = 3`, `INFO = 4`.

---

#### **4. `report_data.proto`**

**Task:** Define communication protocol for report data.

**Purpose:** This file defines protobuf schema for report data. `message ReportData { string report_id = 1; string scan_id = 2; ExecutiveSummary summary = 3; TechnicalDeepDive details = 4; VulnerabilityTracker tracker = 5; Timeline timeline = 6; }`. Defines `ExecutiveSummary`: `string overview = 1; repeated KeyFinding findings = 2; RiskSummary risk = 3; repeated ActionItem actions = 4`. `TechnicalDeepDive`: `repeated DetailedFinding findings = 1; optional string root_cause = 2; repeated string affected_systems = 3`. `VulnerabilityTracker`: `repeated TrackedVulnerability vulnerabilities = 1; repeated StatusChange history = 2; bool sla_compliance = 3`. `Timeline`: `repeated TimelineEvent events = 1`. Defines `KeyFinding`: `string title = 1; Severity severity = 2; string impact = 3`.

---

#### **5. `api_payloads.proto`**

**Task:** Define communication protocol for API payloads.

**Purpose:** This file defines protobuf schema for API communication. `message ApiRequest { string endpoint = 1; string method = 2; map<string, string> headers = 3; bytes body = 4; }`. `message ApiResponse { int32 status_code = 1; map<string, string> headers = 2; bytes body = 3; string error = 4; }`. Defines payloads: `message ScanRequest { string url = 1; string profile = 2; map<string, string> options = 3; }`. `message ScanResponse { string scan_id = 1; string status = 2; string message = 3; }`. `message StatusRequest { string scan_id = 1; }`. `message StatusResponse { string scan_id = 1; string state = 2; float progress = 3; string current_step = 4; uint64 elapsed_time = 5; uint64 estimated_time = 6; }`. `message ReportRequest { string scan_id = 1; string format = 2; optional ReportOptions options = 3; }`. `message ReportResponse { bytes report_data = 1; string content_type = 2; uint64 size = 3; }`.

---

**indonesian:**

---

## 📂 **`shared/` - SHARED CONTRACTS & TYPES**


---

### 📂 **`shared/contracts/` - KONTRAK LAYANAN**

#### **1. `scanner_contract.rs`**

**Tugas:** Mendefinisikan kontrak formal untuk semua modul scanner yang memastikan konsistensi implementasi.

**Tujuan:** File ini mendefinisikan trait `ScannerContract` dengan method signatures yang harus diimplementasikan oleh semua scanner: `fn scan_website(&self, url: Url) -> Result<ScanResult, ScannerError>`, `fn scan_with_profile(&self, url: Url, profile: Profile) -> Result<ScanResult, ScannerError>`, `fn cancel_scan(&self, scan_id: Uuid) -> Result<(), ScannerError>`, `fn get_scan_status(&self, scan_id: Uuid) -> Result<ScanStatus, ScannerError>`. Mendefinisikan **preconditions** dengan `debug_assert!` untuk memvalidasi input sebelum eksekusi: `debug_assert!(!url.as_str().is_empty())`, `debug_assert!(url.scheme() == "http" || url.scheme() == "https")`, `debug_assert!(profile.threads > 0 && profile.threads <= 200)`. Mendefinisikan **postconditions**: `assert!(result.start_time <= result.end_time)`, `assert!(result.modules_results.len() > 0)`, `assert!(result.status != ScanStatus::Unknown)`. Mendefinisikan **invariants**: `scan_id harus unik`, `start_time < end_time`, `progress antara 0-100`. Mendefinisikan error types: `ScannerError::InvalidUrl`, `ScannerError::DomainNotFound`, `ScannerError::ConnectionTimeout`, `ScannerError::ScanCancelled`, `ScannerError::RateLimited`. Menambahkan documentation attributes `#[doc = "..."]` untuk setiap method.

---

#### **2. `analyzer_contract.rs`**

**Tugas:** Mendefinisikan kontrak formal untuk semua analyzer components.

**Tujuan:** File ini mendefinisikan trait `AnalyzerContract` dengan method: `fn analyze_scan_data(&self, data: ScanResult) -> Result<AnalysisResult, AnalysisError>`, `fn cross_reference_analysis(&self, result: AnalysisResult) -> Result<CrossReferenceResult, AnalysisError>`, `fn calculate_risk(&self, result: AnalysisResult) -> Result<RiskScore, AnalysisError>`. Mendefinisikan **preconditions**: `debug_assert!(!data.url.is_empty())`, `debug_assert!(data.modules_results.len() > 0)`, `debug_assert!(data.status == ScanStatus::Completed)`. Mendefinisikan **performance contract**: `const ANALYSIS_TIMEOUT_MINUTES: u64 = 5` - analysis harus selesai dalam 5 menit untuk website standar (100 pages). Jika tidak, dianggap timeout. Mendefinisikan **error conditions**: `AnalysisError::IncompleteData` jika data tidak lengkap, `AnalysisError::ModelLoadFailed` jika model AI gagal di-load, `AnalysisError::CorrelationFailed` jika cross-reference gagal, `AnalysisError::TimeoutExceeded` jika melebihi 5 menit. Mendefinisikan **output guarantees**: `result.findings.len() > 0` (setidaknya ada satu finding), `result.risk_score >= 0.0 && result.risk_score <= 10.0` (CVSS score range).

---

#### **3. `orchestrator_contract.go`**

**Tugas:** Mendefinisikan kontrak formal untuk orchestrator workflow management.

**Tujuan:** File ini mendefinisikan interface `OrchestratorContract` dengan method: `StartWorkflow(req WorkflowRequest) (WorkflowID, error)`, `GetWorkflowStatus(id WorkflowID) (WorkflowStatus, error)`, `PauseWorkflow(id WorkflowID) error`, `ResumeWorkflow(id WorkflowID) error`, `CancelWorkflow(id WorkflowID) error`. Mendefinisikan **state machine contract** dengan constant state definitions: `StateIdle = "IDLE"`, `StatePreparing = "PREPARING"`, `StateScanning = "SCANNING"`, `StateAnalyzing = "ANALYZING"`, `StateReporting = "REPORTING"`, `StateComplete = "COMPLETE"`, `StateError = "ERROR"`. Mendefinisikan **valid state transitions** dengan `map[State][]State` untuk validasi: `IDLE -> PREPARING`, `PREPARING -> SCANNING`, `SCANNING -> ANALYZING`, `ANALYZING -> REPORTING`, `REPORTING -> COMPLETE`, `ANY -> ERROR`, `ERROR -> IDLE`. Mendefinisikan **workflow guarantees**: workflow harus idempotent (dapat dijalankan ulang tanpa efek samping), workflow harus transactional (all-or-nothing), workflow harus durable (state disimpan di persistent storage). Mendefinisikan error types: `ErrWorkflowNotFound`, `ErrWorkflowAlreadyRunning`, `ErrWorkflowPaused`, `ErrWorkflowCancelled`, `ErrInvalidTransition`.

---

#### **4. `storage_contract.rs`**

**Tugas:** Mendefinisikan kontrak formal untuk semua storage operations.

**Tujuan:** File ini mendefinisikan trait `StorageContract` dengan method: `fn store_data(&self, key: StorageKey, data: StorageValue) -> Result<(), StorageError>`, `fn retrieve_data(&self, key: StorageKey) -> Result<Option<StorageValue>, StorageError>`, `fn delete_data(&self, key: StorageKey) -> Result<(), StorageError>`, `fn query_data(&self, query: DataQuery) -> Result<Vec<StorageValue>, StorageError>`. Mendefinisikan **consistency guarantees**: write operations must be strongly consistent (immediately visible to all readers), read operations can be eventually consistent (cached) dengan `ConsistencyLevel` enum: `StrongConsistency`, `EventualConsistency`, `ReadAfterWriteConsistency`. Mendefinisikan **durability guarantee**: `async fn durability_check() -> Result<bool>` yang memverifikasi data persist across system restarts. Mendefinisikan **error types**: `StorageError::KeyNotFound`, `StorageError::KeyAlreadyExists`, `StorageError::ConnectionFailed`, `StorageError::IntegrityError`, `StorageError::SerializationError`. Mendefinisikan **performance SLAs**: store_data < 100ms, retrieve_data < 50ms, delete_data < 100ms, query_data < 500ms untuk 1000 records.

---

#### **5. `agent_contract.rs`**

**Tugas:** Mendefinisikan kontrak formal untuk semua agents dalam sistem.

**Tujuan:** File ini mendefinisikan trait `AgentContract` dengan method: `fn init(&mut self) -> Result<(), AgentError>`, `fn run(&mut self) -> Result<(), AgentError>`, `fn pause(&mut self) -> Result<(), AgentError>`, `fn resume(&mut self) -> Result<(), AgentError>`, `fn shutdown(&mut self) -> Result<(), AgentError>`, `fn get_state(&self) -> AgentState`, `fn send_message(&self, msg: AgentMessage) -> Result<(), AgentError>`. Mendefinisikan **lifecycle validation** dengan method `fn validate_transition(from: AgentState, to: AgentState) -> bool` dan state transition table: `Uninitialized -> Initialized`, `Initialized -> Running`, `Running -> Paused`, `Paused -> Running`, `Running -> ShuttingDown`, `Paused -> ShuttingDown`, `ShuttingDown -> Shutdown`. Mendefinisikan **heartbeat contract**: `const HEARTBEAT_INTERVAL_SECONDS: u64 = 30`, `fn send_heartbeat(&self) -> Result<()>` - agent harus mengirim heartbeat setiap 30 detik atau supervisor akan menganggap mati dan restart. Mendefinisikan **error types**: `AgentError::AlreadyRunning`, `AgentError::AlreadyStopped`, `AgentError::HeartbeatMissed`, `AgentError::StateTransitionInvalid`, `AgentError::MessageDeliveryFailed`.

---

#### **6. `model_contract.py`**

**Tugas:** Mendefinisikan kontrak formal untuk semua AI/ML models.

**Tujuan:** File ini mendefinisikan abstract base class `ModelContract` dengan method: `@abstractmethod def load_model(self, config: ModelConfig) -> Model`, `@abstractmethod def predict(self, features: FeatureVector) -> PredictionResult`, `@abstractmethod def explain(self, prediction: PredictionResult) -> Explanation`, `@abstractmethod def train(self, data: TrainingData) -> TrainingResult`, `@abstractmethod def evaluate(self, data: TrainingData) -> EvaluationMetrics`. Mendefinisikan **input validation decorator**: `@validate_input` yang memeriksa feature vector length dan data types (semua values harus float, tidak ada NaN, length sesuai dengan model yang di-load). Mendefinisikan **output schema** dengan dataclass `PredictionResult` yang wajib memiliki fields: `label: str` (prediksi label), `confidence: float` (0-1), `probabilities: Dict[str, float]` (label -> probability). Mendefinisikan **performance requirements**: predict < 100ms untuk inference, train < 1 hour untuk dataset 10,000 samples, evaluate < 5 minutes untuk test dataset 2,000 samples. Mendefinisikan error types: `ModelError::LoadFailed`, `ModelError::InvalidInput`, `ModelError::PredictionFailed`, `ModelError::TrainingFailed`.

---

#### **7. `module_contract.rs`**

**Tugas:** Mendefinisikan kontrak formal untuk semua functional modules.

**Tujuan:** File ini mendefinisikan trait `ModuleContract` dengan method: `fn execute(&self, input: ModuleInput) -> Result<ModuleOutput, ModuleError>`, `fn validate_config(&self, config: Config) -> Result<ValidationResult, ModuleError>`, `fn get_capabilities(&self) -> Capabilities`, `fn get_version(&self) -> Version`. Mendefinisikan **capability flags** dengan bitmask: `CAP_NETWORK_SCAN = 0x01`, `CAP_CONTENT_ANALYSIS = 0x02`, `CAP_SECURITY_CHECK = 0x04`, `CAP_INTELLIGENCE = 0x08`, `CAP_INFRASTRUCTURE = 0x10`. Mendefinisikan **timeout behavior**: `fn execute_with_timeout(&self, input: ModuleInput, timeout: Duration) -> Result<ModuleOutput>` - setiap module harus support timeout dan cancelation. Mendefinisikan **error handling contract**: setiap error harus mengimplementasikan `ModuleError` trait dengan methods: `code() -> String` (error code), `message() -> String` (human-readable message), `details() -> Option<serde_json::Value>` (optional detail). Mendefinisikan **versioning**: `const API_VERSION: &str = "v1.0"` - module harus compatible dengan API version.

---

#### **8. `integration_contract.rs`**

**Tugas:** Mendefinisikan kontrak formal untuk semua third-party integrations.

**Tujuan:** File ini mendefinisikan trait `IntegrationContract` dengan method: `fn connect(&self, config: IntegrationConfig) -> Result<Connection, IntegrationError>`, `fn query(&self, request: Request) -> Result<Response, IntegrationError>`, `fn validate_credentials(&self, creds: Credentials) -> Result<(), IntegrationError>`, `fn disconnect(&self) -> Result<(), IntegrationError>`. Mendefinisikan **retry policy** dengan struct `RetryPolicy { max_attempts: u32, initial_backoff: Duration, max_backoff: Duration, multiplier: f32 }` - default: max_attempts=3, initial_backoff=1s, max_backoff=30s, multiplier=2. Mendefinisikan **rate limiting contract**: `fn check_rate_limit(&self) -> Result<bool>` yang mengembalikan false jika rate limit exceeded (berdasarkan tier: free = 1 req/s, paid = 5 req/s). Mendefinisikan **error types**: `IntegrationError::AuthenticationFailed`, `IntegrationError::RateLimitExceeded`, `IntegrationError::Timeout`, `IntegrationError::InvalidResponse`, `IntegrationError::ServiceUnavailable`. Mendefinisikan **connection lifecycle**: connect -> (query)* -> disconnect, connection harus di-reconnect jika idle > 5 menit.

---

#### **9. `api_contract.rs`**

**Tugas:** Mendefinisikan kontrak formal untuk semua API endpoints.

**Tujuan:** File ini mendefinisikan trait `ApiContract` dengan method: `fn handle_request(&self, req: ApiRequest) -> ApiResponse`, `fn authenticate(&self, req: ApiRequest) -> Result<AuthResult, ApiError>`, `fn authorize(&self, req: ApiRequest, permission: Permission) -> Result<AuthResult, ApiError>`. Mendefinisikan **API versioning** dengan enum `ApiVersion { V1, V2, V3 }` - API harus mendukung versioning untuk backward compatibility. Mendefinisikan **response format** dengan struct `ApiResponse<T>`: `status: u16` (HTTP status code), `data: Option<T>` (response data), `error: Option<ApiError>` (error details), `timestamp: DateTime<Utc>` (response timestamp), `request_id: String` (unique request ID untuk tracing). Mendefinisikan **error codes**: `AUTH_001` (Invalid token), `AUTH_002` (Token expired), `AUTH_003` (Permission denied), `API_001` (Invalid request), `API_002` (Rate limit exceeded), `API_003` (Internal server error), `API_004` (Resource not found). Mendefinisikan **API guarantees**: semua response harus memiliki `request_id` untuk tracing, semua error harus memiliki `code` dan `message`, response time harus < 1s untuk 95% requests.

---

### 📂 **`shared/types/` - TIPE DATA BERSAMA**

#### **1. `common_types.rs`**

**Tugas:** Mendefinisikan tipe data dasar yang digunakan di seluruh sistem.

**Tujuan:** File ini mendefinisikan tipe-tipe fundamental yang digunakan secara global. `Url` dengan wrapper struct `#[derive(Debug, Clone, Serialize, Deserialize)]` dan custom validation di `new()` method: jika scheme tidak http/https, return `InvalidUrl` error. `Domain` dengan validation `is_ascii() && !is_empty() && !contains(' ')` dan `ToAscii` untuk punycode. `IpAddress` dengan enum `#[derive(Debug, Clone, Copy, PartialEq, Eq)]` dengan variants `V4(Ipv4Addr)` dan `V6(Ipv6Addr)`. `Timestamp` dengan `DateTime<Utc>` dan `FromStr` impl untuk parsing ISO 8601. `Duration` dengan `std::time::Duration` dan methods `to_millis()`, `to_seconds()`. `ScanId` dan `UserId` dengan `Uuid` wrapper. `Severity` enum `#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]` dengan variants `Critical`, `High`, `Medium`, `Low`, `Info` dan `Display` impl untuk string representation. `Status` enum dengan variants `Active`, `Completed`, `Failed`, `Cancelled`, `Pending` dan `From<&str>` impl untuk parsing. `Confidence` enum `#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]` dengan variants `High`, `Medium`, `Low`. `Priority` enum dengan variants `P0`, `P1`, `P2`, `P3`, `P4` dan `rank()` method mengembalikan u8 (P0=0, P4=4). Implementasi `Serialize` dan `Deserialize` untuk semua tipe menggunakan `serde`.

---

#### **2. `network_types.rs`**

**Tugas:** Mendefinisikan tipe data untuk semua operasi jaringan.

**Tujuan:** File ini mendefinisikan struktur data untuk network reconnaissance. `DnsRecord` dengan fields: `record_type: DnsRecordType` (enum A, AAAA, CNAME, MX, TXT, NS, SOA, SRV), `name: String`, `value: String`, `ttl: u32`, `priority: Option<u16>` (untuk MX dan SRV). `PortScanResult` dengan fields: `port: u16`, `protocol: Protocol` (TCP, UDP), `state: PortState` (Open, Closed, Filtered), `service: Option<String>`, `version: Option<String>` (service version). `WhoisData` dengan fields: `registrar: String`, `creation_date: DateTime<Utc>`, `expiry_date: DateTime<Utc>`, `updated_date: Option<DateTime<Utc>>`, `nameservers: Vec<String>`, `registrant: Option<Contact>`, `tech_contact: Option<Contact>`, `admin_contact: Option<Contact>`, `status: Vec<String>`. `TracerouteHop` dengan fields: `hop_number: u32`, `ip: IpAddr`, `hostname: Option<String>`, `rtt_ms: Vec<u64>` (per probe, biasanya 3 probes), `location: Option<GeoLocation>` (country, city, lat, lon). `SslCertificate` dengan fields: `subject: String`, `issuer: String`, `valid_from: DateTime<Utc>`, `valid_to: DateTime<Utc>`, `san: Vec<String>` (Subject Alternative Names), `cipher_suites: Vec<CipherSuite>`, `key_type: KeyType` (RSA, ECDSA), `key_size: u16`, `signature_algorithm: String`. Implementasi `Display` untuk semua tipe untuk logging.

---

#### **3. `security_types.rs`**

**Tugas:** Mendefinisikan tipe data untuk analisis keamanan.

**Tujuan:** File ini mendefinisikan struktur data untuk security assessment. `HeaderAnalysis` dengan fields: `header_name: String`, `value: String`, `score: u8` (0-10), `issues: Vec<String>`, `recommendations: Vec<String>`, `is_present: bool`, `is_valid: bool`. `CookieAnalysis` dengan fields: `name: String`, `secure: bool`, `httponly: bool`, `samesite: SameSite` (Lax, Strict, None), `domain: Option<String>`, `path: Option<String>`, `expires: Option<DateTime<Utc>>`, `is_session: bool` (tidak ada expiration). `Vulnerability` dengan fields: `cve_id: String`, `title: String`, `description: String`, `severity: Severity`, `cvss_score: f32`, `cvss_vector: String`, `affected_software: Vec<SoftwareVersion>` (name, version, version_ranges), `references: Vec<String>`, `remediation: Option<String>`. `XssFinding` dengan fields: `type: XssType` (Reflected, Stored, DOM), `location: String` (URL atau form), `payload: String`, `context: XssContext` (HTML, Attribute, JavaScript), `confidence: Confidence`. `SqlFinding` dengan fields: `type: SqlType` (Boolean, Time, Error, Union), `parameter: String`, `payload: String`, `error_message: Option<String>`, `confidence: Confidence`. `CsrfFinding` dengan fields: `form_action: String`, `method: String`, `has_token: bool`, `token_valid: bool`, `samesite_status: SameSite`, `vulnerable: bool`. Implementasi `to_json()` untuk semua tipe menggunakan `serde_json`.

---

#### **4. `content_types.rs`**

**Tugas:** Mendefinisikan tipe data untuk konten website.

**Tujuan:** File ini mendefinisikan struktur data untuk content analysis. `ParsedHtml` dengan fields: `title: Option<String>`, `headings: Vec<Heading>` (level, text, id), `forms: Vec<Form>` (action, method, inputs, id, name), `scripts: Vec<Script>` (src, type, content, async, defer), `images: Vec<Image>` (src, alt, title, width, height), `links: Vec<Link>` (href, rel, target, text). `JsAnalysis` dengan fields: `framework_detected: Vec<Framework>` (name, version), `api_keys: Vec<ApiKey>` (key, source, line, column), `dangerous_functions: Vec<DangerousFunction>` (name, line, column, context), `library_versions: Vec<LibraryVersion>` (name, version), `obfuscation_detected: bool`, `total_functions: usize`, `total_lines: usize`. `CssInfo` dengan fields: `total_rules: usize`, `total_selectors: usize`, `frameworks: Vec<Framework>`, `unused_percentage: f32`, `variables: Vec<CssVariable>` (name, value), `imports: Vec<String>`. `MetaData` dengan fields: `og_tags: HashMap<String, String>`, `twitter_cards: HashMap<String, String>`, `canonical_url: Option<Url>`, `robots: String`, `sitemap: Option<Url>`, `rss_feed: Option<Url>`, `description: Option<String>`, `keywords: Vec<String>`. `LinkGraph` dengan fields: `internal_links: Vec<Link>`, `external_links: Vec<Link>`, `broken_links: Vec<BrokenLink>` (url, status_code, error), `redirect_chains: Vec<RedirectChain>` (urls, status_codes). Implementasi `From<&str>` untuk parsing.

---

#### **5. `infrastructure_types.rs`**

**Tugas:** Mendefinisikan tipe data untuk infrastruktur dan hosting.

**Tujuan:** File ini mendefinisikan struktur data untuk infrastructure intelligence. `ServerInfo` dengan fields: `server_type: ServerType` (Apache, Nginx, IIS, Lighttpd, Caddy, Unknown), `version: Option<String>`, `os: Option<String>` (Linux, Windows, macOS), `powered_by: Option<String>` (X-Powered-By header), `framework: Option<String>` (backend framework), `platform: Option<String>` (PHP, .NET, Node.js, Python). `CloudProvider` dengan fields: `provider: CloudProviderType` (AWS, GCP, Azure, DO, Heroku, Unknown), `region: Option<String>` (us-east-1, eu-west-1, etc), `services: Vec<CloudService>` (EC2, S3, CloudFront, GCE, etc), `confidence: f32`. `CdnInfo` dengan fields: `provider: CdnProvider` (Cloudflare, Akamai, Fastly, CloudFront, GCP, Azure, Unknown), `edge_locations: Vec<String>` (data center codes), `cache_status: CacheStatus` (Hit, Miss, Bypass), `is_cdn: bool`. `LoadBalancerInfo` dengan fields: `type: LBType` (Hardware, Software, Cloud), `algorithm: LBAlgorithm` (RoundRobin, LeastConn, IPHash, Random), `backend_count: Option<usize>`, `sticky_session: bool`, `detected_via: DetectionMethod` (Header, Cookie, TTL, Pattern). `HostingInfo` dengan fields: `provider: String`, `isp: String`, `asn: u32`, `country: String`, `city: String`, `latitude: f64`, `longitude: f64`, `timezone: String`, `data_center: Option<String>`. Implementasi `Display` untuk logging.

---

#### **6. `intelligence_types.rs`**

**Tugas:** Mendefinisikan tipe data untuk threat intelligence.

**Tujuan:** File ini mendefinisikan struktur data untuk threat intelligence. `ThreatData` dengan fields: `indicators: Vec<Indicator>` (type, value, first_seen, last_seen), `families: Vec<String>` (malware families), `severity: Severity`, `timestamp: DateTime<Utc>`, `sources: Vec<ThreatSource>` (name, confidence, url). `ReputationScore` dengan fields: `overall_score: u8` (0-100), `categories: Vec<ReputationCategory>` (Malicious, Phishing, Suspicious, Benign, Unknown), `vendors: Vec<VendorResult>` (name, verdict, detection_rate), `total_vendors: usize`, `detections: usize`. `BlacklistStatus` dengan fields: `listed_in: Vec<BlacklistSource>` (Spamhaus, SURBL, URIBL, DNSBL, SORBS), `categories: Vec<BlacklistCategory>` (Spam, Malware, Phishing, Exploit, Abuse), `reasons: Vec<String>`. `HarvestedEmail` dengan fields: `email: String`, `category: EmailCategory` (Role, Personal, Generic), `source: EmailSource` (HTML, JS, Mailto, File, Comment), `validation_status: ValidationStatus` (Valid, Invalid, Disposable, RoleBased). Implementasi `From<serde_json::Value>` untuk parsing dari API responses.

---

#### **7. `agent_types.rs`**

**Tugas:** Mendefinisikan tipe data untuk agent system.

**Tujuan:** File ini mendefinisikan struktur data untuk agent management. `AgentState` enum dengan variants: `Uninitialized`, `Initialized`, `Running`, `Paused`, `ShuttingDown`, `Shutdown`. `AgentMessage` dengan fields: `sender_id: String`, `recipient_id: String`, `message_type: MessageType` (Heartbeat, StatusRequest, StatusResponse, TaskAssignment, TaskComplete, ErrorReport, LogEntry), `payload: Vec<u8>`, `timestamp: DateTime<Utc>`. `WorkflowRequest` dengan fields: `scan_target: Url`, `profile: Profile`, `options: WorkflowOptions` (timeout, max_pages, follow_redirects, respect_robots). `WorkflowStatus` dengan fields: `id: WorkflowID` (UUID), `state: WorkflowState` (IDLE, PREPARING, SCANNING, ANALYZING, REPORTING, COMPLETE, ERROR), `progress: f32`, `start_time: DateTime<Utc>`, `end_time: Option<DateTime<Utc>>`, `error: Option<String>`, `current_step: String`. Implementasi `Serialize` dan `Deserialize` untuk semua tipe.

---

#### **8. `model_types.py`**

**Tugas:** Mendefinisikan tipe data untuk AI/ML models.

**Tujuan:** File ini mendefinisikan dataclass untuk model management. `FeatureVector` dengan `values: List[float]` dan `validate()` method untuk memeriksa panjang dan NaN values. `PredictionResult` dengan fields: `label: str`, `confidence: float` (0-1), `probabilities: Dict[str, float]` (label -> probability). `TrainingData` dengan fields: `features: List[FeatureVector]`, `labels: List[str]`, `weights: Optional[List[float]]`, `validate()` untuk memeriksa konsistensi (semua features sama panjang). `ModelConfig` dengan fields: `model_type: str` (RandomForest, SVM, GradientBoosting), `hyperparameters: Dict[str, Any]`, `path: Optional[str]`, `version: str`. `EvaluationMetrics` dengan fields: `accuracy: float`, `precision: float`, `recall: float`, `f1: float`, `auc_roc: float`, `confusion_matrix: List[List[int]]`. Implementasi `__post_init__` untuk validasi.

---

#### **9. `storage_types.rs`**

**Tugas:** Mendefinisikan tipe data untuk storage operations.

**Tujuan:** File ini mendefinisikan struktur data untuk storage management. `StorageKey` dengan fields: `namespace: String` (scan, report, config), `id: String`, `to_string()` method untuk serialization. `StorageValue` dengan fields: `data: Vec<u8>`, `metadata: ValueMetadata` (content_type, size, created_at, updated_at, version). `DataQuery` dengan fields: `filters: Vec<QueryFilter>` (field, operator, value), `sort: Option<SortOrder>` (field, direction), `limit: Option<u32>`, `offset: Option<u32>`. `VersionMetadata` dengan fields: `version: u32`, `timestamp: DateTime<Utc>`, `author: String`, `message: String`. `BackupInfo` dengan fields: `backup_id: String`, `size: u64`, `created_at: DateTime<Utc>`, `location: BackupLocation` (Local, S3, GCS, Azure), `status: BackupStatus` (Pending, Running, Complete, Failed). Implementasi `Serialize` dan `Deserialize`.

---

#### **10. `report_types.rs`**

**Tugas:** Mendefinisikan tipe data untuk reporting.

**Tujuan:** File ini mendefinisikan struktur data untuk report generation. `ReportData` dengan fields: `title: String`, `summary: Option<String>`, `findings: Vec<Finding>`, `recommendations: Vec<Recommendation>`, `metadata: ReportMetadata` (created_at, generated_by, version, language). `ExecutiveSummary` dengan fields: `overview: String`, `key_findings: Vec<KeyFinding>` (title, severity, impact), `risk_summary: RiskSummary` (total, critical, high, medium, low), `actions: Vec<ActionItem>` (priority, description, effort). `TechnicalDeepDive` dengan fields: `detailed_findings: Vec<DetailedFinding>` (description, evidence, poc, impact, remediation), `root_cause: Option<String>`, `affected_systems: Vec<String>`. `VulnerabilityTracker` dengan fields: `tracked_vulnerabilities: Vec<TrackedVulnerability>` (id, status, opened_at, closed_at, updated_at), `status_history: Vec<StatusChange>`, `sla_compliance: bool`. `TimelineEvent` dengan fields: `timestamp: DateTime<Utc>`, `event_type: TimelineEventType` (ScanStart, ScanComplete, FindingFound, StatusChange, AlertTriggered), `description: String`, `details: Option<serde_json::Value>`. Implementasi `ToHtml` dan `ToJson`.

---

#### **11. `error_types.rs`**

**Tugas:** Mendefinisikan semua tipe error yang digunakan di sistem.

**Tujuan:** File ini mendefinisikan error types untuk semua komponen. `ScannerError` enum dengan variants: `Timeout`, `ConnectionFailed(String)`, `ParseError(String)`, `InvalidTarget(String)`, `RateLimited`, `ScanCancelled`, `InternalError(String)`. `AnalyzerError` enum dengan variants: `InvalidData(String)`, `IncompleteData`, `ModelError(String)`, `CorrelationFailed(String)`, `TimeoutExceeded`. `StorageError` enum dengan variants: `NotFound`, `AlreadyExists`, `ConnectionFailed`, `IntegrityError(String)`, `SerializationError(String)`. `AgentError` enum dengan variants: `AlreadyRunning`, `AlreadyStopped`, `HeartbeatMissed`, `StateTransitionInvalid(String)`, `MessageDeliveryFailed`. `ApiError` enum dengan variants: `AuthenticationFailed`, `AuthorizationFailed`, `RateLimitExceeded`, `InvalidRequest(String)`, `InternalError(String)`. Setiap error mengimplementasikan `Display` dan `Error` traits, dan memiliki `code()` method mengembalikan string error code (E1001-E9999) dan `severity()` method mengembalikan `Severity`. Implementasi `From` untuk konversi antar error types.

---

#### **12. `config_types.rs`**

**Tugas:** Mendefinisikan tipe data untuk konfigurasi sistem.

**Tujuan:** File ini mendefinisikan struktur data untuk configuration management. `AppConfig` dengan fields: `env: Environment` (Development, Testing, Production), `debug: bool`, `log_level: LogLevel` (Debug, Info, Warn, Error, Fatal). `DatabaseConfig` dengan fields: `url: String`, `pool_size: usize` (default 20), `timeout: Duration` (default 30s), `max_lifetime: Duration` (default 10m). `ScanningConfig` dengan fields: `profiles: HashMap<String, Profile>`, `default_profile: String`, `max_threads: usize`, `default_timeout: Duration`. `ApiConfig` dengan fields: `host: String` (default "0.0.0.0"), `port: u16` (default 8080), `cors_origins: Vec<String>`, `rate_limit: RateLimitConfig` (requests, window). `IntegrationConfig` dengan fields: `service_name: String`, `api_key: String`, `endpoint: Url`, `timeout: Duration` (default 30s), `retry_policy: RetryPolicy`. Implementasi `FromEnv` untuk loading dari environment variables dengan prefix `IWS_`. Implementasi `Validate` untuk validasi konfigurasi.

---

### 📂 **`shared/interfaces/` - INTERFACE DEFINITIONS**

#### **1. `scanner_interface.rs`**

**Tugas:** Mendefinisikan trait Scanner yang menjadi base abstraction untuk semua scanner.

**Tujuan:** File ini mendefinisikan trait `Scanner` dengan method `scan(Url) -> ScanResult`, `scan_with_profile(Url, Profile) -> ScanResult`, `cancel(Uuid) -> Result`, `status(Uuid) -> ScanStatus`. Menggunakan `async_trait` untuk async methods dengan `Send + Sync` bounds untuk thread safety. Mendefinisikan associated type `Error = ScannerError`. Menyertakan `#[must_use]` attribute untuk semua method yang mengembalikan Result (memperingatkan jika result diabaikan). Implementasi default method `scan_with_retry(url: Url, profile: Profile, max_retries: u8) -> ScanResult` yang memanggil `scan_with_profile()` dan retry jika gagal dengan exponential backoff. Menyertakan `fn get_capabilities()` untuk mendapatkan capabilities scanner.

---

#### **2. `analyzer_interface.rs`**

**Tugas:** Mendefinisikan trait Analyzer untuk semua analyzer components.

**Tujuan:** File ini mendefinisikan trait `Analyzer` dengan method `analyze(ScanResult) -> AnalysisResult`, `cross_reference(AnalysisResult) -> CrossReferenceResult`, `calculate_risk(AnalysisResult) -> RiskScore`. Menggunakan `async_trait` dengan `Send + Sync` bounds. Mendefinisikan associated type `Error = AnalyzerError`. Menambahkan method `get_analysis_progress() -> f32` untuk streaming progress (0-100) selama analisis berlangsung. Menambahkan method `get_analysis_stages() -> Vec<AnalysisStage>` untuk mendapatkan daftar stage dan statusnya. Implementasi `default` untuk method yang opsional.

---

#### **3. `orchestrator_interface.go`**

**Tugas:** Mendefinisikan interface Orchestrator untuk workflow management.

**Tujuan:** File ini mendefinisikan interface `Orchestrator` dengan method `StartWorkflow(WorkflowRequest) (WorkflowID, error)`, `GetWorkflowStatus(WorkflowID) (WorkflowStatus, error)`, `PauseWorkflow(WorkflowID) error`, `ResumeWorkflow(WorkflowID) error`, `CancelWorkflow(WorkflowID) error`. Menambahkan method `On(event string, callback func(WorkflowStatus))` untuk event registration (memungkinkan subscriber mendengar event). Implementasi `RegisterWorkflowCallback` dengan map of callbacks. Menambahkan method `GetWorkflowHistory(WorkflowID) ([]WorkflowStatus, error)` untuk mendapatkan history status changes. Menambahkan method `ListActiveWorkflows() ([]WorkflowID, error)` untuk mendapatkan semua workflow yang aktif.

---

#### **4. `storage_interface.rs`**

**Tugas:** Mendefinisikan trait Storage untuk data persistence.

**Tujuan:** File ini mendefinisikan trait `Storage` dengan method `store(StorageKey, StorageValue) -> Result`, `retrieve(StorageKey) -> Option<StorageValue>`, `delete(StorageKey) -> Result`, `query(DataQuery) -> Vec<StorageValue>`. Menggunakan `async_trait` dengan `Send + Sync` bounds. Method tambahan: `backup(BackupRequest) -> Result<BackupInfo>`, `restore(RestoreRequest) -> Result`, `health_check() -> bool`. Menambahkan method `get_metrics()` untuk mendapatkan storage metrics (size, entries, qps). Menambahkan method `compact()` untuk melakukan compaction/optimization.

---

#### **5. `agent_interface.rs`**

**Tugas:** Mendefinisikan trait Agent untuk semua autonomous agents.

**Tujuan:** File ini mendefinisikan trait `Agent` dengan method `init() -> Result`, `run() -> Result`, `pause() -> Result`, `resume() -> Result`, `shutdown() -> Result`, `get_state() -> AgentState`, `send_message(AgentMessage) -> Result`. Menggunakan `async_trait` dengan `Send + Sync` bounds. Method tambahan: `get_id() -> String` (unique agent ID), `get_type() -> AgentType` (Reconnaissance, Analysis, Reporting, Monitoring, ModelIntegration), `on_event(callback: fn(AgentEvent))` untuk event handling. Implementasi `default` untuk method yang opsional.

---

#### **6. `module_interface.rs`**

**Tugas:** Mendefinisikan trait Module untuk semua functional modules.

**Tujuan:** File ini mendefinisikan trait `Module` dengan method `execute(ModuleInput) -> ModuleOutput`, `validate_config(Config) -> ValidationResult`, `get_capabilities() -> Capabilities`, `get_version() -> Version`. Menggunakan `async_trait` dengan `Send + Sync` bounds. Method tambahan: `get_dependencies() -> Vec<String>` (dependencies pada module lain), `get_requirements() -> Vec<Requirement>` (system requirements), `get_config_schema() -> JsonSchema` (schema untuk konfigurasi module). Implementasi `default` untuk method yang opsional.

---

#### **7. `integration_interface.rs`**

**Tugas:** Mendefinisikan trait Integration untuk third-party integrations.

**Tujuan:** File ini mendefinisikan trait `Integration` dengan method `connect(IntegrationConfig) -> Connection`, `query(Request) -> Response`, `validate_credentials(Credentials) -> Result`, `disconnect() -> Result`. Menggunakan `async_trait` dengan `Send + Sync` bounds. Method tambahan: `get_rate_limit_status() -> RateLimitStatus` (remaining, reset), `get_health_status() -> HealthStatus` (healthy, degraded, unhealthy), `get_capabilities() -> IntegrationCapabilities` (supported features). Implementasi `default` untuk method yang opsional.

---

#### **8. `reporter_interface.rs`**

**Tugas:** Mendefinisikan trait Reporter untuk report generation.

**Tujuan:** File ini mendefinisikan trait `Reporter` dengan method `generate(ReportData) -> Vec<u8>`, `export(ReportData, Format) -> Vec<u8>`, `validate_template(String) -> bool`. Menggunakan `async_trait` dengan `Send + Sync` bounds. Method tambahan: `get_supported_formats() -> Vec<Format>` (JSON, TXT, DOCS, CSV, HTML, PDF), `set_template(String) -> Result`, `get_template() -> String` (get current template), `get_template_variables() -> Vec<String>` (variables available in template). Implementasi `default` untuk method yang opsional.

---

### 📂 **`shared/proto/` - PROTOCOL BUFFERS**

#### **1. `agent_messages.proto`**

**Tugas:** Mendefinisikan protokol komunikasi untuk inter-agent messaging.

**Tujuan:** File ini mendefinisikan protobuf schema untuk agent communication. `message AgentMessage { string sender_id = 1; string recipient_id = 2; string message_type = 3; bytes payload = 4; uint64 timestamp = 5; }`. Mendefinisikan enum `MessageType`: `HEARTBEAT = 0`, `STATUS_REQUEST = 1`, `STATUS_RESPONSE = 2`, `TASK_ASSIGNMENT = 3`, `TASK_COMPLETE = 4`, `ERROR_REPORT = 5`, `LOG_ENTRY = 6`. Menambahkan `oneof` untuk type-specific payloads: `oneof message_payload { HeartbeatPayload heartbeat = 6; StatusResponse status = 7; TaskAssignment task = 8; TaskComplete complete = 9; ErrorReport error = 10; LogEntry log = 11; }`. Mendefinisikan service `AgentService { rpc SendMessage(AgentMessage) returns (Ack); rpc StreamMessages(stream AgentMessage) returns (stream AgentMessage); }` untuk streaming communication.

---

#### **2. `scan_events.proto`**

**Tugas:** Mendefinisikan protokol komunikasi untuk scan events.

**Tujuan:** File ini mendefinisikan protobuf schema untuk scan events. `message ScanEvent { string scan_id = 1; string event_type = 2; bytes data = 3; uint64 timestamp = 4; }`. Mendefinisikan enum `EventType`: `SCAN_STARTED = 0`, `SCAN_PROGRESS = 1`, `SCAN_PAUSED = 2`, `SCAN_RESUMED = 3`, `MODULE_STARTED = 4`, `MODULE_COMPLETED = 5`, `SCAN_COMPLETED = 6`, `SCAN_FAILED = 7`. Menambahkan type-specific payloads: `message ScanProgress { float progress = 1; string status = 2; uint32 pages_done = 3; uint32 pages_total = 4; }`, `message ModuleEvent { string module_name = 1; string status = 2; uint32 duration = 3; }`. Mendefinisikan service `ScanEventService { rpc SubscribeEvents(SubscribeRequest) returns (stream ScanEvent); rpc EmitEvent(ScanEvent) returns (Ack); }`.

---

#### **3. `analysis_results.proto`**

**Tugas:** Mendefinisikan protokol komunikasi untuk analysis results.

**Tujuan:** File ini mendefinisikan protobuf schema untuk analysis results. `message AnalysisResult { string scan_id = 1; repeated Finding findings = 2; RiskScore risk_score = 3; string summary = 4; uint64 timestamp = 5; }`. Mendefinisikan `Finding` dengan `oneof` untuk finding types: `message Finding { string id = 1; string type = 2; Severity severity = 3; string description = 4; oneof detail { SecurityFinding security = 5; ContentFinding content = 6; InfrastructureFinding infrastructure = 7; IntelligenceFinding intelligence = 8; } }`. Mendefinisikan `RiskScore`: `float base_score = 1; float temporal_score = 2; float environmental_score = 3; float business_score = 4; string priority = 5`. Mendefinisikan `Severity` enum: `CRITICAL = 0`, `HIGH = 1`, `MEDIUM = 2`, `LOW = 3`, `INFO = 4`.

---

#### **4. `report_data.proto`**

**Tugas:** Mendefinisikan protokol komunikasi untuk report data.

**Tujuan:** File ini mendefinisikan protobuf schema untuk report data. `message ReportData { string report_id = 1; string scan_id = 2; ExecutiveSummary summary = 3; TechnicalDeepDive details = 4; VulnerabilityTracker tracker = 5; Timeline timeline = 6; }`. Mendefinisikan `ExecutiveSummary`: `string overview = 1; repeated KeyFinding findings = 2; RiskSummary risk = 3; repeated ActionItem actions = 4`. `TechnicalDeepDive`: `repeated DetailedFinding findings = 1; optional string root_cause = 2; repeated string affected_systems = 3`. `VulnerabilityTracker`: `repeated TrackedVulnerability vulnerabilities = 1; repeated StatusChange history = 2; bool sla_compliance = 3`. `Timeline`: `repeated TimelineEvent events = 1`. Mendefinisikan `KeyFinding`: `string title = 1; Severity severity = 2; string impact = 3`.

---

#### **5. `api_payloads.proto`**

**Tugas:** Mendefinisikan protokol komunikasi untuk API payloads.

**Tujuan:** File ini mendefinisikan protobuf schema untuk API communication. `message ApiRequest { string endpoint = 1; string method = 2; map<string, string> headers = 3; bytes body = 4; }`. `message ApiResponse { int32 status_code = 1; map<string, string> headers = 2; bytes body = 3; string error = 4; }`. Mendefinisikan payloads: `message ScanRequest { string url = 1; string profile = 2; map<string, string> options = 3; }`. `message ScanResponse { string scan_id = 1; string status = 2; string message = 3; }`. `message StatusRequest { string scan_id = 1; }`. `message StatusResponse { string scan_id = 1; string state = 2; float progress = 3; string current_step = 4; uint64 elapsed_time = 5; uint64 estimated_time = 6; }`. `message ReportRequest { string scan_id = 1; string format = 2; optional ReportOptions options = 3; }`. `message ReportResponse { bytes report_data = 1; string content_type = 2; uint64 size = 3; }`.

---

**END**
