# agents/reporting_agent.py
# IWS v1.0 - Reporting Agent
# Menghasilkan laporan dalam berbagai format

import json
import os
from datetime import datetime
from typing import Dict, List, Optional, Any
from dataclasses import dataclass, field
from enum import Enum


class ReportFormat(str, Enum):
    JSON = "json"
    TXT = "txt"
    DOCS = "docs"
    CSV = "csv"
    HTML = "html"
    PDF = "pdf"


@dataclass
class ReportData:
    title: str
    target_url: str
    scan_id: str
    scan_date: str
    findings: List[Dict[str, Any]]
    risk_score: float
    summary: str = ""
    recommendations: List[str] = field(default_factory=list)
    statistics: Dict[str, Any] = field(default_factory=dict)


class ReportingAgent:
    def __init__(self):
        self._reports: Dict[str, ReportData] = {}
        self._templates: Dict[str, str] = {}
        self._load_default_templates()

    def _load_default_templates(self):
        self._templates["html"] = """<!DOCTYPE html>
<html><head><meta charset="utf-8"><title>{{title}}</title>
<style>body{font-family:Arial;margin:40px}h1{color:#333}.critical{color:red}.high{color:orange}.medium{color:gold}.low{color:green}
table{border-collapse:collapse;width:100%}th,td{border:1px solid #ddd;padding:8px}th{background:#f2f2f2}</style></head>
<body><h1>{{title}}</h1><p><strong>Target:</strong> {{target_url}}</p><p><strong>Risk Score:</strong> {{risk_score}}/10</p>
<h2>Summary</h2><p>{{summary}}</p><h2>Findings ({{findings_count}})</h2><table><tr><th>Title</th><th>Severity</th><th>Type</th><th>Confidence</th></tr>
{{#findings}}<tr><td>{{title}}</td><td class="{{severity}}">{{severity}}</td><td>{{type}}</td><td>{{confidence}}</td></tr>{{/findings}}
</table><h2>Recommendations</h2><ul>{{#recommendations}}<li>{{.}}</li>{{/recommendations}}</ul>
<p><em>Generated: {{generated_at}}</em></p></body></html>"""

    def generate_report(self, data: ReportData, fmt: ReportFormat) -> str:
        if fmt == ReportFormat.JSON:
            return self._generate_json(data)
        elif fmt == ReportFormat.TXT:
            return self._generate_txt(data)
        elif fmt == ReportFormat.CSV:
            return self._generate_csv(data)
        elif fmt == ReportFormat.HTML:
            return self._generate_html(data)
        else:
            return self._generate_json(data)

    def _generate_json(self, data: ReportData) -> str:
        return json.dumps({
            "title": data.title, "target_url": data.target_url,
            "scan_id": data.scan_id, "scan_date": data.scan_date,
            "risk_score": data.risk_score, "summary": data.summary,
            "findings": data.findings, "recommendations": data.recommendations,
            "statistics": data.statistics,
            "generated_at": datetime.utcnow().isoformat(),
        }, indent=2)

    def _generate_txt(self, data: ReportData) -> str:
        lines = [
            f"{'='*60}", f"  {data.title}", f"{'='*60}",
            f"Target: {data.target_url}", f"Scan ID: {data.scan_id}",
            f"Date: {data.scan_date}", f"Risk Score: {data.risk_score}/10",
            f"", f"SUMMARY", f"{data.summary}", f"",
            f"FINDINGS ({len(data.findings)})", f"{'-'*40}",
        ]
        for f in data.findings:
            lines.append(f"  [{f.get('severity','?').upper()}] {f.get('title','')} ({f.get('type','')}) - confidence: {f.get('confidence','?')}")
        lines.append(f"")
        lines.append(f"RECOMMENDATIONS")
        for i, r in enumerate(data.recommendations):
            lines.append(f"  {i+1}. {r}")
        lines.append(f"")
        lines.append(f"Generated: {datetime.utcnow().isoformat()}")
        return "\n".join(lines)

    def _generate_csv(self, data: ReportData) -> str:
        header = "Title,Severity,Type,Confidence,Description"
        rows = [header]
        for f in data.findings:
            rows.append(f"{f.get('title','')},{f.get('severity','')},{f.get('type','')},{f.get('confidence','')},{f.get('description','')[:100]}")
        return "\n".join(rows)

    def _generate_html(self, data: ReportData) -> str:
        template = self._templates.get("html", "<html><body>{{title}}</body></html>")
        html = template.replace("{{title}}", data.title)
        html = html.replace("{{target_url}}", data.target_url)
        html = html.replace("{{risk_score}}", str(data.risk_score))
        html = html.replace("{{summary}}", data.summary)
        html = html.replace("{{findings_count}}", str(len(data.findings)))
        html = html.replace("{{generated_at}}", datetime.utcnow().isoformat())

        # Build findings table
        findings_html = ""
        for f in data.findings:
            findings_html += f"<tr><td>{f.get('title','')}</td><td class=\"{f.get('severity','')}\">{f.get('severity','')}</td><td>{f.get('type','')}</td><td>{f.get('confidence','')}</td></tr>"
        html = html.replace("{{#findings}}...{{/findings}}", findings_html)

        # Build recommendations
        recs_html = "".join(f"<li>{r}</li>" for r in data.recommendations)
        html = html.replace("{{#recommendations}}...{{/recommendations}}", recs_html)

        return html

    def export_report(self, data: ReportData, fmt: ReportFormat, output_dir: str) -> str:
        content = self.generate_report(data, fmt)
        filename = f"report_{data.scan_id}_{datetime.utcnow().strftime('%Y%m%d%H%M%S')}.{fmt.value}"
        path = os.path.join(output_dir, filename)
        os.makedirs(output_dir, exist_ok=True)
        with open(path, 'w') as f:
            f.write(content)
        self._reports[data.scan_id] = data
        return path

    def generate_all_formats(self, data: ReportData, output_dir: str) -> Dict[str, str]:
        paths = {}
        for fmt in ReportFormat:
            paths[fmt.value] = self.export_report(data, fmt, output_dir)
        return paths

    def set_template(self, name: str, template: str) -> None:
        self._templates[name] = template

    def get_report_data(self, scan_id: str) -> Optional[ReportData]:
        return self._reports.get(scan_id)

    def list_reports(self) -> List[str]:
        return list(self._reports.keys())


if __name__ == "__main__":
    agent = ReportingAgent()
    data = ReportData(
        title="Security Scan Report",
        target_url="https://example.com",
        scan_id="scan-123",
        scan_date="2024-01-15T10:30:00Z",
        risk_score=7.5,
        summary="Found 3 vulnerabilities including 1 critical.",
        findings=[
            {"title": "SQL Injection", "severity": "critical", "type": "vulnerability", "confidence": "0.9", "description": "SQL injection in login form"},
            {"title": "Missing HSTS", "severity": "medium", "type": "misconfiguration", "confidence": "0.95", "description": "HSTS header not set"},
            {"title": "XSS in comment", "severity": "high", "type": "vulnerability", "confidence": "0.8", "description": "Reflected XSS in comment field"},
        ],
        recommendations=["Fix SQL injection with parameterized queries", "Enable HSTS header", "Sanitize user input in comments"],
    )
    print(agent.generate_report(data, ReportFormat.TXT))
