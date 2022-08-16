use chrono::{DateTime,Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize,Serialize)]
pub struct StructCustomData{
    random:u32,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StructDevice {
    id: uuid::Uuid,
    name:String,
    serial:u32,
    custom_data:StructCustomData,
    created_at:Option<DateTime<Utc>>,
    updated_at:Option<DateTime<Utc>>,
}

impl StructDevice{
    pub fn new(name: String, serial: u32) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name,
            serial,
            custom_data: StructCustomData{random: 1},
            created_at:Some(Utc::now()),
            updated_at:None,
        }
    } 
}
