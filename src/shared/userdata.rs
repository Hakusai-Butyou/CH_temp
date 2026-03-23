use surrealdb_types::SurrealValue;

#[derive(SurrealValue)]
pub struct UserInitData{
    name:String,
    google_id:String,
    email:String,
}

#[derive(SurrealValue)]
pub struct UserFullData{
    name:String,
    google_id:String,
    email:String,
}
