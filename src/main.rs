// use std::time::Instant;

mod anime_scrapper;
mod cli;
mod display;
mod generic_scrapper;
mod source;

fn main() {
    //DO BENCHMARKING DOWN HERE BELOW
    // let before = Instant::now();
    cli::cli_app().unwrap();

    // println!("Elapsed time: {:.2?}", before.elapsed());
}
