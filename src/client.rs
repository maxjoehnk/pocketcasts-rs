use crate::episode::Episode;
use crate::error::PocketcastError;
use failure::Error;
use crate::podcast::{DiscoverPodcast, Podcast};
use reqwest::{Client, Response, StatusCode, ClientBuilder, redirect::Policy};
use crate::responses::*;
use crate::api;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct PocketcastClient {
    token: String
}

#[derive(Debug, Clone, Deserialize)]
struct LoginResponseBody {
    token: String,
    uuid: String
}

#[derive(Debug, Clone, Serialize)]
struct LoginRequest {
    email: String,
    password: String,
    scope: LoginScope
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
enum LoginScope {
    Webplayer
}

#[derive(Debug, Clone, Serialize)]
struct Version {
    #[serde(rename = "v")]
    version: usize
}

#[derive(Debug, Clone, Serialize)]
struct SearchRequest {
    term: String
}

impl PocketcastClient {
    pub fn new(token: String) -> PocketcastClient {
        PocketcastClient {
            token
        }
    }

    pub async fn login(email: String, password: String) -> Result<Self, Error> {
        let body = LoginRequest {
            email,
            password,
            scope: LoginScope::Webplayer
        };
        let client = Client::new();

        let res = client.post(api::LOGIN_URI)
            .json(&body)
            .send()
            .await?;

        if res.status() == StatusCode::UNAUTHORIZED {
            Err(Error::from(PocketcastError::InvalidCredentials))?;
        }

        if !res.status().is_success() {
            Err(Error::from(PocketcastError::HttpStatusError(res.status())))?;
        }

        let res: LoginResponseBody = res.json().await?;

        Ok(PocketcastClient {
            token: res.token
        })
    }

    pub async fn get_subscriptions(&self) -> Result<Vec<Podcast>, Error> {
        let body = Version { version: 1 };
        let res = self.post(api::GET_SUBSCRIPTIONS_URI, Some(body)).await?;

        if !res.status().is_success() {
            return Err(Error::from(PocketcastError::HttpStatusError(res.status())));
        }

        let res: SubscriptionsResponse = res.json().await?;

        Ok(res.podcasts)
    }

    pub async fn get_episodes(&self, podcast_id: &str) -> Result<Vec<Episode>, Error> {
        let url = format!("{}/{}/{}.json", api::GET_EPISODES_URI_PREFIX, podcast_id, api::GET_EPISODES_URI_SUFFIX);
        let res = self.get(&url, None).await?;

        if !res.status().is_success() {
            return Err(Error::from(PocketcastError::HttpStatusError(res.status())));
        }

        let res: EpisodesResponse = res.json().await?;

        Ok(res.podcast.episodes)
    }

    pub async fn search_podcasts<Q: Into<String>>(&self, query: Q) -> Result<Vec<SearchPodcast>, Error> {
        let body = SearchRequest { term: query.into() };
        let res = self.post(api::SEARCH_PODCASTS_URI, Some(body)).await?;

        if !res.status().is_success() {
            return Err(Error::from(PocketcastError::HttpStatusError(res.status())));
        }

        let res: SearchResponse = res.json().await?;

        Ok(res.podcasts)
    }

    pub async fn get_top_charts() -> Result<Vec<DiscoverPodcast>, Error> {
        PocketcastClient::get_discover(api::GET_TOP_CHARTS_URI).await
    }

    pub async fn get_featured() -> Result<Vec<DiscoverPodcast>, Error> {
        PocketcastClient::get_discover(api::GET_FEATURED_URI).await
    }

    pub async fn get_trending() -> Result<Vec<DiscoverPodcast>, Error> {
        PocketcastClient::get_discover(api::GET_TRENDING_URI).await
    }

    async fn post<TReq>(&self, url: &'static str, body: Option<TReq>) -> Result<Response, Error>
        where TReq: Serialize {
        let client = Client::new();

        let request_builder = client.post(url).bearer_auth(&self.token);

        let res = match body {
            Some(json) => request_builder
                .json(&json)
                .send(),
            None => request_builder.send()
        };
        Ok(res.await?)
    }

    async fn get(&self, url: &str, query: Option<Vec<(String, String)>>) -> Result<Response, Error> {
        let client = Client::new();

        let request_builder = client.get(url).bearer_auth(&self.token);

        let res = match query {
            Some(json) => request_builder
                .query(&json)
                .send(),
            None => request_builder
                .send()
        };
        Ok(res.await?)
    }

    async fn get_discover(uri: &'static str) -> Result<Vec<DiscoverPodcast>, Error> {
        let client = Client::new();
        let res = client
            .get(uri)
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(Error::from(PocketcastError::HttpStatusError(res.status())));
        }

        let res: DiscoverResponse = res.json().await?;

        Ok(res.result.ok_or(PocketcastError::EmptyResponse).map(|result| result.podcasts)?)
    }
}

#[cfg(test)]
mod tests {
    use failure::Error;
    use super::*;

    async fn login() -> Result<PocketcastClient, Error> {
        PocketcastClient::login(env!("POCKETCAST_EMAIL").to_string(), env!("POCKETCAST_PASSWORD").to_string()).await
    }

    #[tokio::test]
    async fn it_logs_in() {
        let client = login().await;
        assert_eq!(client.is_ok(), true);
    }

    #[tokio::test]
    async fn it_should_fetch_subscriptions() {
        let client = login().await.unwrap();
        let subscriptions = client.get_subscriptions().await.unwrap();
        assert_ne!(subscriptions, vec![]);
    }

    #[tokio::test]
    async fn it_should_fetch_top_charts() {
        let charts = PocketcastClient::get_top_charts().await.unwrap();
        assert_ne!(charts, vec![]);
    }

    #[tokio::test]
    async fn it_should_fetch_featured() {
        let charts = PocketcastClient::get_featured().await.unwrap();
        assert_ne!(charts, vec![]);
    }

    #[tokio::test]
    async fn it_should_fetch_trending() {
        let charts = PocketcastClient::get_trending().await.unwrap();
        assert_ne!(charts, vec![]);
    }

    #[tokio::test]
    async fn it_should_fetch_podcast_episodes() {
        let client = login().await.unwrap();
        let episodes = client.get_episodes("c55316c0-d9ab-0136-3249-08b04944ede4").await.unwrap();
        assert_ne!(episodes, vec![]);
    }

    #[tokio::test]
    async fn it_should_search_for_podcasts() {
        let client = login().await.unwrap();
        let podcasts = client.search_podcasts("Film Riot").await.unwrap();
        assert_ne!(podcasts, vec![]);
    }
}
