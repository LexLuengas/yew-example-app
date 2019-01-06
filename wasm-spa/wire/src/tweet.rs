use identifiers::tweet::TweetUuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TweetResponse {
    pub uuid: TweetUuid,
    pub user_name: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FullTweetResponse {
    pub uuid: TweetUuid,
    pub user_name: String,
    pub text: String,
    pub locked: bool,
    pub banned: bool,
}
