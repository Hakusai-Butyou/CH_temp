use leptos::server_fn::ServerFnError;
use surrealdb_types::RecordId;

#[cfg(feature = "ssr")]
pub async fn register_session(user_record_id:RecordId)->Result<String,ServerFnError>{
    use surrealdb_types::ToSql;
    use leptos::prelude::use_context;
    use std::sync::Arc;
    use crate::server::session::generate_session_id::generate_session_id;
    use redis::{Client, AsyncTypedCommands };

    if let Some(client) = use_context::<Arc<Client>>(){
        let session_id=generate_session_id();
        let record_id=user_record_id.to_sql();
        let mut conection=client.get_multiplexed_async_connection().await?;
        let _=conection.set(session_id.clone(), record_id).await;
        Ok(session_id)
    } else {
        Err(ServerFnError::new("Redis not found."))
    }
}
#[cfg(not(feature = "ssr"))]
pub async fn register_session(_:RecordId)->Result<String,ServerFnError>{
    Err(ServerFnError::new("csr mode."))
}