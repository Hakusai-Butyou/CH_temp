use leptos::prelude::*;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use std::sync::Arc;
use crate::server::db::account::db_account_data::DBAccountData;
use surrealdb::Error;
use surrealdb_types::{RecordIdKey,ToSql};

pub async fn get_account_data_by_record_id_key(record_id_key:RecordIdKey) -> Result<Option<DBAccountData>, Error>{
    let db=use_context::<Arc<Surreal<Client>>>().unwrap();
    let get_result:Option<DBAccountData>=db.select(("Account",record_id_key.to_sql())).await?;
    Ok(get_result)
}
pub async fn get_account_data_by_google_id(google_id:String) -> Result<Option<DBAccountData>, Error>{
    let db=use_context::<Arc<Surreal<Client>>>().unwrap();
    let mut get_result=db.query("SELECT * FROM Account WHERE google_id = $google_id")
                                               .bind(("google_id", google_id)).await?;
    let user_data:Option<DBAccountData>=get_result.take(0)?;
    Ok(user_data)
}