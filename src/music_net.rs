#![allow(non_snake_case, dead_code)]

use std::fmt::{self, Display, Formatter};

use serde::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct TopMusic {
    pub code: i32,
    pub message: String,
    pub totalItems: i32,
    pub data: Vec<Music>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Music {
    pub title: String,
    pub artists: Vec<String>,
    pub rank: i32,
    pub stats: Stats,
    pub mediaUrl: MediaUrl,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stats {
    pub peak: i32,
    pub previously: StringOrInteger,
    pub weeks: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrInteger {
    Int(i32),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaUrl {
    pub youtube: Option<String>,
    pub spotify: Option<String>,
    pub itunes: Option<String>,
}
//listen on yt,sp,it

impl Display for StringOrInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(val) => write!(f, "{}", val),
            Self::String(val) => write!(f, "{}", val),
        }
    }
}

impl Display for Stats {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            self::Stats {
                peak,
                previously,
                weeks,
            } => write!(f, "{} {} {} ", peak, previously, weeks),
        }
    }
}

#[tokio::main]
pub async fn get_music() -> TopMusic {
    let request = reqwest::get("https://anitop.vercel.app/api/v1/music-chart")
        .await
        .unwrap();
    assert!(request.status().is_success());
    let result = request.json::<TopMusic>().await.unwrap();
    result
}

//this crate/file is responsible for getting the top music from anime shows using anitop api
//built using zero-cost abstractions
