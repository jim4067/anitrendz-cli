#![allow(dead_code)]

use std::f64::consts::PI;

use crate::anime_net;

fn rgb(freq: f64, spread: f64, i: f64) -> (u8, u8, u8) {
    let j = i / spread;
    let red = (freq * j + 0.0).sin() * 127.0 + 128.0;
    let green = (freq * j + 2.0 * PI / 3.0).sin() * 127.0 + 128.0;
    let blue = (freq * j + 4.0 * PI / 3.0).sin() * 127.0 + 128.0;

    (red as u8, green as u8, blue as u8)
}

pub fn rainbow_println(line: &str, frequency: f64, spread: f64) {
    for (i, c) in line.char_indices() {
        let (r, g, b) = rgb(frequency, spread, i as f64);
        print!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, c);
    }
    println!();
}

//I was using this for testing
pub fn top_anime_offline() {
    let res = anime_net::offline_test();
    let data = res.data;

    for (pos, anime) in data.iter().enumerate().rev() {
        rainbow_println(format!(" title: {}", anime.title).as_str(), 0.1, 3.0);
        rainbow_println(format!(" studio: {}", anime.studio).as_str(), 0.1, 3.0);
        rainbow_println(format!(" rank: {}", pos + 1).as_str(), 0.1, 3.0);
        rainbow_println(format!(" {:?}", anime.stats).as_str(), 0.1, 3.0); //stats for nerds. impl Display and use a bunch of prints
        println!();
    }
}
