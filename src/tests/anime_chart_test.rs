use crate::anime_chart;
use crate::source;

#[test]
//After the tenth entry, weeks_on_top for other entries is not empty. Instead, I hardcoded a string val of zero
//so making sure that it is there
fn weeks_on_top_not_empty() {
    let html = source::anime_charts_offline();
    for val in anime_chart::TopAnime::get_anime_charts(html){
        assert!(!val.stats.weeks_on_top.is_empty())
    }
}