use std::fmt::{self, Display, Formatter};

use std::fs::File;
use std::io::Read; //this was the be all end all line that was missing

use serde::*;

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct TopAnime {
    pub code: i32,
    pub message: String,
    pub totalItems: i32,
    pub data: Vec<Anime>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Anime {
    pub title: String,
    pub imageUrl: String,
    pub studio: String,
    pub stats: Stats,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Stats {
    pub peak: i32,
    pub previously: StringOrInteger,
    pub stat: String,
    pub status: String, //consider making this an enum value
    pub weeksOnTop: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrInteger {
    Int(i32),
    String(String),
}

impl Display for StringOrInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(val) => write!(f, "{}", val),
            Self::String(val) => write!(f, "{}", val),
        }
    }
}

#[tokio::main]
pub async fn get_anime() -> TopAnime {
    let request = reqwest::get("https://anitop.vercel.app/api/v1/top-anime")
        .await
        .unwrap();
    let result = request.json::<TopAnime>().await.unwrap();
    result
}

pub fn offline_test() -> TopAnime {
    let mut file = File::open("topanime.json").unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();

    let res: TopAnime = serde_json::from_str(&buff).unwrap();
    res
}

//this crate/file is responsible for getting the top ranked anime from the anitop api