use crate::JwtClaims;
use anyhow::Result;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

pub fn jwt_encode(claims: JwtClaims, header: Header, key: EncodingKey) -> Result<String> {
    let token = encode(&header, &claims, &key)?;
    Ok(token)
}
pub fn jwt_decode(
    token: String,
    key: DecodingKey,
    sub: String,
    aud: String,
    alg: Algorithm,
) -> Result<String> {
    let mut validation = Validation::new(alg);
    validation.set_audience(&[aud]);
    validation.sub = Some(sub);
    validation.set_required_spec_claims(&["alg", "aud"]);
    let text = decode::<JwtClaims>(&token, &key, &validation)?;
    let payload = text.claims;
    Ok(format!("{}", payload))
}
