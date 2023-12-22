use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};

use utoipa::OpenApi;
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify,
};
use utoipa_swagger_ui::SwaggerUi;

mod restaurants;
use restaurants::{
    models::CreateRestaurantReviewBody,
    services::{self, GetRestaurantReviewPathParams, GetRestaurantReviewQueryParams},
};

struct AppState {}

#[derive(serde::Serialize, utoipa::ToSchema)]
struct SimpleStatus {
    status: u16,
}

#[utoipa::path(
    responses(
        (status = 200, description = "API is alive and well!", body = SimpleStatus)
    )
)]
#[get("/")] // GET method for the "/" path
async fn index() -> impl Responder {
    HttpResponse::Ok().json(SimpleStatus { status: 200 })
}

// This tells our program to utilize the actix_web runtime
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            index,
            restaurants::services::create_restaurant_review,
            restaurants::services::get_restaurant_review_with_path_params,
        ),
        components(
            schemas(
                SimpleStatus,
                CreateRestaurantReviewBody,
                GetRestaurantReviewPathParams,
                GetRestaurantReviewQueryParams,
            ),
        ),
        modifiers(&SecurityModifier)
    )]
    struct ApiDoc;

    struct SecurityModifier;
    impl Modify for SecurityModifier {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            let components = openapi.components.as_mut().unwrap();
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
            components.add_security_scheme(
                "basic_auth",
                SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Basic).build()),
            );
        }
    }

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {}))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(index)
            .configure(services::config)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
