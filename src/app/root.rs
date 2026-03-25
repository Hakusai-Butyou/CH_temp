use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use crate::app::pages::home::HomePage;
use crate::app::pages::database::DBPage;
use crate::app::pages::login::LoginPage;
use crate::app::pages::not_found_page::NotFoundPage;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/creater-hub-temp.css"/>

        // sets the document title
        // <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| view! { <NotFoundPage/> }>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("database") view=DBPage/>
                    <Route path=StaticSegment("login") view=LoginPage/>
                </Routes>
            </main>
        </Router>
    }
}
