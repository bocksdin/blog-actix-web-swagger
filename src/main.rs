use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};

mod todolist;
use todolist::services;

struct AppState {}

#[get("/")] // GET method for the "/" path
async fn index() -> impl Responder {
    HttpResponse::Ok().json("{ status: OK }")
}

// This tells our program to utilize the actix_web runtime
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {}))
            .service(index)
            .configure(services::config)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
