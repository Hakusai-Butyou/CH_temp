use leptos::prelude::*;
use leptos_meta::Script;
use leptos_router::components::A;

#[component]
pub fn LoginPage() -> impl IntoView {
    //let app_meta_data=use_context::<Arc<AppMetaData>>().unwrap();
    let client_id= env!("CLIENT_ID");
    view! {
        <Script src="/script/google_auth.js" />
        <Script src="https://accounts.google.com/gsi/client" />
        <div
        id="g_id_onload"
        data-auto_prompt="false"
        data-callback="googleCallback"
        data-client_id={client_id}
        ></div>
        <div class="g_id_signin"></div>
        <A href="/">"戻る"</A> //仮
    }
}