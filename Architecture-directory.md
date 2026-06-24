iws/
├── iws.py
│
├── core/
│   ├── scanner.rs
│   ├── analyzer.rs
│   ├── orchestrator.go
│   ├── extractor.py
│   ├── validator.rs
│   └── engine.rs
│
├── modules/
│   ├── network/
│   │   ├── dns_enum.rs
│   │   ├── port_scanner.go
│   │   ├── whois_lookup.py
│   │   ├── traceroute_analyzer.rs
│   │   └── ssl_cert_analyzer.rs
│   │
│   ├── content/
│   │   ├── html_parser.rs
│   │   ├── js_analyzer.rs
│   │   ├── css_extractor.rs
│   │   ├── meta_extractor.py
│   │   └── link_graph.rs
│   │
│   ├── security/
│   │   ├── header_analyzer.rs
│   │   ├── cookie_scanner.rs
│   │   ├── cve_checker.py
│   │   ├── xss_detector.rs
│   │   ├── sql_injection_detector.rs
│   │   └── csrf_analyzer.rs
│   │
│   ├── infrastructure/
│   │   ├── server_fingerprint.rs
│   │   ├── cloud_detector.go
│   │   ├── cdn_detector.rs
│   │   ├── load_balancer_detector.rs
│   │   └── hosting_provider_lookup.rs
│   │
│   └── intelligence/
│       ├── threat_intel_integration.rs
│       ├── reputation_scanner.go
│       ├── domain_blacklist_checker.rs
│       └── email_harvester.py
│
├── agents/
│   ├── base_agent.rs
│   ├── reconnaissance_agent.go
│   ├── analysis_agent.rs
│   ├── reporting_agent.py
│   ├── monitoring_agent.go
│   └── model_integration.rs
│
├── models/
│   ├── ml_scanner.py
│   ├── anomaly_detector.rs
│   ├── pattern_recognizer.rs
│   ├── nlp_processor.py
│   └── risk_scorer.rs
│
├── storage/
│   ├── data_warehouse.rs
│   ├── json_handler.rs
│   ├── txt_generator.rs
│   ├── docs_builder.rs
│   ├── csv_exporter.rs
│   ├── html_reporter.rs
│   └── pdf_generator.py
│
├── utils/
│   ├── request_handler.rs
│   ├── proxy_manager.go
│   ├── rate_limiter.rs
│   ├── concurrency_manager.rs
│   ├── logging_system.rs
│   ├── encryption.rs
│   ├── hash_generator.rs
│   └── validator_utils.rs
│
├── config/
│   ├── settings.rs
│   ├── apikeys_template.py
│   ├── user_agents.rs
│   ├── scanning_profiles.go
│   └── webhook_configs.rs
│
├── database/
│   ├── schema.sql
│   ├── connection_pool.rs
│   ├── orm_models.py
│   ├── query_builder.rs
│   └── migrations.rs
│
├── api/
│   ├── rest_api.rs
│   ├── websocket_handler.rs
│   ├── authentication.rs
│   ├── rate_limiter_middleware.rs
│   └── endpoint_routes.rs
│
├── reports/
│   ├── report_factory.rs
│   ├── executive_summary_builder.rs
│   ├── technical_deep_dive.rs
│   ├── vulnerability_tracker.rs
│   ├── timeline_generator.rs
│   └── graph_visualizer.py
│
├── terminal/
│   ├── cli_interface.rs
│   ├── termux_support.rs
│   ├── desktop_support.go
│   ├── progress_display.rs
│   └── interactive_shell.rs
│
├── integration/
│   ├── shodan_wrapper.rs
│   ├── censys_connector.rs
│   ├── virustotal_adapter.rs
│   ├── alienvault_otx.rs
│   ├── urlscan_integration.rs
│   ├── securitytrails_client.rs
│   ├── crtsh_wrapper.rs
│   ├── dnsdb_client.rs
│   └── greyhat_warfare.rs
│
├── deployment/
│   ├── dockerfile
│   ├── docker-compose.yml
│   ├── kubernetes_deployment.yaml
│   ├── nginx_config.conf
│   ├── systemd_service.rs
│   └── install.sh
│
├── tests/
│   ├── test_scanner.rs
│   ├── test_analyzer.rs
│   ├── test_integration.rs
│   ├── test_security_modules.rs
│   └── test_agents.rs
│
├── docs/
│   ├── architecture.md
│   ├── deployment_guide.md
│   ├── scanning_profiles.md
│   ├── api_documentation.md
│   ├── troubleshooting.md
│   └── contributions.md
│
├── scripts/
│   ├── init_database.py
│   ├── migrate_schema.py
│   ├── backup_storage.rs
│   ├── cleanup_cache.rs
│   └── generate_analysis_report.rs
│
├── data/
│   ├── scans/
│   │   ├── active/
│   │   ├── completed/
│   │   └── archived/
│   ├── reports/
│   │   ├── json/
│   │   ├── txt/
│   │   ├── docs/
│   │   ├── csv/
│   │   ├── html/
│   │   └── pdf/
│   ├── cache/
│   │   ├── dns_cache.db
│   │   ├── html_cache.db
│   │   └── entity_cache.db
│   ├── logs/
│   │   ├── access.log
│   │   ├── error.log
│   │   ├── scanner_activity.log
│   │   └── agent_trace.log
│   ├── exports/
│   │   ├── json_exports/
│   │   ├── txt_exports/
│   │   ├── docs_exports/
│   │   ├── csv_exports/
│   │   ├── html_exports/
│   │   └── pdf_exports/
│   └── temp/
│
├── main.py
├── Cargo.toml
├── go.mod
├── requirements.txt
├── setup.py
├── .env_template
├── .gitignore
├── README.md
└── LICENSE
└── shared/
    ├── contracts/
    │   ├── scanner_contract.rs
    │   ├── analyzer_contract.rs
    │   ├── orchestrator_contract.go
    │   ├── storage_contract.rs
    │   ├── agent_contract.rs
    │   ├── model_contract.py
    │   ├── module_contract.rs
    │   ├── integration_contract.rs
    │   └── api_contract.rs
    │
    ├── types/
    │   ├── common_types.rs
    │   ├── network_types.rs
    │   ├── security_types.rs
    │   ├── content_types.rs
    │   ├── infrastructure_types.rs
    │   ├── intelligence_types.rs
    │   ├── agent_types.rs
    │   ├── model_types.py
    │   ├── storage_types.rs
    │   ├── report_types.rs
    │   ├── error_types.rs
    │   └── config_types.rs
    │
    ├── interfaces/
    │   ├── scanner_interface.rs
    │   ├── analyzer_interface.rs
    │   ├── orchestrator_interface.go
    │   ├── storage_interface.rs
    │   ├── agent_interface.rs
    │   ├── module_interface.rs
    │   ├── integration_interface.rs
    │   └── reporter_interface.rs
    │
    └── proto/
        ├── agent_messages.proto
        ├── scan_events.proto
        ├── analysis_results.proto
        ├── report_data.proto
        └── api_payloads.proto
