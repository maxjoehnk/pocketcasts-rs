use episode::PocketcastEpisode;

const GET_EPISODES_URI: &str = "https://play.pocketcasts.com/web/episodes/find_by_podcast.json";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PocketcastPodcast {
    id: Option<i32>,
    pub uuid: String,
    pub title: String,
    pub author: String,
    pub description: String,
    pub url: Option<String>,
    pub thumbnail_url: Option<String>,
    #[serde(skip)]
    pub episodes: Vec<PocketcastEpisode>,
}
