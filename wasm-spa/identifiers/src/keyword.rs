use uuid::Uuid;
use serde_derive::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Default, Hash, Eq, PartialOrd, Ord)]
pub struct KeywordUuid(pub Uuid);
