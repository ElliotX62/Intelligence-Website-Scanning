// database/migrations.rs
// IWS v1.0 - Database Migrations
// Mengelola migrasi database untuk version control schema

use std::fs;
use std::path::PathBuf;
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct MigrationManager {
    migrations_dir: PathBuf,
}

#[derive(Debug, Clone)]
pub struct Migration {
    pub name: String,
    pub timestamp: String,
    pub up_sql: String,
    pub down_sql: String,
}

impl MigrationManager {
    pub fn new(migrations_dir: &str) -> Self {
        MigrationManager {
            migrations_dir: PathBuf::from(migrations_dir),
        }
    }

    /// List semua file migrasi di direktori
    pub fn list_migration_files(&self) -> Result<Vec<String>, String> {
        let mut files: Vec<String> = fs::read_dir(&self.migrations_dir)
            .map_err(|e| format!("Cannot read migrations dir: {}", e))?
            .filter_map(|entry| {
                entry.ok().map(|e| e.file_name().to_string_lossy().to_string())
            })
            .filter(|name| name.ends_with(".sql") && !name.starts_with("."))
            .collect();
        files.sort();
        Ok(files)
    }

    /// Parse file migrasi menjadi Migration struct
    pub fn parse_migration(&self, filename: &str) -> Result<Migration, String> {
        let path = self.migrations_dir.join(filename);
        let content = fs::read_to_string(&path)
            .map_err(|e| format!("Cannot read {}: {}", filename, e))?;

        // Extract timestamp dan nama
        let name = filename.trim_end_matches(".sql").to_string();
        let timestamp = name.chars().take(14).collect::<String>();

        // Split UP dan DOWN sections
        let (up_sql, down_sql) = if content.contains("-- +migrate Down") {
            let parts: Vec<&str> = content.splitn(2, "-- +migrate Down").collect();
            let up = parts[0].replace("-- +migrate Up", "").trim().to_string();
            let down = if parts.len() > 1 { parts[1].trim().to_string() } else { String::new() };
            (up, down)
        } else {
            (content.trim().to_string(), String::new())
        };

        Ok(Migration { name, timestamp, up_sql, down_sql })
    }

    /// Generate file migrasi baru
    pub fn create_migration(&self, name: &str) -> Result<String, String> {
        fs::create_dir_all(&self.migrations_dir)
            .map_err(|e| format!("Cannot create dir: {}", e))?;

        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let filename = format!("{}_{}.sql", timestamp, name);
        let template = format!(
            "-- +migrate Up\n-- Migration: {}\n-- Created: {}\n\n-- Write your UP migration SQL here\n\n-- +migrate Down\n-- Write your DOWN (rollback) SQL here\n",
            name, Utc::now().to_rfc3339()
        );

        let path = self.migrations_dir.join(&filename);
        fs::write(&path, template)
            .map_err(|e| format!("Cannot write migration: {}", e))?;

        Ok(filename)
    }

    /// Dapatkan semua migrasi yang sudah diparse
    pub fn get_all_migrations(&self) -> Result<Vec<Migration>, String> {
        let files = self.list_migration_files()?;
        files.iter().map(|f| self.parse_migration(f)).collect()
    }

    /// Validasi integritas semua migrasi
    pub fn validate_migrations(&self) -> Result<Vec<String>, String> {
        let files = self.list_migration_files()?;
        let mut warnings = Vec::new();

        let mut prev_timestamp = String::new();
        for file in &files {
            let migration = self.parse_migration(file)?;

            // Cek timestamp format
            if migration.timestamp.len() != 14 || !migration.timestamp.chars().all(|c| c.is_numeric()) {
                warnings.push(format!("{}: Invalid timestamp format", file));
            }

            // Cek urutan timestamp
            if !prev_timestamp.is_empty() && migration.timestamp <= prev_timestamp {
                warnings.push(format!("{}: Timestamp out of order", file));
            }

            // Cek UP SQL tidak kosong
            if migration.up_sql.is_empty() {
                warnings.push(format!("{}: UP migration is empty", file));
            }

            prev_timestamp = migration.timestamp.clone();
        }

        Ok(warnings)
    }

    /// Backup database sebelum migrasi
    pub fn backup_database(&self, backup_path: &str) -> Result<String, String> {
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let filename = format!("{}/backup_{}.sql", backup_path, timestamp);

        fs::create_dir_all(backup_path)
            .map_err(|e| format!("Cannot create backup dir: {}", e))?;

        // Placeholder — actual implementation uses pg_dump
        fs::write(&filename, "-- Database backup placeholder\n")
            .map_err(|e| format!("Cannot write backup: {}", e))?;

        Ok(filename)
    }

    /// Dapatkan status migrasi
    pub fn status(&self) -> Result<MigrationStatus, String> {
        let files = self.list_migration_files()?;
        let migrations = self.get_all_migrations()?;

        Ok(MigrationStatus {
            total_files: files.len(),
            total_migrations: migrations.len(),
            migrations_dir: self.migrations_dir.to_string_lossy().to_string(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct MigrationStatus {
    pub total_files: usize,
    pub total_migrations: usize,
    pub migrations_dir: String,
}

impl std::fmt::Display for MigrationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Migrations: {} files in {}", self.total_files, self.migrations_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_migration() {
        let tmp = std::env::temp_dir().join("iws_migrations_test");
        let manager = MigrationManager::new(tmp.to_str().unwrap());
        let result = manager.create_migration("add_test_table");
        assert!(result.is_ok());
        let filename = result.unwrap();
        assert!(filename.ends_with(".sql"));
        assert!(filename.contains("add_test_table"));

        // Cleanup
        let path = tmp.join(&filename);
        fs::remove_file(&path).ok();
        fs::remove_dir(&tmp).ok();
    }

    #[test]
    fn test_list_migration_files() {
        let tmp = std::env::temp_dir().join("iws_migrations_list");
        fs::create_dir_all(&tmp).unwrap();

        fs::write(tmp.join("20240101000000_init.sql"), "test").unwrap();
        fs::write(tmp.join("20240102000000_add_users.sql"), "test").unwrap();

        let manager = MigrationManager::new(tmp.to_str().unwrap());
        let files = manager.list_migration_files().unwrap();
        assert_eq!(files.len(), 2);

        fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn test_parse_migration() {
        let tmp = std::env::temp_dir().join("iws_migrations_parse");
        fs::create_dir_all(&tmp).unwrap();

        let content = "-- +migrate Up\nCREATE TABLE test (id SERIAL);\n\n-- +migrate Down\nDROP TABLE test;";
        fs::write(tmp.join("20240101000000_test.sql"), content).unwrap();

        let manager = MigrationManager::new(tmp.to_str().unwrap());
        let migration = manager.parse_migration("20240101000000_test.sql").unwrap();
        assert!(migration.up_sql.contains("CREATE TABLE"));
        assert!(migration.down_sql.contains("DROP TABLE"));

        fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn test_validate_migrations() {
        let tmp = std::env::temp_dir().join("iws_migrations_validate");
        fs::create_dir_all(&tmp).unwrap();

        let content = "-- +migrate Up\nCREATE TABLE a (id INT);\n\n-- +migrate Down\nDROP TABLE a;";
        fs::write(tmp.join("20240101000000_first.sql"), content).unwrap();
        fs::write(tmp.join("20240102000000_second.sql"), content).unwrap();

        let manager = MigrationManager::new(tmp.to_str().unwrap());
        let warnings = manager.validate_migrations().unwrap();
        assert!(warnings.is_empty());

        fs::remove_dir_all(&tmp).ok();
    }
}
