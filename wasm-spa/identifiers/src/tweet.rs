use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Default, Hash, Eq, PartialOrd, Ord)]
pub struct TweetUuid(pub Uuid);
