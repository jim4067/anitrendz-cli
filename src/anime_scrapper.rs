use select::document::Document;
use select::node::Node;
use select::predicate::Predicate;
use select::predicate::{Class, Name};

#[derive(Debug)]
pub struct Header {
    pub season: String,
    pub week: String,
    pub date: String,
    pub top_anime: Vec<TopAnime>,
}

#[derive(Debug)]
pub struct TopAnime {
    pub title: String,
    pub studio: String,
    pub rank: i16,
    pub stats: Stats,
}

#[derive(Debug)]
pub struct Stats {
    pub status: String,
    pub stat: String,
    pub peak: String,
    pub prev: String,
    pub weeks_on_top: String,
}

impl Header {
    // pub async fn chart_details(html_source: &'static str) -> Header {
    //     let season = tokio::spawn(async move {
    //         Document::from(html_source)
    //             .find(Class("at-cth-top-season"))
    //             .next()
    //             .unwrap()
    //             .text()
    //             .trim()
    //             .to_string()
    //     });
    //     let week = tokio::spawn(async move {
    //         Document::from(html_source)
    //             .find(Class("at-cth-b-week-no"))
    //             .next()
    //             .unwrap()
    //             .text()
    //             .trim()
    //             .to_string()
    //     });
    //     let date = tokio::spawn(async move {
    //         Document::from(html_source)
    //             .find(Class("at-cth-b-date"))
    //             .next()
    //             .unwrap()
    //             .text()
    //             .trim()
    //             .to_string()
    //     });

    //     let top_anime = tokio::spawn(async move { TopAnime::get_anime_charts(html_source) });

    // let season = season.await.unwrap();
    // let week = week.await.unwrap();
    // let date = date.await.unwrap();
    // let top_anime = top_anime.await.unwrap();

    //     Self {
    //         season,
    //         week,
    //         date,
    //         top_anime,
    //     }
    // }

    pub fn chart_details(html_source: String) -> Header {
        let season = Document::from(html_source.as_str())
            .find(Class("at-cth-top-season"))
            .next()
            .unwrap()
            .text()
            .trim()
            .to_string();
        let week = Document::from(html_source.as_str())
            .find(Class("at-cth-b-week-no"))
            .next()
            .unwrap()
            .text()
            .trim()
            .to_string();

        let date = Document::from(html_source.as_str())
            .find(Class("at-cth-b-date"))
            .next()
            .unwrap()
            .text()
            .trim()
            .to_string();

        let top_anime = TopAnime::get_anime_charts(html_source.as_str());

        // let (season, week, date, top_anime) = join!(season, week, date, top_anime);

        Self {
            season,
            week,
            date,
            top_anime,
        }
    }
}

impl TopAnime {
    pub fn get_anime_charts(html_source: &str) -> Vec<TopAnime> {
        Document::from(html_source)
            .find(Class("at-mcc-entry")) //removed into iter in the next line
            .map(|node| TopAnime::anime(&node))
            .collect()
    }

    fn anime(node: &Node) -> TopAnime {
        let title = node
            .find(Class("entry-title"))
            .next()
            .unwrap()
            .text()
            .trim()
            .to_string();
        let studio = node
            .find(Class("entry-detail"))
            .next()
            .unwrap()
            .text()
            .trim()
            .to_string();
        let main_rank = node.find(Class("main-rank")).next().unwrap().text();
        let rank = &main_rank.trim_start()[0..3];
        let rank = rank.to_string().trim().parse().unwrap();

        let stats = Stats::stats(&node);

        Self {
            title,
            studio,
            rank,
            stats,
        }
    }
}

#[allow(unused_assignments)]
impl Stats {
    pub fn stats(node: &Node) -> Stats {
        let peak = node
            .find(Class("peak").child(Name("span")))
            .next()
            .unwrap()
            .text()
            .trim()
            .to_string();
        let prev = node
            .find(Class("prev").child(Name("span")))
            .next()
            .unwrap()
            .text()
            .trim()
            .to_string();
        let mut weeks_on_top = String::new();
        if let Some(weeks) = node.find(Class("weeks")).next() {
            weeks_on_top = weeks.text().trim().to_string();
        } else {
            weeks_on_top = "0".to_string()
        }

        let status_iter = node
            .find(Class("arrow-container").descendant(Name("img")))
            .map(|val| val.attr("alt").unwrap().to_string());
        let status = String::from_iter(status_iter);

        let stat = node
            .find(Class("arrow-number"))
            .next()
            .unwrap()
            .text()
            .trim()
            .to_string();

        Self {
            peak,
            prev,
            weeks_on_top,
            status,
            stat,
        }
    }
}

#[cfg(test)]
#[path = "./tests/anime_chart_test.rs"]
mod anime_chart_test;
