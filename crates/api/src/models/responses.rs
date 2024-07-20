use domain::models::greeting::Greeting;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GreetingsResponse {
    pub greetings: Vec<Greeting>
}
