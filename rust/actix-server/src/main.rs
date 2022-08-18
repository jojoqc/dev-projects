use actix_server::utils::repository::Repository;
use actix_server::utils::repository::MemoryRepository;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::{atomic::{AtomicU16,Ordering},Arc};
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

async fn get_device(id: web::Path<String>) -> HttpResponse {
    if let Ok(parsed_device_id) = uuid::Uuid::parse_str(&id){  
        let repo = MemoryRepository::default();
        match repo.get_(&parsed_device_id) {
            Ok(device) => HttpResponse::Ok().json(device),
            Err(_) => HttpResponse::NotFound().body("Error en la peticion."),
        }
    }else{
        HttpResponse::BadRequest().body("Not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let address = format!("127.0.0.1:{}",port);
    
    println!("Starting our Server");
    let thread_counter = Arc::new(AtomicU16::new(1));
    
    HttpServer::new(move || {
        let thread_index = thread_counter.fetch_add(1, Ordering::SeqCst);
        println!("Starting thread {}",thread_index);
        let thread_index = thread_counter.load(Ordering::SeqCst);
        App::new()
            //.route("/", web::get().to(move || HttpResponse::Ok().body("DATA")))
            .service(web::resource("/device/{id}").route(web::get().to(get_device)))
            .route(
              "/status",
                web::get().to(move || {
                    HttpResponse::Ok()
                        .insert_header(("thread-id",thread_index.to_string()))
                        .body("data")
                    }),
            )
    })
    .bind(&address)?
    //.unwrap_or_else(|err| panic!("Could't start the server in port {:?}", port, err))
    .workers(8)
    .run()
    .await
}
