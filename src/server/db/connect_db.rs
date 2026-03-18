use surrealdb::Surreal;
use surrealdb::opt::auth::Root;
use surrealdb::engine::remote::ws::{Ws,Client};
use std::env::var;
use surrealdb::Error;

pub async fn connect_to_db() -> Result<Surreal<Client>, Error> {
    let db = Surreal::new::<Ws>("surrealdb:8000").await.unwrap();
    db.signin(Root {
        username: var("DB_USER").unwrap(),
        password: var("DB_PASS").unwrap(),
    }).await?;
    db.use_ns("main").use_db("main").await?;
    Ok(db)
}