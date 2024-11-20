use crate::cli::text::TextSignFormat;
use crate::{get_reader, process_genpass};
use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use std::fs;
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

pub struct Blake3 {
    key: [u8; 32],
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf: Vec<u8> = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, mut reader: impl Read, signature: &[u8]) -> Result<bool> {
        let mut buf: Vec<u8> = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        let hash = hash.as_bytes();
        Ok(hash == signature)
    }
}

impl TextGenerator for Blake3 {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let key = process_genpass(32, false, false, false, false)?;
        Ok(vec![key.into_bytes()])
    }
}

pub struct Ed25519Signer {
    key: SigningKey,
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf: Vec<u8> = Vec::new();
        reader.read_to_end(&mut buf)?;
        let signature = self.key.sign(&buf);
        Ok(signature.to_bytes().to_vec())
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, mut reader: impl Read, signature: &[u8]) -> Result<bool> {
        let mut buf: Vec<u8> = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = Signature::from_bytes(signature.try_into()?);
        Ok(self.key.verify(&buf, &sig).is_ok())
    }
}

impl TextGenerator for Ed25519Signer {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let sk = signing_key.to_bytes().to_vec();
        let pk = signing_key.verifying_key().to_bytes().to_vec();

        Ok(vec![sk, pk])
    }
}

pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> Result<String> {
    let mut reader = get_reader(input)?;

    let signature = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            let signer = Ed25519Signer::load(key)?;
            signer.sign(&mut reader)?
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
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load(key)?;
            verifier.verify(reader, &signature)?
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(key)?;
            verifier.verify(reader, &signature)?
        }
    };

    Ok(result)
}

pub fn process_text_generate(format: TextSignFormat) -> Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key: [u8; 32] = key.try_into()?;
        Ok(Self::new(key))
    }
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = SigningKey::from_bytes(&key[..32].try_into()?);
        Ok(Self::new(key))
    }
}

impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = VerifyingKey::from_bytes(&key[..32].try_into()?)?;
        Ok(Self::new(key))
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_blake3_sign_verify() -> Result<()> {
        let signer = Blake3::load("fixtures/blake3.key")?;
        let verifier = Blake3::load("fixtures/blake3.key")?;
        let data = b"hello world";
        let signature = signer.sign(&mut &data[..])?;
        assert!(verifier.verify(&data[..], &signature)?);
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
}
