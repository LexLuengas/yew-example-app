use identifiers::tweet::TweetUuid;
use wire::tweet::TweetResponse;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct TweetData {
    pub uuid: TweetUuid,
    pub user_name: String,
    pub text: String,
}

impl From<TweetResponse> for TweetData {
    fn from(response: TweetResponse) -> Self {
        TweetData {
            uuid: response.uuid,
            user_name: response.user_name,
            text: response.text,
        }
    }
}