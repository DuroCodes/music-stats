use std::error::Error;

use reqwest::Client;

use super::types::{TimePeriod, TopAlbums, UserInfo, BASE_URL};

pub struct LastFmUser {
    client: Client,
    api_key: String,
    user: String,
    limit: usize,
    pub period: TimePeriod,
}

impl LastFmUser {
    pub fn new(
        api_key: String,
        user: String,
        limit: usize,
        period: TimePeriod,
    ) -> Self {
        LastFmUser {
            client: Client::new(),
            api_key,
            user,
            limit,
            period,
        }
    }

    pub async fn get_top_albums(&self) -> Result<TopAlbums, Box<dyn Error>> {
        let url =
            format!(
            "{BASE_URL}?method=user.gettopalbums&api_key={}&user={}&limit={}&period={}&format=json",
            self.api_key, self.user, self.limit, self.period.api_value()
        );

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(format!("Failed to get top albums: HTTP {}", response.status()).into());
        }

        let top_albums = response.json::<TopAlbums>().await?;
        Ok(top_albums)
    }

    pub async fn get_info(&self) -> Result<UserInfo, Box<dyn Error>> {
        let url = format!(
            "{BASE_URL}?method=user.getinfo&api_key={}&user={}&format=json",
            self.api_key, self.user
        );

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(format!("Failed to get user info: HTTP {}", response.status()).into());
        }

        let user_info = response.json::<UserInfo>().await?;
        Ok(user_info)
    }
}
