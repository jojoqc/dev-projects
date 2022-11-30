use actix_server::utils::repository::Repository;
use actix_server::utils::repository::MemoryRepository;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::{atomic::{AtomicU16,Ordering},Arc};

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

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
        .append_header(("thread-id",thread_index.to_string()))
        .finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //init env vars
    dotenv::dotenv().ok();
    //init tracing subscriber for diagnosis
    let tracing = tracing_subscriber::fmt()
        .with_timer(tracing_subscriber::fmt::time::UtcTIme::rfc_3339())
        .with_env_filter(EnvFilter::from_default_env());
    if cfg!(debug_assertions){
        tracing.pretty().init();
    } else{
        tracing.json().init();
    }


    //building address
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let address = format!("127.0.0.1:{}",port);
    // building share state
    println!("Starting Server at {}", address);
    let thread_counter = Arc::new(AtomicU16::new(1));
    // Start the server 
    HttpServer::new(move || {
        let thread_index = thread_counter.fetch_add(1, Ordering::SeqCst);
        println!("Starting thread {}",thread_index);
        let thread_index = thread_counter.load(Ordering::SeqCst);
        //start the services
        App::new()
            .service(web::resource("/device/{id}").route(web::get().to(get_device)))
            .service(web::resource("/checkinfo/").route(web::get().to(health_check)))
    })
    .bind(&address)?
    .unwrap_or_else(|err|{
        panic!("🔥🔥🔥 Couldn't start the server in port {}: {:?}",port, err)
    })
    .workers(8)
    .run()
    .await
}
