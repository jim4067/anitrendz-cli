use crate::display;
use clap::{App, Arg};
use std::error::Error;

pub fn cli_app() -> Result<(), Box<dyn Error>> {
    let cli = App::new("anitrendz-cli")
        .version("0.0.1")
        .about("Get the anitrendz.net charts on your terminal")
        .arg(
            Arg::with_name("anime charts")
                .short("a")
                .long("anime-charts")
                .help("default, lists anime charts from anitrendz.net"),
        )
        .arg(
            Arg::with_name("male character charts")
                .short("m")
                .long("male-characters")
                .help("lists, male characters from anitrendz"),
        )
        .arg(
            Arg::with_name("female character charts")
                .short("f")
                .long("female-characters")
                .help("lists, female characters from anitrendz"),
        )
        .arg(
            Arg::with_name("couple ships charts")
                .short("c")
                .long("couple-ships")
                .help("lists, couple ships characters from anitrendz"),
        )
        .arg(
            Arg::with_name("opening song charts")
                .short("o")
                .long("op-songs")
                .help("print the top anime opening songs"),
        )
        .arg(
            Arg::with_name("ending song charts")
                .short("e")
                .long("ed-songs")
                .help("print the top anime ending songs"),
        )
        .get_matches();

    if cli.is_present("anime charts") {
        display::print_anime_charts();
    } else if cli.is_present("male character charts") {
        display::print_male_character_charts();
    } else if cli.is_present("female character charts") {
        display::print_female_character_charts();
    } else if cli.is_present("couple ships charts") {
        display::print_couple_ships();
    } else if cli.is_present("opening song charts") {
        display::print_opening_songs();
    } else if cli.is_present("ending song charts") {
        display::print_ending_songs();
    } else {
        display::print_anime_charts();
    }
    Ok(())
}

//this crate is responsible for parsing cli input
