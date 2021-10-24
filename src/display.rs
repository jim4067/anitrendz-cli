use crate::anime_chart;
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

//this is teh crate that is responsible for displaying printing the results to the terminal
