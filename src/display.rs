#![allow(dead_code)]
use crate::anime_scrapper;
use crate::generic_scrapper;
use crate::source;

pub fn print_anime_charts() {
    let base_url = "https://anitrendz.net/charts/top-anime/";
    let source = source::download_html_source(base_url);
    let anime_list = anime_scrapper::Header::chart_details(source);

    println!(
        "\n\t\t\t\t\t [ {} | {} | {} ]\n",
        anime_list.season, anime_list.week, anime_list.date
    );

    for anime in anime_list.top_anime.iter().rev() {
        println!(" title: {}", anime.title);
        println!(" rank: {}", anime.rank);
        println!(" studio: {}", anime.studio);
        println!(" stats for nerd : [ peak: {} | previously: {} | stat: {} | status: {} | weeks_on_top: {} ] \n", anime.stats.peak, anime.stats.prev,anime.stats.stat, anime.stats.status, anime.stats.weeks_on_top );
    }
    println!(
        "\n\t\t\t\t [ {} | {} | {} ]\n",
        anime_list.season, anime_list.week, anime_list.date
    );
}

fn generic_print_function(base_url: &str) {
    let source = source::download_html_source(base_url);
    let entry_list = generic_scrapper::Header::chart_details(source);

    println!(
        "\n\t\t\t [ {} | {} | {} | {} ]\n",
        entry_list.title, entry_list.season, entry_list.week, entry_list.date
    );

    for entry in entry_list.chart_entry.iter().rev() {
        println!(" name: {}", entry.name);
        println!(" show: {} ", entry.show);
        println!(" rank: {} ", entry.rank);
        println!(" stats for nerd : [ peak: {} | previously: {} | stat: {} | status: {} | weeks_on_top: {} ] \n", entry.stats.peak, entry.stats.prev,entry.stats.stat, entry.stats.status, entry.stats.weeks_on_top );
    }

    println!(
        "\n\t\t\t [ {} | {} | {} | {} ]\n",
        entry_list.title, entry_list.season, entry_list.week, entry_list.date
    );
}

pub fn print_male_character_charts() {
    let base_url = "https://anitrendz.net/charts/male-characters/";
    generic_print_function(base_url);
}

pub fn print_female_character_charts() {
    let base_url = "https://anitrendz.net/charts/female-characters/";
    generic_print_function(base_url);
}

pub fn print_couple_ships() {
    let base_url = "https://anitrendz.net/charts/couple-ship/";
    generic_print_function(base_url);
}

pub fn print_ending_songs() {
    let base_url = "https://anitrendz.net/charts/ed-theme-songs/";
    generic_print_function(base_url);
}

pub fn print_opening_songs() {
    let base_url = "https://anitrendz.net/charts/op-theme-songs/";
    generic_print_function(base_url);
}

// this is teh crate that is responsible for displaying printing the results to the terminal
