use crate::cli::Base64Format;
use crate::utils::get_reader;
use anyhow::Result;
use base64::engine::general_purpose::URL_SAFE;
use base64::prelude::*;
use std::io::Read;
pub fn process_encode(input: &str, format: Base64Format) -> Result<String> {
    let mut reader: Box<dyn Read> = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let encoded = match format {
        Base64Format::UrlSafe => URL_SAFE.encode(buf),
        Base64Format::Standard => BASE64_STANDARD.encode(buf),
    };
    Ok(encoded)
}
pub fn process_decode(input: &str, format: Base64Format) -> Result<String> {
    let mut reader: Box<dyn Read> = get_reader(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();
    let decoded = match format {
        Base64Format::UrlSafe => URL_SAFE.decode(buf).expect("编码格式错误"),
        Base64Format::Standard => BASE64_STANDARD.decode(buf).expect("编码格式错误"),
    };
    Ok(String::from_utf8(decoded)?)
}
#[cfg(test)]
mod tests {
    use super::get_reader;
    use base64::prelude::*;
    #[test]
    fn test_process_decode() {
        // 相当于读取utf-8编码格式下被encode的Cargo.toml文件
        let mut reader = get_reader("decode.txt").unwrap();
        let mut buf = String::new();
        reader.read_to_string(&mut buf).unwrap();
        let buf = buf.trim();
        let decode = BASE64_STANDARD.decode(buf).expect("编码格式错误");
        println!("{}", String::from_utf8(decode).unwrap());
    }
}
