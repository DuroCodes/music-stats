use serde::Deserialize;

pub const BASE_URL: &str = "http://ws.audioscrobbler.com/2.0/";

pub enum TimePeriod {
    Week,
    Month,
    Quarter,
    Half,
    Year,
    Overall,
}

impl TimePeriod {
    pub fn display(&self) -> &str {
        match self {
            TimePeriod::Overall => "Overall",
            TimePeriod::Week => "Last 7 days",
            TimePeriod::Month => "Last 30 days",
            TimePeriod::Quarter => "Last 3 months",
            TimePeriod::Half => "Last 6 months",
            TimePeriod::Year => "Last year",
        }
    }

    pub fn api_value(&self) -> &str {
        match self {
            TimePeriod::Overall => "overall",
            TimePeriod::Week => "7day",
            TimePeriod::Month => "1month",
            TimePeriod::Quarter => "3month",
            TimePeriod::Half => "6month",
            TimePeriod::Year => "12month",
        }
    }
}

impl ToString for TimePeriod {
    fn to_string(&self) -> String {
        match self {
            TimePeriod::Overall => "Overall".to_string(),
            TimePeriod::Week => "7 Days".to_string(),
            TimePeriod::Month => "1 Month".to_string(),
            TimePeriod::Quarter => "3 Months".to_string(),
            TimePeriod::Half => "6 Months".to_string(),
            TimePeriod::Year => "12 Months".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Image {
    pub size: String,
    #[serde(rename = "#text")]
    pub url: String,
}

pub fn get_image<'a>(images: &'a Vec<Image>, size: &'a str) -> Option<&'a Image> {
    images.iter().find(|img| img.size == size)
}

#[derive(Debug, Deserialize)]
pub struct Artist {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct TopAlbums {
    pub topalbums: TopAlbumsData,
}

#[derive(Debug, Deserialize)]
pub struct TopAlbumsData {
    pub album: Vec<Album>,
}

#[derive(Debug, Deserialize)]
pub struct Album {
    pub name: String,
    pub image: Vec<Image>,
    pub artist: Artist,
}

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    pub user: User,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub name: String,
    pub image: Vec<Image>,
}
