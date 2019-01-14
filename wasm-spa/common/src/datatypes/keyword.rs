use identifiers::keyword::KeywordUuid;
use url::form_urlencoded::parse;
use yew_router::prelude::*;

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Keyword {
    pub uuid: KeywordUuid,
    pub query: String,
    pub title: String,
}

impl From<&str> for Keyword {
    fn from(s: &str) -> Self {
        Keyword {
            query: s.to_string(),
            title: s.to_string(),
            ..Default::default()
        }
    }
}

pub fn keywords_from_route(route: &Route) -> Vec<Keyword> {
    let keywords_string: String = route.query.clone().unwrap_or_default();
    let keywords_string = keywords_string.trim_start_matches("q=");
    let keywords_string: String = parse(keywords_string.as_bytes())
        .map(|(key, val)| [key, val].concat())
        .collect();
    keywords_string
        .split(" OR ")
        .map(|s| Keyword::from(s))
        .collect()
}
