use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct UserInitData{
    name:String,
    google_id:String,
    email:String,
}

#[derive(Serialize,Deserialize)]
pub struct UserFullData{
    name:String,
    google_id:String,
    email:String,
}
