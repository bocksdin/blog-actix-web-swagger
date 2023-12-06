use super::models::{CreateEntryBody, TodolistEntry, CompleteEntryBody};
use crate::AppState;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

#[get("/todolist/entries")]
async fn get_entries(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json("TODO")
}

#[post("/todolist/entries")]
async fn create_entry(
    data: web::Data<AppState>,
    body: web::Json<CreateEntryBody>,
) -> impl Responder {
    let param_obj = body.into_inner();

    HttpResponse::Ok().json("TODO")
}

#[put("/todolist/entries/{id}")]
async fn update_entry(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    body: web::Json<CompleteEntryBody>,
) -> impl Responder {
    let id = path.into_inner();
    let param_obj = body.into_inner();

    HttpResponse::Ok().json("TODO")
}

#[delete("/todolist/entries/{id}")]
async fn delete_entry(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();

    HttpResponse::Ok().json("TODO")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_entries)
        .service(create_entry)
        .service(update_entry)
        .service(delete_entry);
}
