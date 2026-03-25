use surrealdb_types::SurrealValue;

#[derive(SurrealValue,Debug)]
pub struct UserInitData{
    pub name:String,
    pub google_id:String,
    pub email:String,
}

#[derive(SurrealValue,Debug)]
pub struct UserFullData{
    pub name:String,
    pub google_id:String,
    pub email:String,
}
