use select::document::Document;

use select::predicate::{Class};

#[derive(Debug)]
pub struct Header {
    pub season: String,
    pub week: String,
    pub date: String, 
}

pub struct TopAnime {
    pub title: String,
    pub studio: String,
    pub rank: i16,
    // pub stats: Stats,
}

pub struct Stats {
    pub peak: i16,
    pub previously: i16,
    pub status: String,
    pub stat: String,
    pub weeks_on_top: i16,
}

impl Header {
    // pub fn chart_details(html_source: &str) /* -> Vec<Header>*/
    // {
    //     // let details: Vec<Header> = Document::from(html_source)
    //     //     .find(Name("at-chart-top-header")).into_iter()
    //     //     .map(|node| {
    //     //         let season = node.find(Class("at-cth-top-season")).next().unwrap().text();
    //     //         let week = node.find(Class("at-cth-b-week-no")).next().unwrap().text();
    //     //         let date = node.find(Class("at-cth-b-date")).next().unwrap().text();
    //     //         Header { season, week, date }
    //     //     })
    //     //     .collect();
    //     //     println!("details {:?}", details);

    //     let season = Document::from(html_source).find(Class("at-cth-top-season")).next().unwrap().text();
    //     println!("season: {}", season.trim());
    //     let week = Document::from(html_source).find(Class("at-cth-b-week-no")).next().unwrap().text();
    //     println!("week: {}", week.trim());
    //     let date = Document::from(html_source).find(Class("at-cth-b-date")).next().unwrap().text();
    //     println!("date: {}", date.trim());

    //     // let details = tokio::spawn

    //     // Self { season }
    // }

    pub async fn chart_details(html_source: &'static str) {
        let season = tokio::spawn(async move {
            Document::from(html_source).find(Class("at-cth-top-season")).next().unwrap().text()
        });
        let week = tokio::spawn(async move {
            Document::from(html_source).find(Class("at-cth-b-week-no")).next().unwrap().text()
        });
        let date = tokio::spawn(async move {
            Document::from(html_source).find(Class("at-cth-b-date")).next().unwrap().text()
        });

        let handles = vec![season, week, date];
        for handle in handles{
            let val = handle.await.unwrap();
            println!("{}", val.trim());
        }
    }
}

impl TopAnime {
    // fn get_anime_charts(html_source: &str) -> Vec<TopAnime> {
    //     // Document::from(html_source).find(Name("at-mcc-entry  "))
    // }

    // fn anime(node: &Node) -> TopAnime {
    // }
}
