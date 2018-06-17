use cookie::Cookie;
use episode::Episode;
use error::PocketcastError;
use failure::Error;
use podcast::Podcast;
use reqwest::{Client, header, RedirectPolicy, Response, StatusCode};
use responses::*;
use serde_json::value::Value;
use user::User;
use api;

#[derive(Debug, Default, Clone)]
pub struct PocketcastClient {
    user: User,
    session: Option<String>,
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
        let body = [
            ("[user]email", self.user.email.as_str()),
            ("[user]password", self.user.password.as_str())
        ];

        let client = Client::builder()
            .redirect(RedirectPolicy::none())
            .build()?;
        let res = client.post(api::LOGIN_URI)
            .form(&body)
            .send()?;

        if res.status() == StatusCode::Ok {
            Err(Error::from(PocketcastError::InvalidCredentials))?;
        }

        if res.status() != StatusCode::Found {
            Err(Error::from(PocketcastError::HttpStatusError(res.status())))?;
        }

        let cookies = res.headers().get::<header::SetCookie>().ok_or(PocketcastError::MissingSession)?;

        let session = cookies
            .iter()
            .map(|header| Cookie::parse(header.to_string()).unwrap())
            .find(|cookie| cookie.name() == "_social_session")
            .map(|cookie| cookie.value().to_string())
            .ok_or(PocketcastError::MissingSession)?;

        self.session = Some(session);

        Ok(())
    }

    pub fn get_subscriptions(&self) -> Result<Vec<Podcast>, Error> {
        let mut res = self.post(api::GET_SUBSCRIPTIONS_URI, None)?;

        if !res.status().is_success() {
            return Err(Error::from(PocketcastError::HttpStatusError(res.status())));
        }

        let res: SubscriptionsResponse = res.json()?;

        Ok(res.podcasts)
    }

    pub fn get_podcast<S: Into<String>>(&self, uuid: S) -> Result<Podcast, Error> {
        let body = json!({
            "uuid": uuid.into()
        });
        let mut res = self.post(api::GET_PODCAST_URI, Some(body))?;

        if !res.status().is_success() {
            return Err(Error::from(PocketcastError::HttpStatusError(res.status())));
        }

        let res: PodcastResponse = res.json()?;

        Ok(res.podcast)
    }

    pub fn get_episodes(&self, podcast: &Podcast) -> Result<Vec<Episode>, Error> {
        let body = json!({
            "uuid": podcast.uuid,
            "page": 1
        });
        let mut res = self.post(api::GET_EPISODES_URI, Some(body))?;

        if !res.status().is_success() {
            return Err(Error::from(PocketcastError::HttpStatusError(res.status())));
        }

        let res: EpisodesResponse = res.json()?;

        Ok(res.result.episodes)
    }

    pub fn search_podcasts<Q: Into<String>>(&self, query: Q) -> Result<Vec<Podcast>, Error> {
        let query = vec![("term".to_string(), query.into())];
        let mut res = self.get(api::SEARCH_PODCASTS_URI, Some(query))?;

        if !res.status().is_success() {
            return Err(Error::from(PocketcastError::HttpStatusError(res.status())));
        }

        let res: SearchResponse = res.json()?;

        Ok(res.podcasts)
    }

    pub fn get_top_charts() -> Result<Vec<Podcast>, Error> {
        PocketcastClient::get_discover(api::GET_TOP_CHARTS_URI)
    }

    pub fn get_featured() -> Result<Vec<Podcast>, Error> {
        PocketcastClient::get_discover(api::GET_FEATURED_URI)
    }

    pub fn get_trending() -> Result<Vec<Podcast>, Error> {
        PocketcastClient::get_discover(api::GET_TRENDING_URI)
    }

    fn post(&self, url: &'static str, body: Option<Value>) -> Result<Response, Error> {
        let client = Client::new();
        let session = self.session.clone().ok_or(PocketcastError::NoSession)?;
        let mut cookies = header::Cookie::new();
        cookies.set("_social_session", session);
        let res = match body {
            Some(json) => client
                .post(url)
                .header(cookies)
                .json(&json)
                .send(),
            None => client
                .post(url)
                .header(cookies)
                .send()
        };
        Ok(res?)
    }

    fn get(&self, url: &'static str, query: Option<Vec<(String, String)>>) -> Result<Response, Error> {
        let client = Client::new();
        let session = self.session.clone().ok_or(PocketcastError::NoSession)?;
        let mut cookies = header::Cookie::new();
        cookies.set("_social_session", session);
        let res = match query {
            Some(json) => client
                .get(url)
                .header(cookies)
                .query(&json)
                .send(),
            None => client
                .get(url)
                .header(cookies)
                .send()
        };
        Ok(res?)
    }

    fn get_discover(uri: &'static str) -> Result<Vec<Podcast>, Error> {
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
        assert_ne!(client.session, None);
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
    fn it_should_fetch_a_podcast() {
        let client = login().unwrap();
        let _podcast = client.get_podcast("4f7ad040-8f5b-0135-9cea-5bb073f92b78").unwrap();
    }

    #[test]
    fn it_should_fetch_podcast_episodes() {
        let client = login().unwrap();
        let podcast = client.get_podcast("4f7ad040-8f5b-0135-9cea-5bb073f92b78").unwrap();
        let episodes = client.get_episodes(&podcast).unwrap();
        assert_ne!(episodes, vec![]);
    }

    #[test]
    fn it_should_search_for_podcasts() {
        let client = login().unwrap();
        let podcasts = client.search_podcasts("Film Riot").unwrap();
        assert_ne!(podcasts, vec![]);
    }
}