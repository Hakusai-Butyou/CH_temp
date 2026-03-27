use leptos::prelude::*;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use std::sync::Arc;
use crate::shared::account_data::{PublicAccountData};
use surrealdb::Error;
use surrealdb_types::{RecordIdKey,ToSql};

pub async fn get_account_data_by_record_id_key(record_id_key:RecordIdKey) -> Result<Option<PublicAccountData>, Error>{
    let db=use_context::<Arc<Surreal<Client>>>().unwrap();
    let get_result:Option<PublicAccountData>=db.select(("Account",record_id_key.to_sql())).await?;
    Ok(get_result)
}