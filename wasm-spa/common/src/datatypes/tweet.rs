use identifiers::tweet::TweetUuid;
use wire::tweet::TwitterResponse;

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TweetData {
    pub uuid: TweetUuid,
    pub user_name: String,
    pub text: String,
}

impl From<TwitterResponse> for TweetData {
    fn from(response: TwitterResponse) -> Self {
        TweetData {
            uuid: response.uuid,
            user_name: response.user_name,
            text: response.text,
        }
    }
}
