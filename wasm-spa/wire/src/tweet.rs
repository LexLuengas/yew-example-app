#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FullTwitterResponse {
    pub rate_limit_remaining: u32,
    pub rate_limit_reset: u32,
    pub tweets: Vec<TwitterResponse>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TwitterResponse {
    pub id: String,
    pub user_name: String,
    pub user_id: String,
    pub text: String,
}
