use super::models::CreateRestaurantReviewBody;
use crate::AppState;
use actix_web::{post, web, HttpResponse, Responder};

#[utoipa::path(
    request_body = CreateRestaurantReviewBody,
)]
#[post("/restaurants/review")]
async fn create_restaurant_review(
    data: web::Data<AppState>,
    body: web::Json<CreateRestaurantReviewBody>,
) -> impl Responder {
    let param_obj = body.into_inner();

    HttpResponse::Ok().json("TODO")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_restaurant_review);
}
