// models/pattern_recognizer.rs
// IWS v1.0 - Pattern Recognizer
// Mengenali pola menggunakan Aho-Corasick dan regex matching

use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct PatternMatch {
    pub pattern_name: String,
    pub pattern_type: String,
    pub matches: Vec<MatchLocation>,
    pub total_count: usize,
}

#[derive(Debug, Clone)]
pub struct MatchLocation {
    pub start: usize,
    pub end: usize,
    pub matched_text: String,
    pub context: String,
}

#[derive(Debug, Clone)]
pub struct Pattern {
    pub name: String,
    pub pattern_type: String,
    pub value: String,
}

pub struct PatternRecognizer {
    automaton: Option<AhoCorasick>,
    patterns: Vec<Pattern>,
    regex_cache: HashMap<String, Regex>,
}

impl PatternRecognizer {
    pub fn new() -> Self {
        PatternRecognizer {
            automaton: None,
            patterns: Vec::new(),
            regex_cache: HashMap::new(),
        }
    }

    pub fn add_pattern(&mut self, name: &str, pattern_type: &str, value: &str) {
        self.patterns.push(Pattern {
            name: name.to_string(),
            pattern_type: pattern_type.to_string(),
            value: value.to_string(),
        });
    }

    pub fn add_patterns_batch(&mut self, patterns: &[(String, String, String)]) {
        for (name, ptype, value) in patterns {
            self.add_pattern(name, ptype, value);
        }
    }

    pub fn compile(&mut self) {
        let substrings: Vec<&str> = self.patterns.iter()
            .filter(|p| p.pattern_type == "substring")
            .map(|p| p.value.as_str())
            .collect();

        if !substrings.is_empty() {
            self.automaton = Some(
                AhoCorasickBuilder::new()
                    .match_kind(MatchKind::LeftmostLongest)
                    .build(&substrings)
            );
        }
    }

    pub fn find_patterns(&self, input: &str) -> Vec<PatternMatch> {
        let mut results = Vec::new();

        // Aho-Corasick untuk substring patterns
        if let Some(ref automaton) = self.automaton {
            let substring_patterns: Vec<&Pattern> = self.patterns.iter()
                .filter(|p| p.pattern_type == "substring")
                .collect();

            for mat in automaton.find_iter(input) {
                if let Some(pattern) = substring_patterns.get(mat.pattern().as_usize()) {
                    let start = mat.start();
                    let end = mat.end();
                    let ctx_start = if start > 20 { start - 20 } else { 0 };
                    let ctx_end = (end + 20).min(input.len());

                    let existing = results.iter_mut().find(|r: &&mut PatternMatch| r.pattern_name == pattern.name);
                    if let Some(result) = existing {
                        result.matches.push(MatchLocation {
                            start, end,
                            matched_text: input[start..end].to_string(),
                            context: input[ctx_start..ctx_end].to_string(),
                        });
                        result.total_count += 1;
                    } else {
                        results.push(PatternMatch {
                            pattern_name: pattern.name.clone(),
                            pattern_type: "substring".to_string(),
                            matches: vec![MatchLocation {
                                start, end,
                                matched_text: input[start..end].to_string(),
                                context: input[ctx_start..ctx_end].to_string(),
                            }],
                            total_count: 1,
                        });
                    }
                }
            }
        }

        // Regex patterns
        for pattern in &self.patterns {
            if pattern.pattern_type != "regex" { continue; }

            let re = self.regex_cache.get(&pattern.value).cloned().unwrap_or_else(|| {
                let r = Regex::new(&pattern.value).unwrap();
                self.regex_cache.entry(pattern.value.clone()).or_insert(r).clone()
            });

            let matches: Vec<MatchLocation> = re.find_iter(input).map(|m| {
                let start = m.start();
                let end = m.end();
                let ctx_start = if start > 20 { start - 20 } else { 0 };
                let ctx_end = (end + 20).min(input.len());
                MatchLocation {
                    start, end,
                    matched_text: input[start..end].to_string(),
                    context: input[ctx_start..ctx_end].to_string(),
                }
            }).collect();

            if !matches.is_empty() {
                results.push(PatternMatch {
                    pattern_name: pattern.name.clone(),
                    pattern_type: "regex".to_string(),
                    total_count: matches.len(),
                    matches,
                });
            }
        }

        results
    }

    pub fn find_sequence(&self, sequence: &[&str], input: &str) -> Vec<usize> {
        if sequence.is_empty() { return vec![]; }
        let mut positions = Vec::new();
        let mut current_pos = 0;

        for &pattern in sequence {
            if let Some(pos) = input[current_pos..].find(pattern) {
                positions.push(current_pos + pos);
                current_pos += pos + pattern.len();
            } else {
                return vec![];
            }
        }
        positions
    }

    pub fn behavior_fingerprint(&self, input: &str, signature_db: &HashMap<String, Vec<String>>) -> Option<String> {
        for (behavior_name, signatures) in signature_db {
            let all_match = signatures.iter().all(|sig| input.contains(sig.as_str()));
            if all_match {
                return Some(behavior_name.clone());
            }
        }
        None
    }

    pub fn pattern_frequency(&self, input: &str, window_size: usize) -> HashMap<String, usize> {
        let mut freq = HashMap::new();
        let chars: Vec<char> = input.chars().collect();

        for i in 0..chars.len().saturating_sub(window_size) {
            let window: String = chars[i..i + window_size].iter().collect();
            *freq.entry(window).or_insert(0) += 1;
        }
        freq
    }

    pub fn count(&self) -> usize {
        self.patterns.len()
    }
}

impl Default for PatternRecognizer {
    fn default() -> Self { PatternRecognizer::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_substring_matching() {
        let mut pr = PatternRecognizer::new();
        pr.add_pattern("xss", "substring", "<script>");
        pr.add_pattern("sql", "substring", "SELECT");
        pr.compile();

        let results = pr.find_patterns("test <script>alert(1)</script> and SELECT * FROM users");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_regex_matching() {
        let mut pr = PatternRecognizer::new();
        pr.add_pattern("email", "regex", r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}");
        pr.add_pattern("ip", "regex", r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}");

        let results = pr.find_patterns("Contact: admin@example.com from 192.168.1.1");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_sequence_finding() {
        let pr = PatternRecognizer::new();
        let seq = &["first", "second", "third"];
        let pos = pr.find_sequence(seq, "zero first middle second last third end");
        assert_eq!(pos.len(), 3);
    }

    #[test]
    fn test_behavior_fingerprint() {
        let pr = PatternRecognizer::new();
        let mut db = HashMap::new();
        db.insert("sql_injection".to_string(), vec!["'".to_string(), "OR".to_string(), "1=1".to_string()]);
        db.insert("xss_attack".to_string(), vec!["<script>".to_string(), "alert".to_string()]);

        let result = pr.behavior_fingerprint("test' OR 1=1 --", &db);
        assert_eq!(result, Some("sql_injection".to_string()));
    }
}
