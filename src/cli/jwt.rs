use crate::*;
use anyhow::Result;
use clap::Parser;
use core::fmt;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use time::Duration;
use time::OffsetDateTime;

const JWT_SECRET: &str = "secret";
#[derive(Debug, Parser)]
#[enum_dispatch(CmdExcutor)]
pub enum JwtSubcommand {
    #[command(about = "Sign a JWT")]
    Sign(JwtSignOpts),
    #[command(about = "Verify a JWT")]
    Verify(JwtVerifyopts),
}
#[derive(Debug, Parser)]
pub struct JwtSignOpts {
    #[arg(short, long)]
    pub sub: String,
    #[arg(long)]
    pub aud: String,
    #[arg(short, long)]
    pub exp: String,
    #[arg(long)]
    pub alg: Algorithm,
}
#[derive(Debug, Parser)]
pub struct JwtVerifyopts {
    #[arg(short, long)]
    pub sub: String,
    #[arg(long)]
    pub aud: String,
    #[arg(long)]
    pub alg: Algorithm,
    #[arg(short, long,value_parser=verify_input_file,default_value="-")]
    pub path: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    sub: String,
    aud: String,
    exp: usize,
    alg: Algorithm,
}
impl JwtClaims {
    fn new(sub: String, aud: String, exp: usize, alg: Algorithm) -> Self {
        JwtClaims { sub, aud, exp, alg }
    }
    fn try_new(sub: String, aud: String, exp: usize, alg: Algorithm) -> Result<Self> {
        Ok(JwtClaims::new(sub, aud, exp, alg))
    }
    fn load(jwt: &JwtSignOpts) -> Result<Self> {
        let unit = &jwt.exp[jwt.exp.len() - 1..];
        let duration = &jwt.exp[..jwt.exp.len() - 1];
        let duration = match unit {
            "m" | "M" => Duration::minutes(duration.parse::<i64>()?),
            "h" | "H" => Duration::hours(duration.parse::<i64>()?),
            "d" | "D" => Duration::days(duration.parse::<i64>()?),
            _ => {
                return Err(anyhow::anyhow!(
                    "invalid unit only support m(minute),h(hour),d(day)"
                ))
            }
        };
        let exp = OffsetDateTime::now_utc() + duration;
        let exp = exp.unix_timestamp() as usize;
        Self::try_new(jwt.sub.clone(), jwt.aud.clone(), exp, jwt.alg)
    }
}
impl fmt::Display for JwtClaims {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let exp = self.exp;
        let date = OffsetDateTime::from_unix_timestamp(exp as i64).unwrap();
        write!(
            f,
            "sub: {}; aud: {}; exp: {}; alg: {:?}",
            self.sub, self.aud, date, self.alg
        )
    }
}
impl CmdExcutor for JwtSignOpts {
    async fn excutor(self) -> Result<()> {
        let claims = JwtClaims::load(&self)?;
        let header = Header::new(self.alg);
        let key = EncodingKey::from_secret(JWT_SECRET.as_ref());
        let token = jwt_encode(claims, header, key)?;
        println!("{}", token);
        Ok(())
    }
}
impl CmdExcutor for JwtVerifyopts {
    async fn excutor(self) -> Result<()> {
        let mut reader = get_reader(&self.path)?;
        let token = get_reader_content(&mut reader)?;
        let token = String::from_utf8(token)?;
        let key = DecodingKey::from_secret(JWT_SECRET.as_ref());
        let text = jwt_decode(token, key, self.sub, self.aud, self.alg)?;
        println!("{:?}", text);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Serialize, Deserialize)]
    struct JwtClaims {
        sub: String,
        company: String,
        exp: usize,
    }
    #[test]
    fn test_jwt() -> anyhow::Result<()> {
        let my_claims = JwtClaims {
            sub: "ttt".to_string(),
            company: "rrr".to_string(),
            exp: 20000000000,
        };
        let token = encode(
            &Header::default(),
            &my_claims,
            &EncodingKey::from_secret("secret".as_ref()),
        )?;
        println!("{}", token);

        println!("-----decode-----");

        let token = decode::<JwtClaims>(
            &token,
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::default(),
        )?;
        println!("{:?}", token);

        Ok(())
    }
}
