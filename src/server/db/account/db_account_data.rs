use surrealdb_types::{SurrealValue,RecordId};
use crate::shared::account_data::{FullAccountData,BasicAccountData,PublicAccountData};

#[derive(SurrealValue,Debug)]
pub struct DBAccountData{
    pub id: RecordId,
    pub name:String,
    pub google_id:String,
    pub email:String,
}

impl DBAccountData{
    pub fn to_basic_data(&self) -> BasicAccountData{
        BasicAccountData{
            name:self.name.clone(),
            google_id:self.google_id.clone(),
            email:self.email.clone(),
        }
    }
    
    pub fn to_full_data(&self) -> FullAccountData{
        FullAccountData{
            name:self.name.clone(),
            google_id:self.google_id.clone(),
            email:self.email.clone(),
        }
    }
    
    pub fn to_public_data(&self) -> PublicAccountData{
        PublicAccountData{
            name:self.name.clone(),
            google_id:self.google_id.clone(),
            email:self.email.clone(),
        }
    }
}