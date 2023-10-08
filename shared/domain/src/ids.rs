use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Serialize, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "query", derive(DieselNewType))] // allows to use a UserId directly with the DB without having to extract the Uuid out of it
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn into_inner(self) -> Uuid {
        self.0
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
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

impl From<Uuid> for UserId {
    fn from(id: Uuid) -> Self {
        UserId(id)
    }
}

impl FromStr for UserId {
    type Err = IdError;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        uuid::Uuid::try_parse(id)
            .map(|id| id.into())
            .map_err(|_| IdError::Parse)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum IdError {
    #[error("failed to parse ID")]
    Parse,
}