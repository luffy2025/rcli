use crate::process::text::{KeyLoader, TextGenerator, TextSign, TextVerify};
use anyhow::Result;
use chacha20poly1305::aead::{Aead, OsRng};
use chacha20poly1305::{aead::KeyInit, AeadCore, ChaCha20Poly1305, Nonce};
use std::fs;
use std::io::Read;
use std::path::Path;

const NONCE_SIZE: usize = 12;

pub struct ChaCha20Poly1305Encryptor {
    cipher: ChaCha20Poly1305,
}

impl ChaCha20Poly1305Encryptor {
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let cipher = ChaCha20Poly1305::new(key.as_ref().into());
        Ok(Self { cipher })
    }
}

impl TextGenerator for ChaCha20Poly1305Encryptor {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let key = ChaCha20Poly1305::generate_key(&mut OsRng);
        Ok(vec![key.to_vec()])
    }
}

impl KeyLoader for ChaCha20Poly1305Encryptor {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = &fs::read(path)?[..32];
        Self::try_new(key)
    }
}

impl TextSign for ChaCha20Poly1305Encryptor {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf: Vec<u8> = Vec::new();
        reader.read_to_end(&mut buf)?;
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let mut ciphertext = self
            .cipher
            .encrypt(&nonce, buf.as_ref())
            .map_err(|_| anyhow::anyhow!("Encryption failed"))?;
        ciphertext.extend_from_slice(nonce.as_slice());
        Ok(ciphertext)
    }
}

impl TextVerify for ChaCha20Poly1305Encryptor {
    fn verify(&self, mut reader: impl Read, signature: &[u8]) -> Result<bool> {
        let mut buf: Vec<u8> = Vec::new();
        reader.read_to_end(&mut buf)?;
        let (ciphertext, nonce) = signature.split_at(signature.len() - NONCE_SIZE);
        let nonce = Nonce::clone_from_slice(nonce);
        let plaintext = self
            .cipher
            .decrypt(&nonce, ciphertext)
            .map_err(|_| anyhow::anyhow!("Decryption failed"))?;
        Ok(plaintext == buf)
    }
}
