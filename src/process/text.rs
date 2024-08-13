use crate::cli::TextSignFormat;
use crate::process_genpass;
use crate::utils::get_reader;
use anyhow::Result;
use base64::prelude::*;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
pub trait TextSign {
    // 动态分派reader
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}
pub trait TextVerify {
    // 静态分派reader
    fn verify(&self, reader: impl Read, sign: &[u8]) -> Result<bool>;
}
pub struct Blake3 {
    key: [u8; 32],
}
pub struct Ed25519Signer {
    key: SigningKey,
}
pub struct Ed25519Verifier {
    key: VerifyingKey,
}
pub trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized;
}
pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> Result<String> {
    let mut reader = get_reader(input)?;
    let signed = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            let signer = Ed25519Signer::load(key)?;
            signer.sign(&mut reader)?
        }
    };
    Ok(BASE64_STANDARD.encode(signed))
}
pub fn process_text_verify(
    input: &str,
    key: &str,
    sign: &str,
    format: TextSignFormat,
) -> Result<bool> {
    // 注意sign如果在reader之后，由于get_sign_content中也调用get_reader，会出错
    let sign = get_sign_content(sign)?;
    let mut reader = get_reader(input)?;
    // let sign=get_sign_content(sign)?;
    let verified = match format {
        TextSignFormat::Blake3 => {
            let verify = Blake3::load(key)?;
            verify.verify(&mut reader, &sign)?
        }
        TextSignFormat::Ed25519 => {
            let verify = Ed25519Verifier::load(key)?;
            verify.verify(&mut reader, &sign)?
        }
    };
    Ok(verified)
}
fn get_sign_content(sign: &str) -> Result<Vec<u8>> {
    let mut reader = get_reader(sign)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();
    let buf = BASE64_STANDARD.decode(buf.as_bytes())?;
    Ok(buf)
}
pub fn process_text_generate(format: TextSignFormat) -> Result<HashMap<&'static str, String>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}
fn get_key_decode(path: impl AsRef<Path>) -> Result<Vec<u8>> {
    let mut buf = String::new();
    let mut reader = File::open(path)?;
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();
    let key = BASE64_STANDARD.decode(buf.as_bytes())?;
    Ok(key)
}
impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = get_key_decode(path)?;
        Self::try_new(&key)
    }
}
impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = get_key_decode(path)?;
        Self::try_new(&key)
    }
}
impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = get_key_decode(path)?;
        Self::try_new(&key)
    }
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        // hash变量为16进制字符串，
        let hash = blake3::keyed_hash(&self.key, &buf);
        Ok(hash.as_bytes().to_vec())
    }
}
impl TextVerify for Blake3 {
    fn verify(&self, mut reader: impl Read, sign: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        let hash = hash.as_bytes();
        Ok(hash == sign)
    }
}
impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sign = self.key.sign(&buf);
        Ok(sign.to_bytes().to_vec())
    }
}
impl TextVerify for Ed25519Verifier {
    fn verify(&self, mut reader: impl Read, sign: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sign = Signature::from_bytes(sign.try_into()?);
        let ret = self.key.verify(&buf, &sign).is_ok();
        Ok(ret)
    }
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }
    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        let signer = Blake3::new(key);
        Ok(signer)
    }
    fn generate() -> Result<HashMap<&'static str, String>> {
        let key = process_genpass(true, true, true, true, 32)?;
        let key = key.as_bytes().to_vec();
        let mut mp = HashMap::new();
        mp.insert("blake3.txt", BASE64_STANDARD.encode(&key));
        Ok(mp)
    }
}
impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }
    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        let key = SigningKey::from_bytes(key);
        let signer = Self::new(key);
        Ok(signer)
    }
    fn generate() -> Result<HashMap<&'static str, String>> {
        let mut csprng = OsRng;
        let sk: SigningKey = SigningKey::generate(&mut csprng);
        let pk = sk.verifying_key().to_bytes().to_vec();
        let sk = sk.to_bytes().to_vec();
        let mut mp = HashMap::new();
        mp.insert("ed25519.sk", BASE64_STANDARD.encode(&sk));
        mp.insert("ed25519.pk", BASE64_STANDARD.encode(&pk));
        Ok(mp)
    }
}
impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }
    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        let key = VerifyingKey::from_bytes(key)?;
        let signer = Self::new(key);
        Ok(signer)
    }
}
