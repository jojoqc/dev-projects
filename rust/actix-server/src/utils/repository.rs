use crate::api::device::StructDevice;
pub trait Repository {
    fn get_(&self, id: &uuid::Uuid)->Result<StructDevice, String>;
}

pub struct MemoryRepository{
    device:Vec<StructDevice>,
}

impl Default for MemoryRepository {
    fn default() -> Self{
        Self{
            device: vec![StructDevice::new("dispositivo 1".to_string(),001)] 
        }
    }
}

impl Repository for MemoryRepository {
    fn get_(&self, id: &uuid::Uuid)->Result<StructDevice, String>{
        self.device
            .iter()
            .find(|u| &u.id == id)
            .ok_or_else(|| "Invalid UUID".to_string())
            .cloned()
    }
}
