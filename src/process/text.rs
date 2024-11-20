use crate::cli::text::TextSignFormat;
use crate::process::crypto::chacha20poly1305::ChaCha20Poly1305Encryptor;
use crate::{get_reader, Blake3, Ed25519Signer, Ed25519Verifier};
use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use std::io::Read;
use std::path::Path;

pub trait TextSign {
    /// Sign the data from the reader and return the signature
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerify {
    /// Verify a message
    fn verify(&self, reader: impl Read, signature: &[u8]) -> Result<bool>;
}

pub trait TextGenerator {
    /// Generate a key pair
    fn generate() -> Result<Vec<Vec<u8>>>;
}

pub trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized;
}

pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> Result<String> {
    let mut reader = get_reader(input)?;

    let signature = match format {
        TextSignFormat::Blake3 => Blake3::load(key)?.sign(&mut reader)?,
        TextSignFormat::Ed25519 => Ed25519Signer::load(key)?.sign(&mut reader)?,
        TextSignFormat::ChaCha20Poly1305 => {
            ChaCha20Poly1305Encryptor::load(key)?.sign(&mut reader)?
        }
    };

    Ok(URL_SAFE_NO_PAD.encode(&signature))
}

pub fn process_text_verify(
    input: &str,
    key: &str,
    signature: &str,
    format: TextSignFormat,
) -> Result<bool> {
    let reader = get_reader(input)?;

    let signature = URL_SAFE_NO_PAD.decode(signature)?;

    let result = match format {
        TextSignFormat::Blake3 => Blake3::load(key)?.verify(reader, &signature)?,
        TextSignFormat::Ed25519 => Ed25519Verifier::load(key)?.verify(reader, &signature)?,
        TextSignFormat::ChaCha20Poly1305 => {
            ChaCha20Poly1305Encryptor::load(key)?.verify(reader, &signature)?
        }
    };

    Ok(result)
}

pub fn process_text_generate(format: TextSignFormat) -> Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
        TextSignFormat::ChaCha20Poly1305 => ChaCha20Poly1305Encryptor::generate(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::process::Blake3;
    use crate::process::{Ed25519Signer, Ed25519Verifier};
    use anyhow::Result;

    #[test]
    fn test_blake3_sign_verify() -> Result<()> {
        let encryptor = Blake3::load("fixtures/blake3.key")?;
        let data = b"hello world";
        let signature = encryptor.sign(&mut &data[..])?;
        assert!(encryptor.verify(&data[..], &signature)?);
        Ok(())
    }

    #[test]
    fn test_ed25519_sign_verify() -> Result<()> {
        let signer = Ed25519Signer::load("fixtures/ed25519.sk")?;
        let verifier = Ed25519Verifier::load("fixtures/ed25519.pk")?;
        let data = b"hello world";
        let signature = signer.sign(&mut &data[..])?;
        assert!(verifier.verify(&data[..], &signature)?);
        Ok(())
    }

    #[test]
    fn test_chacha20poly1305_sign_verify() -> Result<()> {
        let encryptor = ChaCha20Poly1305Encryptor::load("fixtures/chacha20poly1305.key")?;
        let data = b"hello world";
        let signature = encryptor.sign(&mut &data[..])?;
        assert!(encryptor.verify(&data[..], &signature)?);
        Ok(())
    }
}
