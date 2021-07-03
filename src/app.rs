#![allow(non_snake_case, dead_code)]

use crate::anime_net;
use crate::music_net;
use std::f64::consts::PI;

fn rgb(freq: f64, spread: f64, i: f64) -> (u8, u8, u8) {
    let j = i / spread;
    let red = (freq * j + 0.0).sin() * 127.0 + 128.0;
    let green = (freq * j + 2.0 * PI / 3.0).sin() * 127.0 + 128.0;
    let blue = (freq * j + 4.0 * PI / 3.0).sin() * 127.0 + 128.0;

    (red as u8, green as u8, blue as u8)
}

fn rainbow_println(line: &str, frequency: f64, spread: f64) {
    for (i, c) in line.char_indices() {
        let (r, g, b) = rgb(frequency, spread, i as f64);
        print!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, c);
    }
    println!();
}

pub fn top_anime_list() {
    let res = anime_net::get_anime();
    let data = res.data;

    for (pos, anime) in data.iter().enumerate().rev() {
        // println!("title: {}", anime.title);
        // println!("image{}", anime.imageUrl);
        // println!("studio: {}", anime.studio);
        // println!("rank: {}", pos + 1);
        // println!("{:?}", anime.stats);
        // println!();
        rainbow_println(format!(" title: {}", anime.title).as_str(), 0.1, 3.0);
        rainbow_println(format!(" studio: {}", anime.studio).as_str(), 0.1, 3.0);
        rainbow_println(format!(" rank: {}", pos + 1).as_str(), 0.1, 3.0);
        rainbow_println(
            format!(
                " stats for nerds : [ peak: {} | previously: {} | stat: {} | status: {} | weeks_on_top: {} ] ",
                anime.stats.peak,
                anime.stats.previously,
                anime.stats.stat,
                anime.stats.status,
                anime.stats.weeksOnTop
            )
            .as_str(),
            0.1,
            3.0,
        );
        println!();
    }

    // offline testing
    // let res = anime_net::offline_test();
    // let data = res.data;

    // for (pos, anime) in data.iter().enumerate().rev() {
    //     rainbow_println(format!(" title: {}", anime.title).as_str(), 0.1, 3.0);
    //     rainbow_println(format!(" studio: {}", anime.studio).as_str(), 0.1, 3.0);
    //     rainbow_println(format!(" rank: {}", pos + 1).as_str(), 0.1, 3.0);
    //     rainbow_println(format!(" {:?}", anime.stats).as_str(), 0.1, 3.0); //stats for nerds. impl Display and use a bunch of prints
    //     println!();
    // }
}

pub fn top_anime_list_uncolored() {
    let res = anime_net::get_anime();
    let data = res.data;

    for (pos, anime) in data.iter().enumerate().rev() {
        println!(" title: {}", anime.title);
        println!(" studio: {}", anime.studio);
        println!(" rank: {}", pos + 1);
        println!(
            " stats for nerds : [ peak: {} | previously: {} | stat: {} | status: {} | weeks_on_top: {} ] ",
            anime.stats.peak,
            anime.stats.previously,
            anime.stats.stat,
            anime.stats.status,
            anime.stats.weeksOnTop
        );
        println!();
    }
}

pub fn music_charts_list() {
    let res = music_net::get_music(); //if using result, i'd call result here
    let data = res.data;

    // println!("{:#?}", data);
    for song in data.iter().rev() {
        rainbow_println(format!(" title: {}", song.title).as_str(), 0.1, 3.0);
        rainbow_println(format!(" artists: {:?}", song.artists).as_str(), 0.1, 3.0);
        rainbow_println(format!(" rank: {}", song.rank).as_str(), 0.1, 3.0);
        rainbow_println(
            format!(
                " stats for nerds :: [ peak: {} | previously: {} | weeks_on_top: {} ]",
                song.stats.peak, song.stats.previously, song.stats.weeks
            )
            .as_str(),
            0.1,
            3.0,
        );

        //printing where you can listen to the songs and handling the NONE values
        rainbow_println(
            format!(
                " listen: youtube -> {}",
                song.mediaUrl
                    .youtube
                    .as_ref()
                    .unwrap_or(&"404 Not available".to_string())
            )
            .as_str(),
            0.1,
            3.0,
        );
        rainbow_println(
            format!(
                " \t spotify -> {}",
                song.mediaUrl
                    .spotify
                    .as_ref()
                    .unwrap_or(&"404 Not available".to_string())
            )
            .as_str(),
            0.1,
            3.0,
        );
        rainbow_println(
            format!(
                "\t itunes -> {}",
                song.mediaUrl
                    .itunes
                    .as_ref()
                    .unwrap_or(&"404 Not available".to_string())
            )
            .as_str(),
            0.1,
            3.0,
        );

        // match song.mediaUrl.youtube.as_ref() {
        //     Some(val) => rainbow_println(format!(" listen: youtube -> {}", val).as_str(), 0.1, 3.0),
        //     None => rainbow_println(" listen: youtube -> not found on youtube", 0.1, 3.0),
        // }
        println!()
    }
}

pub fn music_charts_list_uncolored() {
    let res = music_net::get_music();
    let data = res.data;

    for song in data.iter().rev() {
        println!(" title: {}", song.title);
        println!(" studio: {:?}", song.artists);
        println!(" rank: {}", song.rank);
        println!(
            " stats for nerds :: [ peak: {} | previously: {} | weeks_on_top: {} ]",
            song.stats.peak, song.stats.previously, song.stats.weeks
        );
        println!(
            " listen: youtube -> {}",
            song.mediaUrl
                .youtube
                .as_ref()
                .unwrap_or(&"404 Not Available".to_string())
        );
        println!(
            " \t spotify -> {}",
            song.mediaUrl
                .spotify
                .as_ref()
                .unwrap_or(&"404 Not Available".to_string())
        );
        println!(
            " \t itunes  -> {}",
            song.mediaUrl
                .itunes
                .as_ref()
                .unwrap_or(&"404 Not Available".to_string())
        );
        println!();
    }
}
//for the cli application. only this will be required in main
fn run() {}

//this crate is responsible for printing to the cli
//anitrendz-cli anitop api but with fearless concurrency on the terminal

//do not forget that you were to add the option to search for anime using the anilist free api
//add cli option to either rainbow print or print basic. i,e green -> basic print

//how many items do you want to be displayed

// -a --anime -for listing the anime. subcommand to either rainbow print or basic print
// -m --music-charts for listing the music charts. subcommand to either rainbow print or...
// randomly recommend anime to user
// randomly recommend song to user

//what I have learned
// using serde.js to deserialize and serialize json data
//How to format and display custom types
//ANSI excape code for the rainbow print
//learn how to make a .deb package
