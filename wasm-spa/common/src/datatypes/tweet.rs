use wire::tweet::TwitterResponse;

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TweetData {
    pub id: String,
    pub user_name: String,
    pub user_id: String,
    pub text: String,
}

impl From<TwitterResponse> for TweetData {
    fn from(response: TwitterResponse) -> Self {
        TweetData {
            id: response.id,
            user_name: response.user_name,
            user_id: response.user_id,
            text: response.text,
        }
    }
}
