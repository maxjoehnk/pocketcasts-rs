use serde::{Serialize, Deserialize};
use crate::Episode;
use crate::{Podcast, DiscoverPodcast};

#[derive(Debug, Serialize, Deserialize)]
pub struct EpisodesResponse {
    pub episode_count: u32,
    pub episode_frequency: String,
    pub estimated_next_episode_at: String,
    pub has_more_episodes: bool,
    pub has_seasons: bool,
    pub season_count: u32,
    pub podcast: EpisodesPodcast
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EpisodesPodcast {
    pub audio: bool,
    pub author: String,
    pub uuid: String,
    pub category: String,
    pub description: String,
    pub title: String,
    pub url: String,
    pub episodes: Vec<Episode>
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
    pub podcasts: Vec<DiscoverPodcast>
}

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub podcasts: Vec<SearchPodcast>
}

#[derive(Debug, Deserialize, PartialEq, Clone, Serialize)]
pub struct SearchPodcast {
    pub author: String,
    pub description: String,
    pub title: String,
    pub url: String,
    pub uuid: String
}

impl SearchPodcast {
    pub fn thumbnail_url(&self) -> String {
        format!("https://static2.pocketcasts.com/discover/images/400/{}.jpg", self.uuid)
    }
}
