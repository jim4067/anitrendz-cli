#![allow(non_snake_case, dead_code)]

use rand::Rng;

use crate::anime_net;
use crate::music_net;
use crate::utils::rainbow_println;

pub fn top_anime_list() {
    let res = anime_net::get_anime();
    let data = res.data;

    for (pos, anime) in data.iter().enumerate().rev() {
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
}

pub fn music_charts_list() {
    let res = music_net::get_music(); //if using result, i'd call unwrap here
    let data = res.data;

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

//generating random anime to watch
pub fn gen_random_anime() {
    let res = anime_net::get_anime();

    let num = rand::thread_rng().gen_range(0..res.totalItems) as usize;

    let anime = &res.data[num];
    rainbow_println("\n A random recommendation of anime to watch \n", 0.1, 3.0);
    rainbow_println(format!(" title: {}", anime.title).as_str(), 0.1, 3.0);
    rainbow_println(format!(" studio: {}", anime.studio).as_str(), 0.1, 3.0);
    rainbow_println(format!(" rank: {}", num + 1).as_str(), 0.1, 3.0);
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
}

pub fn gen_random_song() {
    let res = music_net::get_music();

    let num = rand::thread_rng().gen_range(0..res.totalItems) as usize;

    let song = &res.data[num];
    rainbow_println(
        "\n A random recommendation of a song to listen to \n",
        0.1,
        3.0,
    );

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
}

//this crate is responsible for printing to the cli
//anitrendz-cli uses anitop api for fearless concurrency on the terminal 
