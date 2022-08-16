use crate::api; 

pub trait Repository {
    fn get_(&self, id: uuid::Uuid)->Result<StructDevice, String>;
}

pub struct MemoryRepository{
    data:Vec<StructDevice>,
}

impl Default for MemoryRepository {
    fn default() -> Self{
        Self{
           data: vec![ImplDevice::new("dispositivo 1",001)] 
        }
    }
}

impl Repository for MemoryRepository {
    fn get_(&self, id: uuid::Uuid)->Result<StructDevice, String>{
        self.StructDevice
            .iter()
            .find(|u| u.id == id)
            //.map(|u| u.clone())
            //.ok_or_else(|e| e.to_string())
            .ok_or_else(|| "Invalid UUID".to_string())
            .cloned()
    }
}
