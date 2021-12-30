use crate::anime_scrapper;
use crate::source;

#[test]
//After the tenth entry, weeks_on_top for other entries is not empty. Instead, I hardcoded a string val of zero
//so making sure that it is there
fn weeks_on_top_not_empty() {
    // let html = source::anime_charts_offline();
    let base_url = "https://anitrendz.net/charts/male-characters/";
    let source = source::download_html_source(base_url);
    let anime_list = anime_scrapper::Header::chart_details(source);

    for val in anime_list.top_anime.iter() {
        assert!(!val.stats.weeks_on_top.is_empty());
    }
}
