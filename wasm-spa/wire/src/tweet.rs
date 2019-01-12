use identifiers::tweet::TweetUuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TwitterResponse {
    pub uuid: TweetUuid,
    pub user_name: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FullTwitterResponse {
    pub uuid: TweetUuid,
    pub user_name: String,
    pub text: String,
    pub locked: bool,
    pub banned: bool,
}
