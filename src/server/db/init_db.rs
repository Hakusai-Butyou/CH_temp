use leptos::prelude::*;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use std::sync::Arc;

pub async fn InitDB(db:Arc<Surreal<Client>>){
    db.query(include_str!("../../../migrations/init.surql")).await.unwrap();
}