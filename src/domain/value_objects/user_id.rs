use uuid::Uuid;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_string(id: String) -> Result<Self, String> {
        match Uuid::parse_str(&id) {
            Ok(uuid) => Ok(Self(uuid)),
            Err(_) => Err("Invalid UUID format".to_string()),
        }
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<UserId> for String {
    fn from(user_id: UserId) -> Self {
        user_id.to_string()
    }
}

impl<'a> From<&'a UserId> for &'a Uuid {
    fn from(user_id: &'a UserId) -> Self {
        &user_id.0
    }
}