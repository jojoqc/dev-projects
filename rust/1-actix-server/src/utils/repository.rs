use crate::api::device::Device;
use async_trait::async_trait;
use chrono::Utc;
use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;


/*
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
*/



//new functionality
#[derive(Error, Debug)]
pub enum RespositoryError{
    #[error("PoisonError: `{0}`")]
    LockError(String),
    #[error("This entity already exists")]
    AlreadyExists,
    #[error("This entity does not exist")]
    DoesNotExists,
    #[error("The id format is not valid")]
    InvalidId,

}

impl<T> From<PoisonError<T> for RepositoryError{
    fn from(poison_error: PoisonError<T>) -> Self{
        Repository::LockError(poison_error.to_string())
    }
}

pub type RespositoryResult<T> = Result<T, RepositoryError>;
pub struct Table{
    name: String
}
pub struct Field{
    fields: &str[]
}
//CREATE A DINAMIC QUERY BUILDER BASED ON VECTOR STRING REPLACING 'REPOSITORYRESULT<>'
pub fn query_builder(table:str,field:){
    
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait Repository: Send + Sync + 'static{
    //post function create_
    async fn create_(&self, device:&Device, table:&Table.name) -> RepositoryResult<Device>;
    //get function read_
    async fn read_(&self, id:&Uuid, table:&Table.name) -> RepositoryResult<Device>;
    //put function update_
    async fn update_(&self, id:&Device, table:&Table.name) -> RepositoryResult<Device>;
    //delete function delete_
    async fn delete_(&self, id:&Uuid, table:&Table.name) -> RepositoryResult<Device>;
}

pub struct PostgresRepository{
    pool: sqlx::PgPool,
}


impl PostgresRepository {
    pub async fn from_env() -> sqlx::Result<Self>{
        let conn_str = 
            std::env::var("DATABASE_URL").map_err(|e| sqlx::Error::Configuration(Box::new(e)))?;
        let pool = sqlx::PgPool::connect(&conn_str).await?;
        Ok(self{pool})
    }
}

#[async_trait]
impl Repository for PostgresRepository{
    #[instrument(skip(self))]
    async fn read_(&self, id: &uuid::Uuid) RespositoryResult<Device>{
        let result = sqlx::query_as::<_, Device>(
            "SELECT * FROM" device "WHERE id = $1"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await;

        result.map_err(|e|{
            tracing::error!("{:?}", e);
            RepositoryError::InvalidId
        })
    }
    
    #[instrument(skip(self))]
    async fn create_(&self, object)


}



















