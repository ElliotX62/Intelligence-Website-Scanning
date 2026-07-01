// utils/encryption.rs
// IWS v1.0 - Encryption Utilities
// Menyediakan enkripsi dan dekripsi data untuk keamanan storage dan communication

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use rsa::{RsaPrivateKey, RsaPublicKey, Pkcs1v15Encrypt, Oaep};
use sha2::Sha256;
use rand::RngCore;
use std::fs;

// ============================================================
// ENCRYPTION ALGORITHM
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncryptionAlgorithm {
    Aes256Gcm,
    ChaCha20Poly1305,
    RsaOaep,
}

impl std::fmt::Display for EncryptionAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EncryptionAlgorithm::Aes256Gcm => write!(f, "AES-256-GCM"),
            EncryptionAlgorithm::ChaCha20Poly1305 => write!(f, "ChaCha20-Poly1305"),
            EncryptionAlgorithm::RsaOaep => write!(f, "RSA-OAEP"),
        }
    }
}

// ============================================================
// ENCRYPTION STRUCT
// ============================================================

#[derive(Debug, Clone)]
pub struct Encryption {
    pub algorithm: EncryptionAlgorithm,
    key: Vec<u8>,
}

impl Encryption {
    /// Buat instance baru dengan key dari bytes
    pub fn new(algorithm: EncryptionAlgorithm, key: &[u8]) -> Self {
        Encryption {
            algorithm,
            key: key.to_vec(),
        }
    }

    /// Generate key untuk algoritma yang dipilih
    pub fn generate_key(algorithm: EncryptionAlgorithm) -> Vec<u8> {
        match algorithm {
            EncryptionAlgorithm::Aes256Gcm => {
                let mut key = vec![0u8; 32]; // 256 bit
                OsRng.fill_bytes(&mut key);
                key
            }
            EncryptionAlgorithm::ChaCha20Poly1305 => {
                let mut key = vec![0u8; 32];
                OsRng.fill_bytes(&mut key);
                key
            }
            EncryptionAlgorithm::RsaOaep => {
                // RSA key generation (2048 bit)
                let mut rng = OsRng;
                let private_key = RsaPrivateKey::new(&mut rng, 2048)
                    .expect("Failed to generate RSA key");
                // Export private key in PKCS8 DER format
                private_key.to_pkcs8_der().unwrap().as_bytes().to_vec()
            }
        }
    }

    // ============================================================
    // AES-256-GCM
    // ============================================================

    /// Enkripsi data dengan AES-256-GCM
    /// Returns: nonce (12 bytes) + ciphertext + tag (16 bytes)
    pub fn encrypt_aes_256(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        let cipher = Aes256Gcm::new_from_slice(&self.key)
            .map_err(|e| format!("Invalid AES key: {}", e))?;

        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, data)
            .map_err(|e| format!("AES encryption failed: {}", e))?;

        // Gabungkan: nonce (12) + ciphertext
        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }

    /// Dekripsi data dengan AES-256-GCM
    /// Input: nonce (12 bytes) + ciphertext + tag (16 bytes)
    pub fn decrypt_aes_256(&self, encrypted: &[u8]) -> Result<Vec<u8>, String> {
        if encrypted.len() < 12 {
            return Err("Encrypted data too short".to_string());
        }

        let cipher = Aes256Gcm::new_from_slice(&self.key)
            .map_err(|e| format!("Invalid AES key: {}", e))?;

        let (nonce_bytes, ciphertext) = encrypted.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| format!("AES decryption failed: {}", e))
    }

    // ============================================================
    // RSA ENCRYPTION
    // ============================================================

    /// Enkripsi data dengan RSA public key + OAEP padding
    pub fn rsa_encrypt(public_key_pem: &str, data: &[u8]) -> Result<Vec<u8>, String> {
        let public_key = RsaPublicKey::from_public_key_pem(public_key_pem)
            .map_err(|e| format!("Invalid RSA public key: {}", e))?;

        let mut rng = OsRng;
        let padding = Oaep::new::<Sha256>();
        public_key
            .encrypt(&mut rng, padding, data)
            .map_err(|e| format!("RSA encryption failed: {}", e))
    }

    /// Dekripsi data dengan RSA private key + OAEP padding
    pub fn rsa_decrypt(private_key_pem: &str, encrypted: &[u8]) -> Result<Vec<u8>, String> {
        let private_key = RsaPrivateKey::from_pkcs8_pem(private_key_pem)
            .map_err(|e| format!("Invalid RSA private key: {}", e))?;

        let padding = Oaep::new::<Sha256>();
        private_key
            .decrypt(padding, encrypted)
            .map_err(|e| format!("RSA decryption failed: {}", e))
    }

    /// Generate RSA key pair
    pub fn generate_rsa_keypair() -> Result<(String, String), String> {
        let mut rng = OsRng;
        let private_key = RsaPrivateKey::new(&mut rng, 2048)
            .map_err(|e| format!("RSA key generation failed: {}", e))?;
        let public_key = RsaPublicKey::from(&private_key);

        let private_pem = private_key.to_pkcs8_pem().map_err(|e| format!("PEM export: {}", e))?.to_string();
        let public_pem = public_key.to_public_key_pem().map_err(|e| format!("PEM export: {}", e))?;

        Ok((private_pem, public_pem))
    }

    // ============================================================
    // KEY MANAGEMENT
    // ============================================================

    /// Derive key dari password menggunakan multiple hash iterations
    pub fn derive_key(password: &str, salt: &[u8], iterations: u32) -> Vec<u8> {
        use sha2::Digest;
        let mut key = password.as_bytes().to_vec();
        key.extend_from_slice(salt);

        for _ in 0..iterations {
            let mut hasher = Sha256::new();
            hasher.update(&key);
            key = hasher.finalize().to_vec();
        }
        key
    }

    /// Generate cryptographically secure random bytes
    pub fn generate_secure_random(length: usize) -> Vec<u8> {
        let mut bytes = vec![0u8; length];
        OsRng.fill_bytes(&mut bytes);
        bytes
    }

    /// Generate random hex string
    pub fn generate_random_hex(length: usize) -> String {
        let bytes = Encryption::generate_secure_random(length / 2 + 1);
        hex::encode(&bytes)[..length].to_string()
    }

    /// Load key dari file
    pub fn load_key_from_file(path: &std::path::Path) -> Result<Vec<u8>, String> {
        let content = fs::read(path)
            .map_err(|e| format!("Cannot read key file: {}", e))?;
        Ok(content)
    }

    /// Save key ke file
    pub fn save_key_to_file(path: &std::path::Path, key: &[u8]) -> Result<(), String> {
        fs::write(path, key)
            .map_err(|e| format!("Cannot write key file: {}", e))
    }

    /// Encrypt file
    pub fn encrypt_file(&self, input_path: &std::path::Path, output_path: &std::path::Path) -> Result<(), String> {
        let data = fs::read(input_path)
            .map_err(|e| format!("Cannot read input file: {}", e))?;
        let encrypted = self.encrypt_aes_256(&data)?;
        fs::write(output_path, encrypted)
            .map_err(|e| format!("Cannot write output file: {}", e))
    }

    /// Decrypt file
    pub fn decrypt_file(&self, input_path: &std::path::Path, output_path: &std::path::Path) -> Result<(), String> {
        let encrypted = fs::read(input_path)
            .map_err(|e| format!("Cannot read input file: {}", e))?;
        let decrypted = self.decrypt_aes_256(&encrypted)?;
        fs::write(output_path, decrypted)
            .map_err(|e| format!("Cannot write output file: {}", e))
    }
}

impl Default for Encryption {
    fn default() -> Self {
        let key = Encryption::generate_key(EncryptionAlgorithm::Aes256Gcm);
        Encryption::new(EncryptionAlgorithm::Aes256Gcm, &key)
    }
}

// ============================================================
// UNIT TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_key_aes() {
        let key = Encryption::generate_key(EncryptionAlgorithm::Aes256Gcm);
        assert_eq!(key.len(), 32);
    }

    #[test]
    fn test_aes_encrypt_decrypt() {
        let key = Encryption::generate_key(EncryptionAlgorithm::Aes256Gcm);
        let enc = Encryption::new(EncryptionAlgorithm::Aes256Gcm, &key);
        let plaintext = b"Hello, this is a secret message!";

        let encrypted = enc.encrypt_aes_256(plaintext).unwrap();
        assert!(encrypted.len() > plaintext.len()); // nonce + tag

        let decrypted = enc.decrypt_aes_256(&encrypted).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_aes_decrypt_wrong_key() {
        let key1 = Encryption::generate_key(EncryptionAlgorithm::Aes256Gcm);
        let key2 = Encryption::generate_key(EncryptionAlgorithm::Aes256Gcm);
        let enc = Encryption::new(EncryptionAlgorithm::Aes256Gcm, &key1);
        let dec = Encryption::new(EncryptionAlgorithm::Aes256Gcm, &key2);

        let encrypted = enc.encrypt_aes_256(b"secret").unwrap();
        let result = dec.decrypt_aes_256(&encrypted);
        assert!(result.is_err());
    }

    #[test]
    fn test_aes_decrypt_corrupted_data() {
        let enc = Encryption::default();
        let mut encrypted = enc.encrypt_aes_256(b"data").unwrap();
        // Corrupt middle byte
        if encrypted.len() > 20 {
            encrypted[15] ^= 0xFF;
        }
        let result = enc.decrypt_aes_256(&encrypted);
        assert!(result.is_err());
    }

    #[test]
    fn test_aes_encrypt_empty() {
        let enc = Encryption::default();
        let encrypted = enc.encrypt_aes_256(b"").unwrap();
        assert!(encrypted.len() >= 12);
        let decrypted = enc.decrypt_aes_256(&encrypted).unwrap();
        assert!(decrypted.is_empty());
    }

    #[test]
    fn test_aes_non_deterministic() {
        let enc = Encryption::default();
        let c1 = enc.encrypt_aes_256(b"same data").unwrap();
        let c2 = enc.encrypt_aes_256(b"same data").unwrap();
        // Different nonces = different ciphertexts
        assert_ne!(c1, c2);
    }

    #[test]
    fn test_generate_rsa_keypair() {
        let (private, public) = Encryption::generate_rsa_keypair().unwrap();
        assert!(private.starts_with("-----BEGIN PRIVATE KEY-----"));
        assert!(public.starts_with("-----BEGIN RSA PUBLIC KEY-----"));
    }

    #[test]
    fn test_rsa_encrypt_decrypt() {
        let (private, public) = Encryption::generate_rsa_keypair().unwrap();
        let plaintext = b"RSA encrypted message";

        let encrypted = Encryption::rsa_encrypt(&public, plaintext).unwrap();
        assert_ne!(encrypted, plaintext);

        let decrypted = Encryption::rsa_decrypt(&private, &encrypted).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_derive_key() {
        let key1 = Encryption::derive_key("password123", b"random_salt", 1000);
        let key2 = Encryption::derive_key("password123", b"random_salt", 1000);
        assert_eq!(key1, key2);
        assert_eq!(key1.len(), 32);
    }

    #[test]
    fn test_generate_secure_random() {
        let r1 = Encryption::generate_secure_random(16);
        let r2 = Encryption::generate_secure_random(16);
        assert_eq!(r1.len(), 16);
        assert_ne!(r1, r2);
    }

    #[test]
    fn test_generate_random_hex() {
        let hex = Encryption::generate_random_hex(32);
        assert_eq!(hex.len(), 32);
        assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_encrypt_decrypt_file() {
        let enc = Encryption::default();
        let tmp_in = std::env::temp_dir().join("iws_enc_test_in.txt");
        let tmp_enc = std::env::temp_dir().join("iws_enc_test_out.enc");
        let tmp_dec = std::env::temp_dir().join("iws_enc_test_dec.txt");

        fs::write(&tmp_in, b"file content for encryption").unwrap();
        enc.encrypt_file(&tmp_in, &tmp_enc).unwrap();
        enc.decrypt_file(&tmp_enc, &tmp_dec).unwrap();

        let decrypted = fs::read_to_string(&tmp_dec).unwrap();
        assert_eq!(decrypted, "file content for encryption");

        fs::remove_file(&tmp_in).ok();
        fs::remove_file(&tmp_enc).ok();
        fs::remove_file(&tmp_dec).ok();
    }

    #[test]
    fn test_save_load_key() {
        let key = Encryption::generate_key(EncryptionAlgorithm::Aes256Gcm);
        let tmp = std::env::temp_dir().join("iws_test_key.bin");
        Encryption::save_key_to_file(&tmp, &key).unwrap();
        let loaded = Encryption::load_key_from_file(&tmp).unwrap();
        assert_eq!(key, loaded);
        fs::remove_file(&tmp).ok();
    }

    #[test]
    fn test_encryption_algorithm_display() {
        assert_eq!(EncryptionAlgorithm::Aes256Gcm.to_string(), "AES-256-GCM");
        assert_eq!(EncryptionAlgorithm::RsaOaep.to_string(), "RSA-OAEP");
    }

    #[test]
    fn test_large_data_encryption() {
        let enc = Encryption::default();
        let large_data = vec![0xAAu8; 1024 * 100]; // 100KB
        let encrypted = enc.encrypt_aes_256(&large_data).unwrap();
        let decrypted = enc.decrypt_aes_256(&encrypted).unwrap();
        assert_eq!(decrypted, large_data);
    }
}
