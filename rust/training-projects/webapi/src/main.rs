mod database;
mod handlers;
mod models;

use database::Database;
use handlers::*;
use models::*;

use iron::prelude::Chain;
use iron::Iron;
use logger::Logger;
use router::Router;
use uuid::Uuid;

fn main(){
    env_logger::init();
    let (logger_before, logger_after) = Logger::new(None);
    let mut db = Database::new();

    let p = Post::new(
        "data 1",
        "data 2",
        "data 3",
        chrono::offset::Utc::now(),
        Uuid::new_v4(),
    );
    db.add_posts(p);

    let device_one = Device::new(
        "Serial 1234",
        "Model 1234",
        "SoftwareVersion 1234",
        "Vendor 1234",
        chrono::offset::Utc::now(),
        Uuid::new_v4(),
    );
    db.add_devices(device_one);

    let handlers = Handlers::new(db);
    let json_content_middleware = JsonAfterMiddleware;

    let mut router = Router::new();
    router.get("/post_feed", handlers.post_feed ,"post_feed");
    router.post("/post", handlers.post_post, "post_post");
    router.get("/post/:id", handlers.post, "post");

    let mut chain = Chain::new(router);
    chain.link_before(logger_before);
    chain.link_after(json_content_middleware);
    chain.link_after(logger_after);



    Iron::new(chain).http("localhost:8000").unwrap();
}