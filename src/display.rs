use crate::anime_chart;
use crate::character_chart;
use crate::source;

pub async fn print_anime_charts() {
    let anime_list = anime_chart::Header::chart_details(source::anime_charts_offline()).await;

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
        "\n\t\t\t\t\t [ {} | {} | {} ]\n",
        anime_list.season, anime_list.week, anime_list.date
    );
}

pub async fn print_male_character_charts() {
    let character_list =
        character_chart::Header::chart_details(source::male_characters_offline()).await;

    println!(
        "\n\t\t\t [ {} | {} | {} | {} ]\n",
        character_list.title, character_list.season, character_list.week, character_list.date
    );

    for male_character in character_list.top_characters.iter().rev() {
        println!(" name: {}", male_character.name);
        println!(" show: {} ", male_character.show);
        println!(" rank: {} ", male_character.rank);
        println!(" stats for nerd : [ peak: {} | previously: {} | stat: {} | status: {} | weeks_on_top: {} ] \n", male_character.stats.peak, male_character.stats.prev,male_character.stats.stat, male_character.stats.status, male_character.stats.weeks_on_top );
    }

    println!(
        "\n\t\t\t [ {} | {} | {} | {} ]\n",
        character_list.title, character_list.season, character_list.week, character_list.date
    );
}

pub async fn print_female_character_charts() {
    let character_list =
        character_chart::Header::chart_details(source::female_characters_offline()).await;

    println!(
        "\n\t\t\t [ {} | {} | {} | {} ]\n",
        character_list.title, character_list.season, character_list.week, character_list.date
    );

    for female_character in character_list.top_characters.iter().rev() {
        println!(" name: {}", female_character.name);
        println!(" show: {} ", female_character.show);
        println!(" rank: {} ", female_character.rank);
        println!(" stats for nerd : [ peak: {} | previously: {} | stat: {} | status: {} | weeks_on_top: {} ] \n", female_character.stats.peak, female_character.stats.prev, female_character.stats.stat, female_character.stats.status, female_character.stats.weeks_on_top );
    }

    println!(
        "\n\t\t\t [ {} | {} | {} | {} ]\n",
        character_list.title, character_list.season, character_list.week, character_list.date
    );
}

pub async fn print_couple_ships() {
    let character_list =
        character_chart::Header::chart_details(source::couple_ships_offline()).await;

    println!(
        "\n\t\t\t [ {} | {} | {} | {} ]\n",
        character_list.title, character_list.season, character_list.week, character_list.date
    );

    for couple_ships in character_list.top_characters.iter().rev() {
        println!(" name: {}", couple_ships.name);
        println!(" show: {} ", couple_ships.show);
        println!(" rank: {} ", couple_ships.rank);
        println!(" stats for nerd : [ peak: {} | previously: {} | stat: {} | status: {} | weeks_on_top: {} ] \n", couple_ships.stats.peak, couple_ships.stats.prev, couple_ships.stats.stat, couple_ships.stats.status, couple_ships.stats.weeks_on_top );
    }

    println!(
        "\n\t\t\t [ {} | {} | {} | {} ]\n",
        character_list.title, character_list.season, character_list.week, character_list.date
    );
}

//this is teh crate that is responsible for displaying printing the results to the terminal
