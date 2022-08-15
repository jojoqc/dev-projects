use actix_web::{get, web, App, HttpServer, Responder};

//#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/status")]
async fn get_status()-> impl Responder{
    HttpResponse::Ok().body("get status");
}
#[post("/status")]
async fn post_status()-> impl Responder{
    HttpResponse::Ok().body("post status");
}
#[put("/status/:id")]
async fn put_status()-> impl Responder{
    HttpResponse::Ok().body("put status");
}
#[delete("/status/:id")]
async fn delete_status()-> impl Responder{
    HttpResponse::Ok().body("delete status");
}




#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to()
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

