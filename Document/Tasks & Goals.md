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

