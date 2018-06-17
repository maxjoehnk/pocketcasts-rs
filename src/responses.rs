use Episode;
use Podcast;

#[derive(Debug, Serialize, Deserialize)]
pub struct EpisodesResponse {
    pub status: String,
    pub token: String,
    pub copyright: String,
    pub result: EpisodesResponseResult
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EpisodesResponseResult {
    pub episodes: Vec<Episode>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PodcastResponse {
    pub podcast: Podcast
}

#[derive(Debug, Deserialize)]
pub struct SubscriptionsResponse {
    pub podcasts: Vec<Podcast>
}

#[derive(Debug, Deserialize)]
pub struct DiscoverResponse {
    pub result: Option<DiscoverResult>,
    pub status: String
}

#[derive(Debug, Deserialize)]
pub struct DiscoverResult {
    pub podcasts: Vec<Podcast>
}

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub podcasts: Vec<Podcast>
}