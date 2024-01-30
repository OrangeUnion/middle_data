use serde::{Deserialize, Serialize};

pub mod record;
pub mod reverse;
pub mod league;
pub mod round;
pub mod user;

pub enum ResMessage<T, E> {
    Success(T),
    Failed(E),
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct MiddleResponse<T> where T: Serialize {
    code: i64,
    message: String,
    body: T,
}

impl<T> MiddleResponse<T> where T: Serialize {
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