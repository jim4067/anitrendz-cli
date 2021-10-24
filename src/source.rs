// pub fn online(){}
pub fn download_anime_charts(){}

pub fn anime_charts_offline() -> &'static str {
    include_str!("source.html")
}


//this crate is responsible for downloading the html source for the anitrendz.net or using a locally 
//downloaded version if the network is not available