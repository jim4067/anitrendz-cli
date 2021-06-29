use std::thread::sleep;
use std::time::Duration;
use crate::network;

pub fn top_anime_list(){
    // let res = network::get_anime();
    // let data = res.data;
    
    // let delay = Duration::from_millis(300); //for fun
    
    // for anime in data.iter() {
    //     println!("{:#?}", item);
    //     sleep(delay); //for fun
    // }

    //offline testing
    let res = network::offline_test();
    let data = res.data;

    for anime in data.iter().rev() {
        println!("{}", anime.title);
        println!("{}", anime.imageUrl);
        println!("{}", anime.studio);
        println!("{:?}", anime.stats);
        println!();
    }
}