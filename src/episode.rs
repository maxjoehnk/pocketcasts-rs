use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Episode {
    pub uuid: Uuid,
    pub file_size: i32,
    pub file_type: String,
    pub title: String,
    pub url: String,
    pub duration: u64,
}
