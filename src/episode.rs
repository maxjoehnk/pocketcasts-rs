#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Episode {
    pub uuid: String,
    pub file_size: i32,
    pub file_type: String,
    pub title: String,
    pub url: String,
    pub duration: u64,
}
