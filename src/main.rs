extern crate clap;
extern crate rand;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate tokio;

mod anime_net;
mod app;
mod music_net;
mod utils;

use clap::{App, Arg};

fn main() {
    run();
}

fn run() {
    let cli = App::new("anitrendz-cli")
        .version("0.1-beta")
        .about("Get a list of trending anime, anime music charts and random recommendation right on your terminal.")
        .arg(
            Arg::with_name("anime")
                .short("a")
                .long("anime-list")
                .help("Print a list of top anime")
                // .takes_value(true),
        )
        .arg(
            Arg::with_name("music")
                .short("m")
                .long("music-charts")
                .help("get the list of the music charts"),
        )       
        .arg(
            Arg::with_name("anime-recommendation").long("anime-rec").help("Recommends a random anime"),
        )
        .arg(
            Arg::with_name("song-recommendation").short("s").long("song-rec").help("Recommends a random song"),
        )
        .get_matches();

    // I want the default one to be for listing anime
    if cli.is_present("anime") {
        app::top_anime_list();
    } else if cli.is_present("music") {
        app::music_charts_list();
    } else if cli.is_present("anime-recommendation") {
        app::gen_random_anime();
    } else if cli.is_present("song-recommendation") {
        app::gen_random_song();
    } else {
        app::top_anime_list();
    }
}


//in the future add a feature to search for anime using the Anilist api
//so I will do a POST request with reqwest