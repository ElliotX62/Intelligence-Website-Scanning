// utils/hash_generator.rs
// IWS v1.0 - Hash Generator
// Menyediakan fungsi hash untuk checksum, integrity verification, dan password hashing

use sha2::{Sha256, Sha512, Digest};
use sha3::Sha3_256;
use md5::Md5;
use sha1::Sha1;
use hmac::{Hmac, Mac};
use bcrypt::{hash as bcrypt_hash, verify as bcrypt_verify, DEFAULT_COST};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use std::path::Path;
use std::fs;
use std::io::Read;

// ============================================================
// HASH ALGORITHM
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashAlgorithm {
    MD5,
    SHA1,
    SHA256,
    SHA512,
    SHA3_256,
    HMAC_SHA256,
    BCrypt,
    Argon2id,
}

impl std::fmt::Display for HashAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HashAlgorithm::MD5 => write!(f, "md5"),
            HashAlgorithm::SHA1 => write!(f, "sha1"),
            HashAlgorithm::SHA256 => write!(f, "sha256"),
            HashAlgorithm::SHA512 => write!(f, "sha512"),
            HashAlgorithm::SHA3_256 => write!(f, "sha3-256"),
            HashAlgorithm::HMAC_SHA256 => write!(f, "hmac-sha256"),
            HashAlgorithm::BCrypt => write!(f, "bcrypt"),
            HashAlgorithm::Argon2id => write!(f, "argon2id"),
        }
    }
}

// ============================================================
// HASH GENERATOR
// ============================================================

#[derive(Debug, Clone)]
pub struct HashGenerator {
    pub algorithm: HashAlgorithm,
}

impl HashGenerator {
    pub fn new(algorithm: HashAlgorithm) -> Self {
        HashGenerator { algorithm }
    }

    // ============================================================
    // FAST HASHES (checksum, non-security)
    // ============================================================

    /// MD5 hash (fast, NOT secure for passwords)
    pub fn hash_md5(data: &[u8]) -> [u8; 16] {
        let mut hasher = Md5::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    /// SHA1 hash (NOT secure for passwords)
    pub fn hash_sha1(data: &[u8]) -> [u8; 20] {
        let mut hasher = Sha1::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    // ============================================================
    // SECURE HASHES (integrity verification)
    // ============================================================

    /// SHA256 hash
    pub fn hash_sha256(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    /// SHA512 hash
    pub fn hash_sha512(data: &[u8]) -> [u8; 64] {
        let mut hasher = Sha512::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    /// SHA3-256 hash
    pub fn hash_sha3_256(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha3_256::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    // ============================================================
    // HMAC (message authentication)
    // ============================================================

    /// HMAC-SHA256
    pub fn hash_hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
        let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("HMAC key size");
        mac.update(data);
        mac.finalize().into_bytes().to_vec()
    }

    // ============================================================
    // PASSWORD HASHING (security-critical)
    // ============================================================

    /// BCrypt hash dengan cost factor 12
    pub fn hash_bcrypt(password: &str) -> Result<String, String> {
        bcrypt_hash(password, DEFAULT_COST).map_err(|e| format!("BCrypt hash failed: {}", e))
    }

    /// BCrypt verify
    pub fn verify_bcrypt(password: &str, hash: &str) -> Result<bool, String> {
        bcrypt_verify(password, hash).map_err(|e| format!("BCrypt verify failed: {}", e))
    }

    /// Argon2id hash (memory-hard, lebih aman dari BCrypt)
    pub fn hash_argon2id(password: &str) -> Result<String, String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|h| h.to_string())
            .map_err(|e| format!("Argon2 hash failed: {}", e))
    }

    /// Argon2id verify
    pub fn verify_argon2id(password: &str, hash: &str) -> Result<bool, String> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| format!("Invalid Argon2 hash format: {}", e))?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    // ============================================================
    // FILE HASHING
    // ============================================================

    /// Hash file dengan SHA256
    pub fn hash_file(path: &Path) -> Result<String, String> {
        let mut file = fs::File::open(path)
            .map_err(|e| format!("Cannot open file: {}", e))?;
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 8192];
        loop {
            let bytes_read = file.read(&mut buffer)
                .map_err(|e| format!("Cannot read file: {}", e))?;
            if bytes_read == 0 { break; }
            hasher.update(&buffer[..bytes_read]);
        }
        Ok(hex::encode(hasher.finalize()))
    }

    /// Hash file dengan algoritma pilihan
    pub fn hash_file_with_algorithm(path: &Path, algorithm: HashAlgorithm) -> Result<String, String> {
        let mut file = fs::File::open(path)
            .map_err(|e| format!("Cannot open file: {}", e))?;
        let mut buffer = [0u8; 8192];

        match algorithm {
            HashAlgorithm::MD5 => {
                let mut hasher = Md5::new();
                loop {
                    let n = file.read(&mut buffer).map_err(|e| format!("Read error: {}", e))?;
                    if n == 0 { break; }
                    hasher.update(&buffer[..n]);
                }
                Ok(hex::encode(hasher.finalize()))
            }
            HashAlgorithm::SHA256 => {
                let mut hasher = Sha256::new();
                loop {
                    let n = file.read(&mut buffer).map_err(|e| format!("Read error: {}", e))?;
                    if n == 0 { break; }
                    hasher.update(&buffer[..n]);
                }
                Ok(hex::encode(hasher.finalize()))
            }
            HashAlgorithm::SHA512 => {
                let mut hasher = Sha512::new();
                loop {
                    let n = file.read(&mut buffer).map_err(|e| format!("Read error: {}", e))?;
                    if n == 0 { break; }
                    hasher.update(&buffer[..n]);
                }
                Ok(hex::encode(hasher.finalize()))
            }
            _ => HashGenerator::hash_file(path),
        }
    }

    // ============================================================
    // INTEGRITY VERIFICATION
    // ============================================================

    /// Verifikasi integritas data dengan expected hash
    pub fn verify_integrity(data: &[u8], expected_hash: &str, algorithm: HashAlgorithm) -> bool {
        let actual = match algorithm {
            HashAlgorithm::MD5 => hex::encode(HashGenerator::hash_md5(data)),
            HashAlgorithm::SHA1 => hex::encode(HashGenerator::hash_sha1(data)),
            HashAlgorithm::SHA256 => hex::encode(HashGenerator::hash_sha256(data)),
            HashAlgorithm::SHA512 => hex::encode(HashGenerator::hash_sha512(data)),
            HashAlgorithm::SHA3_256 => hex::encode(HashGenerator::hash_sha3_256(data)),
            _ => return false,
        };
        actual == expected_hash
    }

    /// Verifikasi integritas file
    pub fn verify_file_integrity(path: &Path, expected_hash: &str) -> Result<bool, String> {
        let actual = HashGenerator::hash_file(path)?;
        Ok(actual == expected_hash)
    }

    // ============================================================
    // CONVENIENCE METHODS
    // ============================================================

    /// Hash string to hex
    pub fn hash_string_to_hex(data: &str, algorithm: HashAlgorithm) -> String {
        match algorithm {
            HashAlgorithm::MD5 => hex::encode(HashGenerator::hash_md5(data.as_bytes())),
            HashAlgorithm::SHA1 => hex::encode(HashGenerator::hash_sha1(data.as_bytes())),
            HashAlgorithm::SHA256 => hex::encode(HashGenerator::hash_sha256(data.as_bytes())),
            HashAlgorithm::SHA512 => hex::encode(HashGenerator::hash_sha512(data.as_bytes())),
            HashAlgorithm::SHA3_256 => hex::encode(HashGenerator::hash_sha3_256(data.as_bytes())),
            _ => String::new(),
        }
    }

    /// Generate random salt
    pub fn generate_salt(length: usize) -> String {
        use rand::RngCore;
        let mut salt = vec![0u8; length];
        rand::rngs::OsRng.fill_bytes(&mut salt);
        hex::encode(salt)
    }

    /// Derive key dari password menggunakan PBKDF2-style iterasi
    pub fn derive_key(password: &str, salt: &str, iterations: u32) -> Vec<u8> {
        let mut key = password.as_bytes().to_vec();
        key.extend_from_slice(salt.as_bytes());
        for _ in 0..iterations {
            key = HashGenerator::hash_sha256(&key).to_vec();
        }
        key
    }
}

impl Default for HashGenerator {
    fn default() -> Self {
        HashGenerator::new(HashAlgorithm::SHA256)
    }
}

// ============================================================
// UNIT TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_md5() {
        let hash = HashGenerator::hash_md5(b"hello world");
        assert_eq!(hash.len(), 16);
        let hex_str = hex::encode(hash);
        assert_eq!(hex_str.len(), 32);
    }

    #[test]
    fn test_hash_md5_deterministic() {
        let h1 = HashGenerator::hash_md5(b"test");
        let h2 = HashGenerator::hash_md5(b"test");
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_hash_sha1() {
        let hash = HashGenerator::hash_sha1(b"hello");
        assert_eq!(hash.len(), 20);
    }

    #[test]
    fn test_hash_sha256() {
        let hash = HashGenerator::hash_sha256(b"hello world");
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_hash_sha256_different_inputs() {
        let h1 = HashGenerator::hash_sha256(b"hello");
        let h2 = HashGenerator::hash_sha256(b"world");
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_hash_sha512() {
        let hash = HashGenerator::hash_sha512(b"test");
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_hash_sha3_256() {
        let hash = HashGenerator::hash_sha3_256(b"hello");
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_hash_hmac_sha256() {
        let key = b"secret-key";
        let data = b"message";
        let hmac = HashGenerator::hash_hmac_sha256(key, data);
        assert!(!hmac.is_empty());
        // Deterministic
        let hmac2 = HashGenerator::hash_hmac_sha256(key, data);
        assert_eq!(hmac, hmac2);
    }

    #[test]
    fn test_hash_bcrypt() {
        let hash = HashGenerator::hash_bcrypt("my_password").unwrap();
        assert!(hash.starts_with("$2b$"));
        assert!(HashGenerator::verify_bcrypt("my_password", &hash).unwrap());
        assert!(!HashGenerator::verify_bcrypt("wrong_password", &hash).unwrap());
    }

    #[test]
    fn test_hash_argon2id() {
        let hash = HashGenerator::hash_argon2id("secure_password").unwrap();
        assert!(hash.starts_with("$argon2id$"));
        assert!(HashGenerator::verify_argon2id("secure_password", &hash).unwrap());
        assert!(!HashGenerator::verify_argon2id("wrong", &hash).unwrap());
    }

    #[test]
    fn test_hash_file() {
        let tmp = std::env::temp_dir().join("iws_hash_test.txt");
        fs::write(&tmp, b"test content for hashing").unwrap();
        let hash = HashGenerator::hash_file(&tmp).unwrap();
        assert_eq!(hash.len(), 64);
        fs::remove_file(&tmp).ok();
    }

    #[test]
    fn test_hash_file_nonexistent() {
        let result = HashGenerator::hash_file(Path::new("/nonexistent/file"));
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_integrity() {
        let data = b"verify this data";
        let hash = hex::encode(HashGenerator::hash_sha256(data));
        assert!(HashGenerator::verify_integrity(data, &hash, HashAlgorithm::SHA256));
        assert!(!HashGenerator::verify_integrity(data, "badhash", HashAlgorithm::SHA256));
    }

    #[test]
    fn test_hash_string_to_hex() {
        let hex = HashGenerator::hash_string_to_hex("test", HashAlgorithm::SHA256);
        assert_eq!(hex.len(), 64);
        assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_generate_salt() {
        let salt = HashGenerator::generate_salt(32);
        assert_eq!(salt.len(), 64); // hex encoded
        let salt2 = HashGenerator::generate_salt(32);
        assert_ne!(salt, salt2); // Random
    }

    #[test]
    fn test_derive_key() {
        let key = HashGenerator::derive_key("password", "salt", 1000);
        assert!(!key.is_empty());
        assert_eq!(key.len(), 32); // SHA256 output
        // Deterministic
        let key2 = HashGenerator::derive_key("password", "salt", 1000);
        assert_eq!(key, key2);
    }

    #[test]
    fn test_hash_algorithm_display() {
        assert_eq!(HashAlgorithm::SHA256.to_string(), "sha256");
        assert_eq!(HashAlgorithm::BCrypt.to_string(), "bcrypt");
        assert_eq!(HashAlgorithm::Argon2id.to_string(), "argon2id");
    }
}
