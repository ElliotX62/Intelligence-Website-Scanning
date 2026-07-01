# models/nlp_processor.py
# IWS v1.0 - NLP Processor
# Memproses natural language untuk ekstraksi informasi dan analisis konten

import re
from typing import Dict, List, Optional, Any, Tuple
from dataclasses import dataclass, field
from collections import Counter
import json


@dataclass
class NlpEntity:
    text: str
    entity_type: str
    confidence: float
    start_pos: int = 0
    end_pos: int = 0


@dataclass
class SentimentResult:
    polarity: float
    sentiment: str
    confidence: float
    details: Dict[str, Any] = field(default_factory=dict)


@dataclass
class NlpSummary:
    summary: str
    keywords: List[str]
    topics: List[str]
    entities: List[NlpEntity]
    sentiment: Optional[SentimentResult] = None


class NlpProcessor:
    def __init__(self):
        self._entity_patterns = {
            "EMAIL": re.compile(r'[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}'),
            "URL": re.compile(r'https?://[^\s<>"]+|www\.[^\s<>"]+'),
            "IP": re.compile(r'\b(?:\d{1,3}\.){3}\d{1,3}\b'),
            "CVE": re.compile(r'CVE-\d{4}-\d{4,}'),
            "PHONE": re.compile(r'\+?[\d\s\(\)-]{7,}\d'),
            "DATE": re.compile(r'\d{4}-\d{2}-\d{2}|\d{2}/\d{2}/\d{4}'),
        }
        self._tech_keywords = [
            "nginx", "apache", "iis", "tomcat", "cloudflare", "aws", "azure", "gcp",
            "docker", "kubernetes", "react", "angular", "vue", "django", "flask",
            "laravel", "spring", "nodejs", "python", "java", "php", "mysql", "postgresql",
        ]

    def extract_entities(self, text: str) -> List[NlpEntity]:
        """Ekstrak entities dari teks menggunakan regex patterns"""
        entities = []

        for entity_type, pattern in self._entity_patterns.items():
            for match in pattern.finditer(text):
                entities.append(NlpEntity(
                    text=match.group(),
                    entity_type=entity_type,
                    confidence=0.9,
                    start_pos=match.start(),
                    end_pos=match.end(),
                ))

        # Ekstrak teknologi
        text_lower = text.lower()
        for tech in self._tech_keywords:
            idx = text_lower.find(tech)
            if idx >= 0:
                entities.append(NlpEntity(
                    text=tech, entity_type="TECHNOLOGY",
                    confidence=0.8, start_pos=idx, end_pos=idx + len(tech),
                ))

        # Deduplicate
        seen = set()
        unique = []
        for e in entities:
            key = (e.text, e.entity_type)
            if key not in seen:
                seen.add(key)
                unique.append(e)
        return unique

    def analyze_sentiment(self, text: str) -> SentimentResult:
        """Analisis sentimen sederhana"""
        positive_words = {"secure", "safe", "protected", "encrypted", "valid", "good", "best", "great", "excellent"}
        negative_words = {"vulnerable", "insecure", "exposed", "dangerous", "critical", "fail", "error", "attack", "malware", "threat"}
        neutral_words = {"note", "info", "notice", "update", "change", "normal"}

        words = set(text.lower().split())
        pos_count = len(words & positive_words)
        neg_count = len(words & negative_words)
        neu_count = len(words & neutral_words)
        total = pos_count + neg_count + neu_count or 1

        polarity = (pos_count - neg_count) / total
        sentiment = "positive" if polarity > 0.1 else "negative" if polarity < -0.1 else "neutral"
        confidence = (pos_count + neg_count) / max(total, 1)

        return SentimentResult(polarity=polarity, sentiment=sentiment, confidence=confidence, details={
            "positive_words": pos_count, "negative_words": neg_count, "neutral_words": neu_count,
        })

    def extract_keywords(self, text: str, top_n: int = 10) -> List[str]:
        """Ekstrak keywords dari teks"""
        words = re.findall(r'\b[a-zA-Z]{3,}\b', text.lower())
        stopwords = {"the", "and", "for", "that", "this", "with", "from", "are", "was", "were", "been", "has", "have", "had", "not", "but", "its", "can", "all", "will", "may", "such", "they", "these", "those"}
        filtered = [w for w in words if w not in stopwords]
        counter = Counter(filtered)
        return [word for word, _ in counter.most_common(top_n)]

    def extract_topics(self, texts: List[str], n_topics: int = 5) -> List[str]:
        """Ekstrak topik dari kumpulan teks (simplified LDA-like)"""
        all_keywords = []
        for text in texts:
            all_keywords.extend(self.extract_keywords(text, top_n=20))
        counter = Counter(all_keywords)
        return [word for word, _ in counter.most_common(n_topics)]

    def summarize(self, text: str, max_sentences: int = 3) -> str:
        """Ringkas teks dengan sentence scoring"""
        sentences = re.split(r'[.!?]+', text)
        sentences = [s.strip() for s in sentences if len(s.strip()) > 10]
        if len(sentences) <= max_sentences:
            return ". ".join(sentences) + "."

        # Score sentences berdasarkan keyword frequency
        keywords = set(self.extract_keywords(text, top_n=15))
        scored = []
        for sent in sentences:
            words = set(sent.lower().split())
            score = len(words & keywords)
            scored.append((sent, score))

        scored.sort(key=lambda x: x[1], reverse=True)
        top = [s for s, _ in scored[:max_sentences]]
        return ". ".join(top) + "."

    def process(self, text: str) -> NlpSummary:
        """Process lengkap — entities, sentiment, keywords, summary"""
        entities = self.extract_entities(text)
        sentiment = self.analyze_sentiment(text)
        keywords = self.extract_keywords(text)
        summary = self.summarize(text)

        return NlpSummary(
            summary=summary,
            keywords=keywords,
            topics=keywords[:5],
            entities=entities,
            sentiment=sentiment,
        )

    def process_batch(self, texts: List[str]) -> List[NlpSummary]:
        """Process multiple texts"""
        return [self.process(t) for t in texts]


if __name__ == "__main__":
    processor = NlpProcessor()
    sample = "The nginx server at admin@example.com (192.168.1.1) has CVE-2024-1234 vulnerability. This is a critical security issue that needs immediate attention."

    result = processor.process(sample)
    print(f"Entities: {[(e.text, e.entity_type) for e in result.entities]}")
    print(f"Sentiment: {result.sentiment}")
    print(f"Keywords: {result.keywords}")
    print(f"Summary: {result.summary}")
