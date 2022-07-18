use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use uuid::Uuid;

#[derive(Clone,Debug,RustcEncodable,RustcDecodable)]
pub struct Post{
    title: String,
    body:String,
    author:String,
    datetime:Datetime<UTC>,
    uuid:Uuid
}
impl Post {
    pub fn new(
        title:&str,
        body:&str,
        author:&str,
        datetime:DateTime<UTC>,
        uuid: Uuid) -> Post{
            Post {
                title:title.to_string(),
                body:body.to_string(),
                author:author.to_string(),
                datetime,
                uuid,
            }
    }
    pub fn uuid(&self)->&Uuid {
        &self.uuid
    }
}

#[derive(Clone,Debug,RustcEncodable,RustcDecodable)]
pub struct Config{
    ip:String,
    user:String,
    sshkey:String,
    port:String,
    port_forward:String,
    datetime:Datetime<UTC>,
    uuid:Uuid
}
impl Config {
    pub fn new(
        ip:&str,
        user:&str,
        sshkey:&str,
        port:&str,
        port_forward:&str,
        datetime:Datetime<UTC>,
        uuid:Uuid
    ) -> Config{
        Config {
            ip:ip.to_string(),
            user:user.to_string(),
            sshkey:sshkey.to_string(),
            port:port.to_string(),
            port_forward:port_forward.to_string(),
            datetime,
            uuid
        }
    }
    pub fn uuid(&self)->&Uuid {
        &self.uuid
    }
}

#[derive(Clone,Debug,RustcEncodable,RustcDecodable)]
pub struct Device {
    serial:String,
    model:String,
    software_version:String,
    vendor:String,
    datetime:Datetime<UTC>,
    uuid:Uuid
}
impl Device {
    pub fn new(
        serial:&str,
        model:&str,
        software_version:&str,
        vendor:&str,
        datetime:Datetime<UTC>,
        uuid:Uuid
    ) -> Device{
        Device {
            serial:serial.to_string(),
            model:model.to_string(),
            software_version:software_version.to_string(),
            vendor:vendor.to_string(),
            datetime,
            uuid
        }
    }
    pub fn uuid(&self)->&Uuid {
        &self.uuid
    }
}