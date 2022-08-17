use actix_server::utils::repository::Repository;
use actix_server::utils::repository::MemoryRepository;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::sync::{atomic::{AtomicU16,Ordering},Arc};
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

async fn get_device(req: HttpRequest) -> HttpResponse {
    if let Some(id) = req.match_info().get("id"){
        match uuid::Uuid::parse_str(id) {
            Ok(parsed_device_id) =>{
                let repo = MemoryRepository::default();
                match repo.get_(parsed_device_id) {
                    Ok(device) => HttpResponse::Ok().json(device),
                    Err(_) => HttpResponse::NotFound().body("Error en la peticion.")
                }
            }
            Err(_)=> HttpResponse::BadRequest().body("Fallo en la peticion")
        }
    } else{
        HttpResponse::NotFound().body("Not found")
    }
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let address = format!("127.0.0.1:{}",port);
    
    println!("Starting our Server");
    let thread_counter = Arc::new(AtomicU16::new(1));
    
    HttpServer::new(move || {
        println!("Starting thread {}",thread_counter.fetch_add(1, Ordering::SeqCst));
        let thread_index = thread_counter.load(Ordering::SeqCst);
        App::new()
            .service(web::resource("/device/{id}").route(web::get().to(get_device)))
            .route("/", web::get().to(greet))
            /*.route(\
              "/status",
                web::get().to(move || {
                    HttpResponse::Ok()
                        .header("thread-id",thread_index.to_string())
                        .finish()
                    }),
                )*/
            //.route("/start",web::get().to(move || async {"start"}))
            .route("/{name}", web::get().to(greet))
    })
    .bind(&address)?
    .workers(8)
    .run()
    .await
}
