use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all="camelCase")]
pub struct Podcast {
    pub uuid: Uuid,
    pub title: String,
    pub author: String,
    pub description: String,
    pub url: Option<String>,
    pub auto_start_from: u32,
    pub episodes_sort_order: u32,
    pub last_epsiode_archived: Option<bool>,
    pub last_episode_published: String,
    pub last_episode_uuid: Uuid,
    pub unplayed: bool
}

impl Podcast {
    pub fn thumbnail_url(&self) -> String {
        format!("https://static2.pocketcasts.com/discover/images/400/{}.jpg", self.uuid)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct DiscoverPodcast {
    pub id: Option<i32>,
    pub uuid: Uuid,
    pub title: String,
    pub author: String,
    pub description: String,
    pub url: Option<String>,
    pub thumbnail_url: Option<String>
}
