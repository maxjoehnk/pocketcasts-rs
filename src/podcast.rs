#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Podcast {
    pub id: Option<i32>,
    pub uuid: String,
    pub title: String,
    pub author: String,
    pub description: String,
    pub url: Option<String>,
    pub thumbnail_url: Option<String>
}