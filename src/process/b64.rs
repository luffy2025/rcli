use crate::cli::b64::Base64Format;
use anyhow::Result;
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::engine::Engine as _;
use std::fs::File;
use std::io::Read;

pub fn process_encode(input: &str, format: Base64Format) -> Result<()> {
    let buf = get_buf(input)?;

    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(buf),
    };
    println!("{}", encoded);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<()> {
    let buf = get_buf(input)?;

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };

    let decoded = String::from_utf8(decoded)?;
    println!("{}", decoded);
    Ok(())
}

fn get_buf(input: &str) -> Result<String> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();
    Ok(buf.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_buf() {
        let input = "fixtures/b64.out";
        assert!(get_buf(input).is_ok());
    }

    #[test]
    fn test_process_standard() {
        let input = "Cargo.toml";
        assert!(process_encode(input, Base64Format::Standard).is_ok());
        let input = "fixtures/b64.out";
        assert!(process_decode(input, Base64Format::Standard).is_ok());
    }

    #[test]
    fn test_process_urlsafe() {
        let input = "Cargo.toml";
        assert!(process_encode(input, Base64Format::UrlSafe).is_ok());
        let input = "fixtures/b64.out";
        assert!(process_decode(input, Base64Format::UrlSafe).is_ok());
    }
}
