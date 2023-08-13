use actix_web::{ get, web, App, HttpResponse, HttpServer, Responder, Result, middleware::Logger };
use serde::Serialize;

mod api;
mod models;
mod repository;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[get("/health")]
async fn health_check() -> impl Responder {
    let response = Response {
        message: "Everything works fine".to_string(),
    };

    HttpResponse::Ok().json(response)
}

async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource was not found".to_string(),
    };

    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    let todo_db = repository::tweet::Database::new();
    let app_data = web::Data::new(todo_db);

    println!("🚀 Server started successfully on port 3000");

    HttpServer::new(move ||
        App::new()
            .app_data(app_data.clone())
            .configure(api::tweet::config)
            .service(health_check)
            .default_service(web::route().to(not_found))
            .wrap(Logger::default())
    )
        .bind(("127.0.0.1", 3000))?
        .run().await
}
