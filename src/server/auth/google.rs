use leptos::prelude::*;
use leptos::server_fn::{ServerFnError,codec};
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use crate::server::db::create_user::create_user;
use crate::shared::userdata::UserInitData;
use serde::Deserialize;
use crate::server::session::register_session::register_session;
use serde;
use once_cell::sync::Lazy;
use reqwest::Client;
use std::sync::Arc;
use ring::signature;
use std::time::{UNIX_EPOCH,SystemTime};

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

#[derive(Deserialize, Debug)]
pub struct Jwk{
    pub e:String,
    pub alg:String,
    pub n:String,
    pub kty:String,
    pub r#use:String,
    pub kid:String,
}
#[derive(Deserialize, Debug)]
pub struct Jwks{
    pub keys:Vec<Jwk>
}

pub struct JwksCache{
    pub jwks:Jwks,
    pub kids:Vec<String>,
} 
impl JwksCache {
    pub fn empty()->JwksCache{
        let empty_jwks=Jwks{keys:Vec::<Jwk>::new()};
        JwksCache{
            jwks:empty_jwks,
            kids:vec![],
        }
    }

    pub async fn get_key(&mut self,kid:String) -> Result<&Jwk,ServerFnError> {
        if let Some(key_num)
               =self.kids.iter().position(|x| **x==kid){
            let jwk=&self.jwks.keys[key_num];
            Ok(jwk)
        } else {
            let (new_jwks,new_kids)=fetch_jwks().await?;
            self.jwks=new_jwks;
            self.kids=new_kids;
            if let Some(key_num)
               =self.kids.iter().position(|x| **x==kid){
                let jwk=&self.jwks.keys[key_num];
                Ok(jwk)
            } else {
                Err(ServerFnError::new("the jwk not found"))
            }
        }
    }
}
/// Google の ID Token の payload を JSON に変換する（署名検証なし）
pub fn decode_google_id_token(token: &str) -> (Header,Payload,Vec<&str>) {
    let devided_raw_jwt: Vec<&str> = token.split('.').collect();
    let header_b64 = devided_raw_jwt[0];
    let payload_b64 = devided_raw_jwt[1];

    let decoded_header = URL_SAFE_NO_PAD
        .decode(header_b64)
        .expect("Base64 decode failed");
    let decoded_payload = URL_SAFE_NO_PAD
        .decode(payload_b64)
        .expect("Base64 decode failed");
    let header:Header=serde_json::from_slice(&decoded_header).unwrap();
    let payload:Payload=serde_json::from_slice(&decoded_payload).unwrap();
    (header,payload,devided_raw_jwt)
}
pub fn create_user_init_data(payload:Payload) -> UserInitData {
    UserInitData{
        name:payload.name,
        google_id:payload.sub,
        email:payload.email
    }
}

#[server(name=GoogleAuth, prefix="/auth", endpoint="google", input=codec::Json)]
pub async fn google(credential:String) -> Result<String,ServerFnError> {
    let (header,payload,devided_raw_jwt)=decode_google_id_token(&credential);
    let res=create_user(create_user_init_data(payload)).await;
    if res.is_ok(){
        if let Some(record_id)=res.unwrap(){
            let reg_session_res=register_session(record_id).await;
            if reg_session_res.is_ok() {
                let session_id=reg_session_res.unwrap();
                Ok(session_id)
            } else {
                reg_session_res
            }
        } else {
            Err(ServerFnError::new("Responce of register_session is None."))
        }
    } else {
        Err(ServerFnError::new("cannot make user."))
    }
}