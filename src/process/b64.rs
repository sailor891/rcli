use crate::cli::Base64Format;
use base64::engine::general_purpose::URL_SAFE;
use base64::prelude::*;
use std::io::Read;
use std::{fs::File, io::stdin};

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(stdin())
    } else {
        Box::new(File::open(input)?)
    };
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let encoded = match format {
        Base64Format::UrlSafe => URL_SAFE.encode(&buf),
        Base64Format::Standard => BASE64_STANDARD.encode(&buf),
    };
    Ok(encoded)
}
pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(stdin())
    } else {
        // 读取的文件编码类型务必为utf8格式
        // 否则会报错
        Box::new(File::open(input)?)
    };
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let decoded = match format {
        Base64Format::UrlSafe => URL_SAFE.decode(&buf)?,
        Base64Format::Standard => BASE64_STANDARD.decode(&buf)?,
    };
    Ok(String::from_utf8(decoded)?)
}
#[cfg(test)]
mod tests {
    use base64::prelude::*;
    #[test]
    fn test_process_decode() {
        // 相当于读取utf-8编码格式下的encode之后的Cargo.toml文件
        let input = b"W3BhY2thZ2VdCm5hbWUgPSAicmNsaSIKdmVyc2lvbiA9ICIwLjEuMCIKZWRpdGlvbiA9ICIyMDIxIgphdXRob3JzID0gWyJ4b/CfpoAiXQojIFNlZSBtb3JlIGtleXMgYW5kIHRoZWlyIGRlZmluaXRpb25zIGF0IGh0dHBzOi8vZG9jLnJ1c3QtbGFuZy5vcmcvY2FyZ28vcmVmZXJlbmNlL21hbmlmZXN0Lmh0bWwKCltkZXBlbmRlbmNpZXNdCmFueWhvdyA9ICIxLjAuODYiCmJhc2U2NCA9ICIwLjIyLjEiCmNsYXAgPSB7IHZlcnNpb24gPSAiNC41LjEzIiwgZmVhdHVyZXMgPSBbImRlcml2ZSJdIH0KY3N2ID0gIjEuMy4wIgpyYW5kID0gIjAuOC41IgpzZXJkZSA9IHsgdmVyc2lvbiA9ICIxLjAuMjA0IiwgZmVhdHVyZXMgPSBbImRlcml2ZSJdIH0Kc2VyZGVfanNvbiA9ICIxLjAuMTIyIgpzZXJkZV95YW1sID0gIjAuOS4zNCIKdG9tbCA9ICIwLjguMTkiCnp4Y3ZibiA9ICIzLjEuMCIK";
        let output = BASE64_STANDARD.decode(input).unwrap();
        println!("{}", String::from_utf8(output).unwrap());
    }
}
