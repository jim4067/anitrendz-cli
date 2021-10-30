use futures::join;
use select::document::Document;
use select::node::Node;
use select::predicate::Predicate;
use select::predicate::{Class, Name};

#[derive(Debug)]
pub struct Header {
    pub title: String,
    pub season: String,
    pub week: String,
    pub date: String,
    pub characters: Vec<Characters>,
}

#[derive(Debug)]
pub struct Characters {
    pub name: String,
    pub show: String,
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
    pub async fn chart_details(html_source: String) -> Header {
        let title = async {
            Document::from(html_source.as_str())
                .find(Class("at-cth-top"))
                .next()
                .unwrap()
                .text()
                .trim()
                .split("\n")
                .next()
                .unwrap()
                .to_string()
        };
        let season = async {
            Document::from(html_source.as_str())
                .find(Class("at-cth-top-season"))
                .next()
                .unwrap()
                .text()
                .trim()
                .to_string()
        };
        let week = async {
            Document::from(html_source.as_str())
                .find(Class("at-cth-b-week-no"))
                .next()
                .unwrap()
                .text()
                .trim()
                .to_string()
        };
        let date = async {
            Document::from(html_source.as_str())
                .find(Class("at-cth-b-date"))
                .next()
                .unwrap()
                .text()
                .trim()
                .to_string()
        };

        let characters = async { Characters::get_character_charts(html_source.as_str()) };

        let (title, season, week, date, characters) = join!(title, season, week, date, characters);

        Self {
            season,
            title,
            week,
            date,
            characters,
        }
    }
}

impl Characters {
    pub fn get_character_charts(html_source: &str) -> Vec<Characters> {
        Document::from(html_source)
            .find(Class("at-mcc-entry")) //removed into iter in the next line
            .map(|node| Characters::anime(&node))
            .collect()
    }

    fn anime(node: &Node) -> Characters {
        let name = node
            .find(Class("entry-title"))
            .next()
            .unwrap()
            .text()
            .trim()
            .to_string();
        let show = node
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
            name,
            show,
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

//character charts i.e -> male characters, female characters and couple ship characters
