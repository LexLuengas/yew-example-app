use uuid::{
    Uuid,
    ParseError
};
use std::{
    fmt::{
        Display,
        Formatter,
        Result as FormatResult
    }
};


#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Default, Hash, Eq, PartialOrd, Ord)]
pub struct UserUuid(pub Uuid);

impl UserUuid {
    pub fn to_query_parameter(self) -> String {
        format!("{}={}", PARAM_NAME, self.0)
    }
    pub fn parse_str(input: &str) -> Result<Self, ParseError> {
        Uuid::parse_str(input).map(UserUuid)
    }
}

impl AsRef<Uuid> for UserUuid {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

const PARAM_NAME: &str = "user_uuid";

impl Display for UserUuid {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for UserUuid {
    fn from(uuid: Uuid) -> UserUuid {
        UserUuid(uuid)
    }
}
