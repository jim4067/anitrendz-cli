pub fn download_html_source(base_url: &str) -> String {
    let html_file = reqwest::blocking::get(base_url).unwrap();
    let text_html_file = html_file.text().unwrap();
    text_html_file
}

//Getting local source files
pub fn anime_charts_offline() -> String {
    include_str!("data/top_anime_source.html").to_string()
}

pub fn male_characters_offline() -> String {
    include_str!("data/male_characters.html").to_string()
}

pub fn female_characters_offline() -> String {
    include_str!("data/female_characters.html").to_string()
}

pub fn couple_ships_offline() -> String {
    include_str!("data/coupple.html").to_string()
}

//this crate is responsible for downloading the html source for the anitrendz.net or using a locally
//downloaded version if the network is not available
