use identifiers::keyword::KeywordUuid;
use url::form_urlencoded::{parse, byte_serialize};

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

pub fn keywords_from_query(query: String) -> Vec<Keyword> {
    let keywords_string = query.trim_start_matches("q=");
    let keywords_string: String = parse(keywords_string.as_bytes())
        .map(|(key, val)| [key, val].concat())
        .collect();
    keywords_string
        .split(" OR ")
        .map(|s| Keyword::from(s))
        .collect()
}

pub fn query_from_keywords(keywords: &Vec<Keyword>) -> String {
        let mut query: String = keywords
            .clone()
            .into_iter()
            .map(|k| k.query)
            .collect::<Vec<String>>()
            .join(" OR ");
        // Always filter out retweets 
        query.push_str(" -filter:retweets");
        let urlencoded: String = byte_serialize(&query.as_bytes()).collect();
        format!("q={}", urlencoded)
    }