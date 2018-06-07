use failure::Error;
use reqwest::{Client, header, StatusCode, RedirectPolicy};
use podcast::PocketcastPodcast;
use error::PocketcastError;
use cookie::Cookie;

const LOGIN_URI: &str = "https://play.pocketcasts.com/users/sign_in";
const GET_SUBSCRIPTIONS_URI: &str = "https://play.pocketcasts.com/web/podcasts/all.json";
const GET_TOP_CHARTS_URI: &str = "https://static.pocketcasts.com/discover/json/popular_world.json";
const GET_FEATURED_URI: &str = "https://static.pocketcasts.com/discover/json/featured.json";
const GET_TRENDING_URI: &str = "https://static.pocketcasts.com/discover/json/trending.json";

#[derive(Debug, Deserialize, Clone, Default)]
pub struct PocketcastUser {
    email: String,
    password: String,
    pub session: Option<String>
}

impl PocketcastUser {
    pub fn new<S: Into<String>>(email: S, password: S) -> PocketcastUser {
        PocketcastUser {
            email: email.into(),
            password: password.into(),
            session: None
        }
    }

    pub fn login(&mut self) -> Result<(), Error> {
        let body = [
            ("[user]email", self.email.as_str()),
            ("[user]password", self.password.as_str())
        ];

        let client = Client::builder()
            .redirect(RedirectPolicy::none())
            .build()?;
        let res = client.post(LOGIN_URI)
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

    pub fn get_subscriptions(&self) -> Result<Vec<PocketcastPodcast>, Error> {
        let client = Client::new();
        let session = self.session.clone().ok_or(PocketcastError::NoSession)?;
        let mut cookies = header::Cookie::new();
        cookies.set("_social_session", session);
        let mut res = client.post(GET_SUBSCRIPTIONS_URI)
            .header(cookies)
            .send()?;

        if !res.status().is_success() {
            return Err(Error::from(PocketcastError::HttpStatusError(res.status())));
        }

        let res: SubscriptionsResponse = res.json()?;

        Ok(res.podcasts)
    }

    pub fn get_top_charts(&self) -> Result<Vec<PocketcastPodcast>, Error> {
        self.get_discover(GET_TOP_CHARTS_URI)
    }

    pub fn get_featured(&self) -> Result<Vec<PocketcastPodcast>, Error> {
        self.get_discover(GET_FEATURED_URI)
    }

    pub fn get_trending(&self) -> Result<Vec<PocketcastPodcast>, Error> {
        self.get_discover(GET_TRENDING_URI)
    }

    fn get_discover(&self, uri: &'static str) -> Result<Vec<PocketcastPodcast>, Error> {
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

#[derive(Debug, Deserialize)]
struct SubscriptionsResponse {
    podcasts: Vec<PocketcastPodcast>
}

#[derive(Debug, Deserialize)]
struct DiscoverResponse {
    result: Option<DiscoverResult>,
    status: String
}

#[derive(Debug, Deserialize)]
struct DiscoverResult {
    podcasts: Vec<PocketcastPodcast>
}

#[cfg(test)]
mod tests {
    use super::*;
    use failure::Error;

    fn create_user() -> PocketcastUser {
        PocketcastUser::new(env!("POCKETCAST_EMAIL"), env!("POCKETCAST_PASSWORD"))
    }

    fn login() -> Result<PocketcastUser, Error> {
        let mut user = create_user();
        user.login()?;
        Ok(user)
    }

    #[test]
    fn it_logs_in() {
        let user = login().unwrap();
        assert_ne!(user.session, None);
    }

    #[test]
    fn it_should_fetch_subscriptions() {
        let user = login().unwrap();
        let subscriptions = user.get_subscriptions().unwrap();
        assert_ne!(subscriptions, vec![]);
    }

    #[test]
    fn it_should_fetch_top_charts() {
        let user = create_user();
        let charts = user.get_top_charts().unwrap();
        assert_ne!(charts, vec![]);
    }

    #[test]
    fn it_should_fetch_featured() {
        let user = create_user();
        let charts = user.get_featured().unwrap();
        assert_ne!(charts, vec![]);
    }

    #[test]
    fn it_should_fetch_trending() {
        let user = create_user();
        let charts = user.get_trending().unwrap();
        assert_ne!(charts, vec![]);
    }
}