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
                .long("male-characters") //.takes_value(true)
                .help("lists, male characters from anitrendz"), // .takes_value(true),
        )
        .arg(
            Arg::with_name("female character charts")
                .short("f")
                .long("female-characters")
                // .takes_value(true)
                .help("Play the songs in a specified directory"),
        )
        .arg(
            Arg::with_name("couple ships charts")
                .short("c")
                .long("couple-ships")
                // .takes_value(true)
                .help("Play the songs in a specified directory"),
        )
        .get_matches();

    //getting the value for play from

    if cli.is_present("anime charts") {
        display::print_anime_charts();
    } else if cli.is_present("male character charts") {
        // let input = cli.value_of("play-from").unwrap();
        // playback::play_from(input);
        display::print_male_character_charts();
    } else if cli.is_present("female character charts") {
        display::print_female_character_charts();
    } else if cli.is_present("couple ships charts") {
        display::print_couple_ships();  
    } else {
        display::print_anime_charts();
    }
    Ok(())
}

//show which season we are in eg fall and such
//give the user the ability to choose how many entries he wants eg, 10, 20, 50, 100 etc, or instead of
//having hard coded values you can have the user enter a number and pass that number to the take() function

//this crate is responsible for parsing cli input
