use leptos::prelude::*;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use std::sync::Arc;
use crate::shared::account_data::{BasicAccountData};
use surrealdb::Error;
use surrealdb_types::RecordId;

pub async fn create_account(basic_account_data:BasicAccountData) -> Result<Option<RecordId>, Error>{
    let db=use_context::<Arc<Surreal<Client>>>().unwrap();
    
    let record_id:Result<Option<RecordId>,Error>=db.create("Account")
    .content(basic_account_data).await;
    record_id
}