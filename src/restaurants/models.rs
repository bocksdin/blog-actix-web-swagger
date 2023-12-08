use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct CreateRestaurantReviewBody {
    #[schema(example = 13123, required = true)]
    restaurant_id: u32,
    #[schema(example = "Eggs Benedict", required = false)]
    menu_item: Option<String>,
    #[schema(
        example = "The restaurant was clean and the staff were helpful!",
        required = true
    )]
    review_description: String,
    #[schema(example = 5, required = true)]
    rating: u8,
    #[schema(example = true, required = true)]
    would_recommend: Option<bool>,
}
