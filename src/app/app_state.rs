use std::env::var;

#[cfg(feature = "ssr")]
use surrealdb::engine::remote::ws::Client;
#[cfg(feature = "ssr")]
use leptos::config::LeptosOptions;
#[cfg(feature = "ssr")]
use surrealdb::Surreal;
#[cfg(feature = "ssr")]
use redis;
#[cfg(feature = "ssr")]
use std::sync::Arc;
#[derive(Clone)]
#[cfg(feature = "ssr")]
pub struct AppState{
    pub leptos_options:LeptosOptions,
    pub db:Arc<Surreal<Client>>,
    pub redis:Arc<redis::Client>,
    pub app_meta_data:Arc<AppMetaData>,
}

#[cfg(feature = "ssr")]
use axum::extract::FromRef;
#[cfg(feature = "ssr")]
impl FromRef<AppState> for LeptosOptions {
    fn from_ref(state: &AppState) -> Self {
        state.leptos_options.clone()
    }
}
pub struct AppMetaData{
    pub client_id:String,
}
pub fn get_app_meta_data()->AppMetaData{
    let client_id= var("CLIENT_ID").unwrap();
    AppMetaData { client_id }
}