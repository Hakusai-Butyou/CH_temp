use leptos::prelude::*;
use leptos_meta::{Title};
use leptos_router::components::A;
/// Renders the home page of your application.
#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <Title text="404 Not found."/>
        <h1>"Page not found."</h1>
        <A href="/" >
        <button>"Back to home page"</button>
        </A>
    }
}
