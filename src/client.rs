use episode::Episode;
use error::PocketcastError;
use failure::Error;
use podcast::{DiscoverPodcast, Podcast};
use reqwest::{Client, Response, StatusCode};
use responses::*;
use serde_json::value::Value;
use user::User;
use api;

#[derive(Debug, Default, Clone)]
pub struct PocketcastClient {
    user: User,
    token: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct LoginResponseBody {
    token: String,
    uuid: String
}

impl PocketcastClient {
    pub fn new<S: Into<String>>(email: S, password: S) -> PocketcastClient {
        PocketcastClient {
            user: User {
                email: email.into(),
                password: password.into(),
            },
            ..PocketcastClient::default()
        }
    }

    pub fn with_user(user: User) -> PocketcastClient {
        PocketcastClient {
            user,
            ..PocketcastClient::default()
        }
    }

    pub fn login(&mut self) -> Result<(), Error> {
        let body = json!({
            "email": self.user.email.clone(),
            "password": self.user.password.clone(),
            "scope": "webplayer"
        });

        let mut res = self.post(api::LOGIN_URI, Some(body))?;

        if res.status() == StatusCode::UNAUTHORIZED {
            Err(Error::from(PocketcastError::InvalidCredentials))?;
        }

        if !res.status().is_success() {
            Err(Error::from(PocketcastError::HttpStatusError(res.status())))?;
        }

        let res: LoginResponseBody = res.json()?;

        self.token = Some(res.token);

        Ok(())
    }

    pub fn get_subscriptions(&self) -> Result<Vec<Podcast>, Error> {
        let body = json!({
            "v": 1
        });
        let mut res = self.post(api::GET_SUBSCRIPTIONS_URI, Some(body))?;

        if !res.status().is_success() {
            return Err(Error::from(PocketcastError::HttpStatusError(res.status())));
        }

        let res: SubscriptionsResponse = res.json()?;

        Ok(res.podcasts)
    }

    pub fn get_episodes(&self, podcast_id: &str) -> Result<Vec<Episode>, Error> {
        let url = format!("{}/{}/{}", api::GET_EPISODES_URI_PREFIX, podcast_id, api::GET_EPISODES_URI_SUFFIX);
        let mut res = self.get(&url, None)?;

        if !res.status().is_success() {
            return Err(Error::from(PocketcastError::HttpStatusError(res.status())));
        }

        let res: EpisodesResponse = res.json()?;

        Ok(res.podcast.episodes)
    }

    pub fn search_podcasts<Q: Into<String>>(&self, query: Q) -> Result<Vec<SearchPodcast>, Error> {
        let body = json!({ "term": query.into() });
        let mut res = self.post(api::SEARCH_PODCASTS_URI, Some(body))?;

        if !res.status().is_success() {
            return Err(Error::from(PocketcastError::HttpStatusError(res.status())));
        }

        let res: SearchResponse = res.json()?;

        Ok(res.podcasts)
    }

    pub fn get_top_charts() -> Result<Vec<DiscoverPodcast>, Error> {
        PocketcastClient::get_discover(api::GET_TOP_CHARTS_URI)
    }

    pub fn get_featured() -> Result<Vec<DiscoverPodcast>, Error> {
        PocketcastClient::get_discover(api::GET_FEATURED_URI)
    }

    pub fn get_trending() -> Result<Vec<DiscoverPodcast>, Error> {
        PocketcastClient::get_discover(api::GET_TRENDING_URI)
    }

    fn post(&self, url: &'static str, body: Option<Value>) -> Result<Response, Error> {
        let client = Client::new();

        let mut request_builder = client.post(url);
        if let Some(ref token) = self.token {
            request_builder = request_builder.bearer_auth(token);
        }

        let res = match body {
            Some(json) => request_builder
                .json(&json)
                .send(),
            None => request_builder.send()
        };
        Ok(res?)
    }

    fn get(&self, url: &str, query: Option<Vec<(String, String)>>) -> Result<Response, Error> {
        let client = Client::new();

        let mut request_builder = client.get(url);
        if let Some(ref token) = self.token {
            request_builder = request_builder.bearer_auth(token);
        }

        let res = match query {
            Some(json) => request_builder
                .query(&json)
                .send(),
            None => request_builder
                .send()
        };
        Ok(res?)
    }

    fn get_discover(uri: &'static str) -> Result<Vec<DiscoverPodcast>, Error> {
        let client = Client::new();
        let mut res = client
            .get(uri)
            .send()?;

        if !res.status().is_success() {
            return Err(Error::from(PocketcastError::HttpStatusError(res.status())));
        }

        let res: DiscoverResponse = res.json()?;

        Ok(res.result.ok_or(PocketcastError::EmptyResponse).map(|result| result.podcasts)?)
    }
}

#[cfg(test)]
mod tests {
    use failure::Error;
    use User;
    use super::*;

    fn create_user() -> User {
        User::new(env!("POCKETCAST_EMAIL"), env!("POCKETCAST_PASSWORD"))
    }

    fn login() -> Result<PocketcastClient, Error> {
        let user = create_user();
        let mut client = PocketcastClient::with_user(user);
        client.login()?;
        Ok(client)
    }

    #[test]
    fn it_logs_in() {
        let client = login().unwrap();
        assert_ne!(client.token, None);
    }

    #[test]
    fn it_should_fetch_subscriptions() {
        let client = login().unwrap();
        let subscriptions = client.get_subscriptions().unwrap();
        assert_ne!(subscriptions, vec![]);
    }

    #[test]
    fn it_should_fetch_top_charts() {
        let charts = PocketcastClient::get_top_charts().unwrap();
        assert_ne!(charts, vec![]);
    }

    #[test]
    fn it_should_fetch_featured() {
        let charts = PocketcastClient::get_featured().unwrap();
        assert_ne!(charts, vec![]);
    }

    #[test]
    fn it_should_fetch_trending() {
        let charts = PocketcastClient::get_trending().unwrap();
        assert_ne!(charts, vec![]);
    }

    #[test]
    fn it_should_fetch_podcast_episodes() {
        let client = login().unwrap();
        let episodes = client.get_episodes("4f7ad040-8f5b-0135-9cea-5bb073f92b78").unwrap();
        assert_ne!(episodes, vec![]);
    }

    #[test]
    fn it_should_search_for_podcasts() {
        let client = login().unwrap();
        let podcasts = client.search_podcasts("Film Riot").unwrap();
        assert_ne!(podcasts, vec![]);
    }
}