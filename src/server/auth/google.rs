use leptos::prelude::*;
use leptos::server_fn::{ServerFnError,codec};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Header{
    pub alg:String,
    pub kid:String,
    pub typ:String,
}
#[derive(Deserialize, Debug)]
pub struct Payload{
    pub iss:String,
    pub azp:String,
    pub aud:String,
    pub sub:String,
    pub email:String,
    pub email_verified:bool,
    pub nbf:i64,
    pub name:String,
    pub picture:String,
    pub given_name:String,
    pub iat:i64,
    pub exp:i64,
    pub jti:String
}

/// Google の ID Token の payload を JSON に変換する（署名検証なし）
pub fn decode_google_id_token(token: &str) -> (Header,Payload) {
    let parts: Vec<&str> = token.split('.').collect();
    let header_b64 = parts[0];
    let payload_b64 = parts[1];

    let decoded_header = URL_SAFE_NO_PAD
        .decode(header_b64)
        .expect("Base64 decode failed");
    let decoded_payload = URL_SAFE_NO_PAD
        .decode(payload_b64)
        .expect("Base64 decode failed");
    let header:Header=serde_json::from_slice(&decoded_header).unwrap();
    let payload:Payload=serde_json::from_slice(&decoded_payload).unwrap();
    (header,payload)
}
#[derive(serde::Deserialize,serde::Serialize,Debug,Clone)]
pub struct GoogleLoginInput {
    pub credential: String,
}

#[server(name=GoogleAuth, prefix="/auth", endpoint="google", input=codec::Json)]
pub async fn google(credential:String) -> Result<(),ServerFnError> {
    println!("{}",credential);
    let (decoded_header,decoded_payload)=decode_google_id_token(&credential);
    println!("header : {}", String::from_utf8(decoded_header).unwrap());
    println!("payload : {}", String::from_utf8(decoded_payload).unwrap());
    Ok(())
}