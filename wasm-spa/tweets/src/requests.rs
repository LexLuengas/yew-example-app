use serde_derive::{Serialize, Deserialize};
use util::fetch::{Auth, FetchRequest, HttpMethod};

#[derive(Serialize, Deserialize)]
pub enum TwitterRequest {
    Search { query: String },
}

impl FetchRequest for TwitterRequest {
    fn resolve_path(&self) -> String {
        use self::TwitterRequest::*;
        match *self {
            Search { ref query } => format!("search/{}", query),
        }
    }
    fn resolve_auth(&self) -> Auth {
        self::Auth::NotRequired
    }
    fn resolve_body_and_method(&self) -> HttpMethod {
        self::HttpMethod::Get
    }
}
