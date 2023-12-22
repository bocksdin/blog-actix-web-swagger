use super::models::CreateRestaurantReviewBody;
use crate::AppState;
use actix_web::{get, post, web, HttpResponse, Responder};
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
pub struct GetRestaurantReviewPathParams {
    restaurant_id: u32,
}

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct GetRestaurantReviewQueryParams {
    menu_item: Option<String>,
}

#[utoipa::path(params(GetRestaurantReviewPathParams, GetRestaurantReviewQueryParams))]
#[get("/restaurants/{restaurant_id}/review")]
async fn get_restaurant_review_with_path_params(
    path_params: web::Path<GetRestaurantReviewPathParams>,
    query_params: web::Query<GetRestaurantReviewQueryParams>,
) -> impl Responder {
    let path_param_obj = path_params.into_inner();
    let query_param_obj = query_params.into_inner();

    HttpResponse::Ok().json("TODO")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_restaurant_review)
        .service(get_restaurant_review_with_path_params);
}
