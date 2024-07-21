use domain::models::greeting::Greeting;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetGreetingsResponse {
    pub greetings: Vec<Greeting>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateGreetingResponse {
    pub greeting: Greeting,
}
