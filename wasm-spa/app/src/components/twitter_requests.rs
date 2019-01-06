use common::fetch::Auth;
use common::fetch::FetchRequest;
use common::fetch::HttpMethod;

#[derive(Serialize, Deserialize)]
pub enum TwitterRequest {
    Test,
    Search { keyword: String },
}

impl FetchRequest for TwitterRequest {
    fn resolve_path(&self) -> String {
        use self::TwitterRequest::*;
        match *self {
            Test => "test".into(),
            Search { ref keyword } => format!("keyword/{}", keyword),
        }
    }
    fn resolve_auth(&self) -> Auth {
        self::Auth::NotRequired
    }
    fn resolve_body_and_method(&self) -> HttpMethod {
        self::HttpMethod::Get
    }
}
