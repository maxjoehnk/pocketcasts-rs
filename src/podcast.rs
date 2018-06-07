#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Podcast {
    pub id: Option<i32>,
    pub uuid: String,
    pub title: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub thumbnail_url: Option<String>
}

impl Podcast {
    pub fn new<S: Into<String>>(uuid: S) -> Podcast {
        Podcast {
            uuid: uuid.into(),
            ..Podcast::default()
        }
    }
}
