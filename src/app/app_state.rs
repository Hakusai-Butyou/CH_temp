use leptos::config::LeptosOptions;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
#[derive(Clone)]
pub struct AppState{
    pub leptos_options:LeptosOptions,
    pub db:Surreal<Client>,
}

#[cfg(feature = "ssr")]
use axum::extract::FromRef;
#[cfg(feature = "ssr")]
impl FromRef<AppState> for LeptosOptions {
    fn from_ref(state: &AppState) -> Self {
        state.leptos_options.clone()
    }
}