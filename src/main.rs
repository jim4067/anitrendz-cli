extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate tokio;

mod network;
mod app;

fn main() {
    // println!("let us print the title of the top anime");
    // for anime in data.iter() {
    //     println!(" image {}", anime.imageUrl);
    //     println!(" peak {}", anime.stats.peak);
    //     println!(" previously {}", anime.stats.previously);
    //     println!(" stat {}", anime.stats.stat);
    //     println!(" status {}", anime.stats.status);
    // }
    app::top_anime_list();
}

//see if when i get the data I can sterilize it and change some of the variable names to use snake case
//you could add a rank field and learn how to calculate it