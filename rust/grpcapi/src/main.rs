use tonic::{transport::Server, Request, Response, Status};

use crate::device::device_server::{Device, DeviceServerImpl};
use crate::device::{GetDeviceRequest, GetDeviceResponse};

mod device {
    include!("device.rs");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("device_descriptor");
}

#[derive(Default)]
pub struct DeviceImpl {}

#[tonic::async_trait]
impl Device for DeviceImpl {
    async fn get_device(
        &self,
        request: Request<GetDeviceRequest>,
    ) -> Result<Response<GetDeviceResponse>, Status> {
        let response = GetDeviceResponse {
            id: request.into_inner().id,
            author: "root".to_owned(),
            name: "Zero to One".to_owned(),
            year: 2022,
        };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let device = DeviceImpl::default();

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(device::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    println!("Device server listening on {}", addr);

    Server::builder()
        .add_service(DeviceServerImpl::new(device))
        .add_service(reflection_service)
        .serve(addr)
        .await?;
    Ok(())
}
