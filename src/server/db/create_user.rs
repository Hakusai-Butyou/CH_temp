use leptos::prelude::*;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use std::sync::Arc;
use crate::shared::userdata::{UserInitData};
use surrealdb::Error;
use surrealdb_types::RecordId;

pub async fn create_user(user_init_data:UserInitData) -> Result<Option<RecordId>, Error>{
    let db=use_context::<Arc<Surreal<Client>>>().unwrap();
    
    let record_id:Result<Option<RecordId>,Error>=db.create("Account")
    .content(user_init_data).await;
    record_id
}