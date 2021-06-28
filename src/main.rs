extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::thread::sleep;
use std::time::Duration;

use std::fs::File;
use std::io::Read; //this was the be all end all line that was missing

use serde::*;

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Response {
    code: i32,
    message: String,
    totalItems: i32,
    data: Vec<Anime>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Stats {
    peak: i32,
    //previously: i32, //there is a "-" so find out how to fix it
    stat: String,
    status: String, //consider making this an enum value
    weeksOnTop: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Anime {
    title: String,
    imageUrl: String,
    studio: String,
    stats: Stats,
}

#[derive(Serialize, Deserialize)]
struct Foo {
    name: String,
    age: u32,
}

fn main() {
    let delay = Duration::from_millis(300); //for fun

    let mut file = File::open("topanime.json").unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();

    let res: Response = serde_json::from_str(&buff).unwrap();
    // println!("{:#?}", res.data[3]);
    let data = res.data;
    for item in data.iter() {
        println!("{:?}", item);
        sleep(delay); //for fun
    }
}

//see if when i get the data I can sterilize it and change some of the variable names to use snake case
