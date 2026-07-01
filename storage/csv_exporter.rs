// storage/csv_exporter.rs
// IWS v1.0 - CSV Exporter
// Mengekspor dan mengimpor data dalam format CSV

use std::collections::HashMap;
use csv::{ReaderBuilder, WriterBuilder, QuoteStyle};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct CsvExporter {
    delimiter: u8,
    quote_style: QuoteStyle,
    has_headers: bool,
}

impl CsvExporter {
    pub fn new() -> Self {
        CsvExporter {
            delimiter: b',',
            quote_style: QuoteStyle::Necessary,
            has_headers: true,
        }
    }

    pub fn with_delimiter(mut self, delimiter: u8) -> Self {
        self.delimiter = delimiter;
        self
    }

    pub fn export<T: Serialize>(&self, records: &[T]) -> Result<String, String> {
        let mut writer = WriterBuilder::new()
            .delimiter(self.delimiter)
            .quote_style(self.quote_style)
            .from_writer(Vec::new());

        for record in records {
            writer.serialize(record).map_err(|e| format!("CSV serialize: {}", e))?;
        }

        let data = writer.into_inner().map_err(|e| format!("CSV finalize: {}", e))?;
        String::from_utf8(data).map_err(|e| format!("UTF-8 error: {}", e))
    }

    pub fn export_to_file<T: Serialize>(&self, records: &[T], path: &str) -> Result<(), String> {
        let mut writer = WriterBuilder::new()
            .delimiter(self.delimiter)
            .quote_style(self.quote_style)
            .from_path(path)
            .map_err(|e| format!("Cannot create file: {}", e))?;

        for record in records {
            writer.serialize(record).map_err(|e| format!("Write error: {}", e))?;
        }
        writer.flush().map_err(|e| format!("Flush error: {}", e))?;
        Ok(())
    }

    pub fn import<T: for<'de> Deserialize<'de>>(&self, csv_data: &str) -> Result<Vec<T>, String> {
        let mut reader = ReaderBuilder::new()
            .delimiter(self.delimiter)
            .has_headers(self.has_headers)
            .from_reader(csv_data.as_bytes());

        reader.deserialize()
            .map(|result| result.map_err(|e| format!("CSV deserialize: {}", e)))
            .collect()
    }

    pub fn import_from_file<T: for<'de> Deserialize<'de>>(&self, path: &str) -> Result<Vec<T>, String> {
        let mut reader = ReaderBuilder::new()
            .delimiter(self.delimiter)
            .has_headers(self.has_headers)
            .from_path(path)
            .map_err(|e| format!("Cannot open file: {}", e))?;

        reader.deserialize()
            .map(|result| result.map_err(|e| format!("CSV deserialize: {}", e)))
            .collect()
    }
}

impl Default for CsvExporter {
    fn default() -> Self { CsvExporter::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestRecord {
        name: String,
        value: i32,
    }

    #[test]
    fn test_export_import_roundtrip() {
        let records = vec![
            TestRecord { name: "Alice".into(), value: 100 },
            TestRecord { name: "Bob".into(), value: 200 },
        ];

        let exporter = CsvExporter::new();
        let csv = exporter.export(&records).unwrap();
        let imported: Vec<TestRecord> = exporter.import(&csv).unwrap();

        assert_eq!(records, imported);
    }

    #[test]
    fn test_export_to_file() {
        let records = vec![TestRecord { name: "Test".into(), value: 42 }];
        let tmp = std::env::temp_dir().join("iws_test.csv");
        let path = tmp.to_str().unwrap();

        let exporter = CsvExporter::new();
        exporter.export_to_file(&records, path).unwrap();
        assert!(tmp.exists());

        let imported: Vec<TestRecord> = exporter.import_from_file(path).unwrap();
        assert_eq!(records, imported);

        std::fs::remove_file(&tmp).ok();
    }
}
