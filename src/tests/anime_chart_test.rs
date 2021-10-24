use crate::anime_chart;
use crate::net;

#[test]
//After the tenth entry, weeks_on_top for other entries is not empty. Instead, I hardcoded a string val of zero
//so making sure that it is there
fn weeks_on_top_not_empty() {
    let html = net::offline();
    for val in anime_chart::TopAnime::get_anime_charts(html){
        assert!(!val.stats.weeks_on_top.is_empty())
    }
}