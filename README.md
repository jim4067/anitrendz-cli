![anitrendz-cli](https://socialify.git.ci/jim4067/anitrendz-cli/image?description=1&issues=1&language=1&owner=1&stargazers=1&theme=Dark)

## Note

Rewrote this cli app since the [anitiop api](https://anitop.vercel.app/) api broke. The code for the previous app can be found under the [v1-anitop](https://github.com/jim4067/anitrendz-cli/tree/v1-anitop) branch.

## project setup	
<strike> Download the binary package from the releases page.</strike> As mentioned v1 ued the anitop api and since it isn't online using it won't work. To use the new version, clone the repo and `cargo run` the project.

## Description

Anitrendz is a cli app that uses data scraped from the [anitrendz website](https://anitrendz.net) to list the top anime, ships, characters and songs.

> I made this project to strengthen my knowledge on rust and learn how to handle JSON data but anc continually improve on it.

<sub>I am in the process of writing an article on how to scrape websites with Rust and will publish it very soon</sub>

## Tech stack 💻

This is a [rust](https://www.rust-lang.org/) project and I am using the following crates.
- [clap](https://clap.rs/) for parsing command line arguments.
- [serde](https://serde.rs/) for serializing and deserializing the JSON data.
- [reqwest](https://docs.rs/reqwest/0.11.4/reqwest/) for making HTTP requests to the anitop api.
- [select](https://github.com/utkarshkukreti/select.rs) for scraping the anitrendz website.

## Source data 📝

For the data, i used the anitrendz.net website

-   [anitrendz website](https://anitrendz.net/)

## What I learned?

-   fetching data using reqwest and consuming api's in rust.
-   using serde.js to deserialize and serialize json data.
-   How to format custom types and display them using Display.
-   ANSI escape codes and especially Select Graphics Rendition(SGR) for the colored output
-   making a command line app using clap.rs

## Project setup

Download the binary package from the releases page.

Jimmy ©2021 - present
