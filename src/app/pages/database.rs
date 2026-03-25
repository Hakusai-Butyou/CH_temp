use std::str::FromStr;
use leptos::prelude::*;
use leptos_router::components::A;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use std::sync::Arc;
#[cfg(feature = "ssr")]
use redis;
/// Renders the home page of your application.
#[component]
pub fn DBPage() -> impl IntoView {
    // Creates a reactive value to update the button
    let text_db;
    match use_context::<Arc<Surreal<Client>>>() {
        Some(_v) => text_db=String::from_str("接続済み").unwrap(),
        None => text_db=String::from_str("未接続").unwrap(),
    }
    let mut text_redis;
    #[cfg(feature = "ssr")]
    {
        match use_context::<Arc<redis::Client>>() {
            Some(_v) => text_redis=String::from_str("接続済み").unwrap(),
            None => text_redis=String::from_str("未接続").unwrap(),
        }
    }
    #[cfg(not(feature = "ssr"))]
    {
        text_redis="csrモードです".to_string();
    }
    view! {
        <h1>"データベース情報"</h1>
        <h3>{text_db}</h3>
        <h1>"セッションストア情報"</h1>
        <h3>{text_redis}</h3>
        <A href="/">"戻る"</A> //仮
    }
}
