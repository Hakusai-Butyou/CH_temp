use surrealdb_types::SurrealValue;

#[derive(SurrealValue,Debug)]
pub struct BasicAccountData{
    pub name:String,
    pub google_id:String,
    pub email:String,
}

#[derive(SurrealValue,Debug)]
pub struct FullAccountData{
    pub name:String,
    pub google_id:String,
    pub email:String,
}

#[derive(SurrealValue,Debug)]
pub struct PublicAccountData{
    pub name:String,
    pub google_id:String,
    pub email:String,
}
