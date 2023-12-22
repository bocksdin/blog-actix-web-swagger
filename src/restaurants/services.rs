use super::models::CreateRestaurantReviewBody;
use crate::AppState;
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

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

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct CreateRestaurantReviewPathParams {
    restaurant_id: u32,
}

#[utoipa::path(
    params(CreateRestaurantReviewPathParams),
    request_body = CreateRestaurantReviewBody,
)]
#[post("/restaurants/{restaurant_id}/review")]
async fn create_restaurant_review_with_path_params(
    path_params: web::Path<CreateRestaurantReviewPathParams>,
    body: web::Json<CreateRestaurantReviewBody>,
) -> impl Responder {
    let path_param_obj = path_params.into_inner();
    let param_obj = body.into_inner();

    HttpResponse::Ok().json("TODO")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_restaurant_review)
        .service(create_restaurant_review_with_path_params);
}
