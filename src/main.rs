
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::{provide_context, *};
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use creater_hub_temp::app::root::App;
    use creater_hub_temp::app::shell::shell;
    use creater_hub_temp::server::db::connect_db::connect_to_db;
    use creater_hub_temp::server::db::init_db::init_DB;
    use creater_hub_temp::app::app_state::{self, AppState};
    use std::sync::Arc;

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let db=connect_to_db().await.unwrap();
    let arc_db=Arc::new(db);
    init_DB(arc_db.clone()).await;
    let app_state=AppState{leptos_options,db:arc_db};

    let app = Router::new()
        .leptos_routes_with_context(&app_state, routes, 
            {
                let db_clone=app_state.db.clone();
                move || {
                    provide_context(db_clone.clone());
                }
            }
            ,{
            let leptos_options = app_state.leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler::<AppState,_>(shell))
        .with_state(app_state);
    
    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
