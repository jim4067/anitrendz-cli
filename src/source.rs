//getting anime charts(){}
pub fn download_anime_charts() {}

pub fn anime_charts_offline() -> &'static str {
    include_str!("top_anime_source.html")
}

pub fn male_characters_offline() -> &'static str {
    include_str!("male_characters.html")
}

pub fn female_characters_offline() -> &'static str {
    include_str!("female_characters.html")
}

pub fn couple_ships_offline() -> &'static str {
    include_str!("coupple.html")
}

//this crate is responsible for downloading the html source for the anitrendz.net or using a locally
//downloaded version if the network is not available
