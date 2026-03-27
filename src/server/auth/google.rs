use leptos::prelude::*;
use leptos::server_fn::{ServerFnError,codec};
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use surrealdb_types::RecordId;
use crate::shared::account_data::BasicAccountData;
use serde::Deserialize;
use std::time::{UNIX_EPOCH,SystemTime};
use reqwest::Client;
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
#[cfg(feature = "ssr")]
use tokio::sync::RwLock;
#[cfg(feature = "ssr")]
use once_cell::sync::Lazy;
#[cfg(feature = "ssr")]
use std::sync::Arc;
#[cfg(feature = "ssr")]
pub static JWKS_CACHE: Lazy<Arc<RwLock<JwksCache>>> =
    Lazy::new(|| Arc::new(RwLock::new(JwksCache::empty())));

//jwks:Json Web Key Set
pub async fn fetch_jwks() -> Result<(Jwks,Vec<String>),ServerFnError> {
    let url = "https://www.googleapis.com/oauth2/v3/certs";
    
    let jwks = Client::new()
        .get(url)
        .send()
        .await?
        .json::<Jwks>()
        .await?;
    let kids=jwks.keys
                      .iter()
                      .map(move |x| x.kid.clone())
                      .collect();
    Ok((jwks,kids))
}

pub fn verify_id_token(header:&Header,payload:&Payload) -> Result<(),ServerFnError> {
    if header.alg != "RS256" {
        return Err(ServerFnError::new("'alg' is invalid."));
    }
    if payload.aud != env!("CLIENT_ID") {
        return Err(ServerFnError::new("'client_id' is invalid."));
    }
    if !(payload.iss == "accounts.google.com" || payload.iss == "https://accounts.google.com") {
        return Err(ServerFnError::new("'iss' is invalid."));
    }
    let current_time=SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;
    if current_time > payload.exp {
        return Err(ServerFnError::new("This token is expired."));
    }
    if payload.email_verified == false {
        return Err(ServerFnError::new("This email address is not verified."));
    }
    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn verify_signature(kid:String,devided_raw_jwt:&Vec<&str>) -> Result<(),ServerFnError> {
    use ring::signature;

    let jwks_cache = JWKS_CACHE.clone();
    let mut jwk_cache_lock=jwks_cache.write().await;
    let jwk=jwk_cache_lock.get_key(kid).await?;
    let signed_data = format!("{}.{}", devided_raw_jwt[0], devided_raw_jwt[1]);
    let signature_bytes =        URL_SAFE_NO_PAD.decode(devided_raw_jwt[2]).map_err(|_| "invalid signature b64").unwrap();
    let n = URL_SAFE_NO_PAD.decode(jwk.n.clone()).map_err(|_| "invalid n").unwrap();
    let e = URL_SAFE_NO_PAD.decode(jwk.e.clone()).map_err(|_| "invalid e").unwrap();
    let public_key_der = signature::RsaPublicKeyComponents {
        n: &n,
        e: &e,
    };
    let verification_result=public_key_der
        .verify(
            &signature::RSA_PKCS1_2048_8192_SHA256,
            signed_data.as_bytes(),
            &signature_bytes,
        )
        .map_err(|_| "signature verification failed");
    if verification_result.is_ok(){
        Ok(())
    } else {
        Err(ServerFnError::new("signature varification failed."))
    }
}

/// Google の ID Token の payload を JSON に変換する
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
pub fn create_user_init_data(payload:Payload) -> BasicAccountData {
    BasicAccountData{
        name:payload.name,
        google_id:payload.sub,
        email:payload.email
    }
}

#[server(name=GoogleAuth, prefix="/auth", endpoint="google", input=codec::Json)]
pub async fn google(credential:String) -> Result<String,ServerFnError> {
    use crate::server::session::register_session::register_session;
    use crate::server::db::account::create_account::create_account;
    use crate::server::db::account::get_account_data::get_account_data_by_google_id;

    let (header,payload,devided_raw_jwt)=decode_google_id_token(&credential);
    let verify_id_token_result=verify_id_token(&header, &payload);
    if verify_id_token_result.is_err() {
        return Err(verify_id_token_result.err().unwrap());
    }
    let verify_signature_result=verify_signature(header.kid.clone(), &devided_raw_jwt).await;
    if verify_signature_result.is_err() {
        return Err(verify_signature_result.err().unwrap());
    }

    let record_id:RecordId;
    if let Some(user_data)=get_account_data_by_google_id(payload.sub.clone()).await?{
        record_id=user_data.id.clone();
    } else{
        let res=create_account(create_user_init_data(payload)).await;
        if res.is_err(){
            return Err(ServerFnError::new("cannot make user."))
        }
        if let Some(rec_id)=res.unwrap(){
            record_id=rec_id
        } else {
            return Err(ServerFnError::new("Responce of register_session is None."))
        }

    }

    let reg_session_res=register_session(record_id).await;
    if reg_session_res.is_ok() {
        let session_id=reg_session_res.unwrap();
        Ok(session_id)
    } else {
        reg_session_res
    }
}