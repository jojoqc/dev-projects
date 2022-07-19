use crate::models::Post;
use crate::models::Device;

#[derive(Clone,Debug)]
pub struct Database {
    posts: Vec<Post>
}

impl Database{
    fn new()->Database{ Database{posts:vec![]} }

    pub fn add_posts(&mut self, post:Post){ self.posts.push(post); }
    pub fn posts(&self)->&Vec<Post>{ &self.posts() }

    pub fn add_devices(&mut self, device:Device){ self.devices().push(device)}

    pub fn devices(&self)->&Vec<Device>{ &self.devices() }
}