use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

pub mod result;
pub mod reverse;
pub mod league;
pub mod round;
pub mod user;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct MiddleResponse<T> {
    code: i64,
    message: String,
    body: T,
}

impl<T> MiddleResponse<T> where T: Default + Clone + Serialize + DeserializeOwned {
    pub fn success(body: T) -> Self {
        Self {
            code: 0,
            message: "Success".to_string(),
            body,
        }
    }

    pub fn error(body: T) -> Self {
        Self {
            code: 1,
            message: "Error".to_string(),
            body,
        }
    }
}