use wire::tweet::{FullTwitterResponse, TwitterResponse};
use util::table::{Table, TableData};
use yew::prelude::*;

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

impl TableData for TweetData {
    fn get_field_as_html(&self, field_name: &str) -> Html<Table<Self>> {
        match field_name {
            "id" => html! { { self.id.clone() } },
            "user_name" => html! { { self.user_name.clone() } },
            "user_id" => html! { { self.user_id.clone() } },
            "text" => html! { { self.text.clone() } },
            n => panic!("Unexpected field name {}", n),
        }
    }
}