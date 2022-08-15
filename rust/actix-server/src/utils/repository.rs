use mod api; 

pub trait Repository {
    fn get_(&self, data_id::uuid::Uuid)->Result<ImplDevice, String>;
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
