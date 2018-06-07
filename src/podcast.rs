use episode::PocketcastEpisode;
use user::PocketcastUser;
use reqwest::{Client, header};
use failure::Error;
use error::PocketcastError;

const GET_EPISODES_URI: &str = "https://play.pocketcasts.com/web/episodes/find_by_podcast.json";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PocketcastPodcast {
    pub id: Option<i32>,
    pub uuid: String,
    pub title: String,
    pub author: String,
    pub description: String,
    pub url: Option<String>,
    pub thumbnail_url: Option<String>
}

impl PocketcastPodcast {
    pub fn get_episodes(&self, user: &PocketcastUser) -> Result<Vec<PocketcastEpisode>, Error> {
        let body = json!({
            "uuid": self.uuid,
            "page": 1
        });
        let client = Client::new();
        let session = user.session.clone().expect("Login first");
        let mut cookies = header::Cookie::new();
        cookies.set("_social_session", session);
        let mut res = client.post(GET_EPISODES_URI)
            .header(cookies)
            .json(&body)
            .send()?;

        if !res.status().is_success() {
            return Err(Error::from(PocketcastError::HttpStatusError(res.status())));
        }

        let res: EpisodesResponse = res.json().unwrap();

        Ok(res.result.episodes)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct EpisodesResponse {
    status: String,
    token: String,
    copyright: String,
    result: EpisodesResponseResult
}

#[derive(Debug, Serialize, Deserialize)]
struct EpisodesResponseResult {
    episodes: Vec<PocketcastEpisode>
}