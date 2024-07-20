use validator::Validate;

#[derive(serde::Serialize, serde::Deserialize, Validate)]
pub struct CreateGreetingRequest {
    #[validate(length(min = 1))]
    pub greeting: String,
}
