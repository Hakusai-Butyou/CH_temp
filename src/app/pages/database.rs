use std::str::FromStr;
use leptos::prelude::*;
use leptos_router::components::A;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use std::sync::Arc;
/// Renders the home page of your application.
#[component]
pub fn DBPage() -> impl IntoView {
    // Creates a reactive value to update the button
    let mut text=String::from_str("");
    match use_context::<Arc<Surreal<Client>>>() {
        Some(_v) => text=String::from_str("接続済み"),
        None => text=String::from_str("未接続"),
    }
    view! {
        <h1>"データベース情報"</h1>
        <h1>{text}</h1>
        <A href="/">"戻る"</A> //仮
    }
}
