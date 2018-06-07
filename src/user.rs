use failure::Error;
use reqwest::{Client, header};
use podcast::PocketcastPodcast;

const LOGIN_URI: &str = "https://play.pocketcasts.com/users/sign_in";
const GET_SUBSCRIPTIONS_URI: &str = "https://play.pocketcasts.com/web/podcasts/all.json";
const GET_TOP_CHARTS_URI: &str = "https://static.pocketcasts.com/discover/json/popular_world.json";
const GET_FEATURED_URI: &str = "https://static.pocketcasts.com/discover/json/featured.json";
const GET_TRENDING_URI: &str = "https://static.pocketcasts.com/discover/json/trending.json";

#[derive(Debug, Deserialize, Clone)]
pub struct PocketcastUser {
    email: String,
    password: String,
    pub session: Option<String>
}


impl PocketcastUser {
    pub fn login(&mut self) -> Result<(), Error> {
        let body = [
            ("[user]email", self.email.as_str()),
            ("[user]password", self.password.as_str())
        ];

        let client = Client::new();
        let res = client.post(LOGIN_URI)
            .form(&body)
            .send()?;

        let _cookies = res.headers().get::<header::SetCookie>().unwrap();

        Ok(())
    }

    pub fn get_subscriptions(&self) -> Result<Vec<PocketcastPodcast>, Error> {
        let client = Client::new();
        let session = self.session.clone().expect("Login first");
        let mut cookies = header::Cookie::new();
        cookies.set("_social_session", session);
        let mut res = client.post(GET_SUBSCRIPTIONS_URI)
            .header(cookies)
            .send()?;

        if !res.status().is_success() {
            return Ok(vec![]); // todo: error
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
        let session = self.session.clone().expect("Login first");
        let mut cookies = header::Cookie::new();
        cookies.set("_social_session", session);
        let mut res = client
            .get(uri)
            .header(cookies)
            .send()?;

        if !res.status().is_success() {
            return Ok(vec![]); // todo: error
        }

        let res: DiscoverResponse = res.json()?;

        Ok(res.result.unwrap().podcasts)
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
