use chrono::{DateTime,Utc};
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomData{
    random:u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Device {
    pub id: uuid::Uuid,
    pub name:String,
    pub serial:u32,
    pub custom_data:CustomData,
    pub created_at:Option<DateTime<Utc>>,
    pub updated_at:Option<DateTime<Utc>>,
}

impl Device{
    pub fn new(name: String, serial: u32) -> Self {
        //let _id = uuid::Uuid::parse_str("659e6c8b-e65e-48b7-93a4-46c57cf7318a").unwrap();
        Self {
            id: uuid::Uuid::new_v4(),
            //id:_id,
            name,
            serial,
            custom_data: CustomData{random: 1},
            created_at:Some(Utc::now()),
            updated_at:None,
        }
    } 
}
