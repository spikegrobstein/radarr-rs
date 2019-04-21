// extern crate alamo_movies;
use alamo_movies::cinema::Cinema;

use std::error::Error;
use std::process;
use std::fmt::Debug;

use serde::Serialize;

mod radarr;

extern crate clap;
use clap::{Arg, App, SubCommand, ArgMatches};

// how this should work:
// iterate over every cinema from alamo
// filter out all movies of show-type terror tuesday, weird wednesday, video vortext
// search radarr API
// from those results, find best match. best match would be exact match on title or any
// "alternativeTitles"
// add any movies that are not already added.

fn main() {
    let app = App::new("radarr")
        .version("0.1.0")
        .author("Spike Grobstein <me@spike.cx>")
        .about("Commandline interface for the Radarr api")
        .arg(Arg::with_name("json")
             .help("Output everything in json")
             .long("json")
             .short("j")
             )
        .arg(Arg::with_name("hostname")
             .help("The hostname, with optional port, to connect (default: `localhost`)")
             .takes_value(true)
             .long("hostname")
             .short("H")
             )
        .arg(Arg::with_name("api-token")
             .help("API token to authenticate with Radarr")
             .takes_value(true)
             .long("api-token")
             .short("t")
             )
        .arg(Arg::with_name("protocol")
             .help("Protocol to use for the connection (http or https; default: `http`)")
             .takes_value(true)
             .long("protocol")
             .short("p")
             )
        .subcommand(SubCommand::with_name("status")
                    .about("Fetch the current server status")
                    )
        .subcommand(SubCommand::with_name("ping")
                    .about("Hit the server's `ping` endpoint")
                    )
        .subcommand(SubCommand::with_name("health")
                    .about("Fetch the server's current health information")
                    )
        .subcommand(SubCommand::with_name("search")
                    .about("Search for films given a search term")
                    .arg(Arg::with_name("term")
                         .help("The search query")
                         .required(true)
                         )
                    )
        .subcommand(SubCommand::with_name("list")
                    .about("List all movies that are currently tracked")
                    );

    if let Err(error) = run(app) {
        eprintln!("Error: {}", error);
        process::exit(1);
    }
}

fn create_client(matches: &ArgMatches) -> Result<radarr::Client, Box<dyn Error>> {
    let mut config = radarr::Config::new_from_env_with_defaults();
    let app_config = config_from_matches(matches);

    config.merge(app_config);

    radarr::Client::new(config)
}

fn config_from_matches(matches: &ArgMatches) -> radarr::Config {
    radarr::Config {
        api_token: matches.value_of("api-token").map(|v| String::from(v)),
        protocol: matches.value_of("protocol").map(|v| String::from(v)),
        hostname: matches.value_of("hostname").map(|v| String::from(v)),
    }
}

fn run(app: App) -> Result<(), Box<dyn Error>> {
    let matches = app.get_matches();

    let client = create_client(&matches)?;

    if let Some(_matches) = matches.subcommand_matches("status") {
        handle_resp(&matches, client.status()?)?;
    } else if let Some(_matches) = matches.subcommand_matches("ping") {
        handle_resp(&matches, client.ping()?)?;
    } else if let Some(_matches) = matches.subcommand_matches("health") {
        handle_resp(&matches, client.health()?)?;
    } else if let Some(matches) = matches.subcommand_matches("search") {
        let term = matches.value_of("term").unwrap();
        handle_resp(&matches, client.search(term)?)?;
    } else if let Some(matches) = matches.subcommand_matches("list") {
        handle_resp(&matches, client.list_movies()?)?;
    } else {
        panic!("Unreachable code.")
    }

    Ok(())
}

fn handle_resp<T: Debug + serde::Serialize>(matches: &ArgMatches, resp: radarr::Response<T>) -> Result<(), Box<dyn Error>> {
    if matches.is_present("json") {
        let json = serde_json::to_string(&resp.data)?;
        println!("{}", json);
    } else {
        println!("{:#?}", &resp.data);
    }

    // exit non-zero if there was any error
    if resp.api_response.status().is_server_error() {
        process::exit(1);
    } else if resp.api_response.status().is_client_error() {
        process::exit(2);
    }

    process::exit(0);
}

    // let config = radarr::Config::new_from_env_with_defaults();
    // let client = radarr::Client::new(config).unwrap();

    // let status = client.status().expect("Failed to connect");

    // eprintln!("Status: {:#?}", status);

    // let health = client.health().expect("Failed to get health");
    // eprintln!("Health: {:#?}", health);

    // let ping = client.ping().expect("Failed to ping");
    // eprintln!("Ping: {:#?}", ping);

    // let root_folders = client.root_folder().expect("Failed to get root folders");
    // eprintln!("Root folders: {:#?}", root_folders);

    // let movies = client.list_movies().expect("failed to list movies");
    // eprintln!("movies: {}", movies.len());

    // let movie = client.movie(63).expect("Failed to get alien");
    // eprintln!("movie: {:#?}", movie);

    // let good_movie_resp = match client.search("affinity") {
        // Ok(Some(movies)) => movies,
        // Ok(None) => panic!("No movie results."),
        // Err(error) => panic!("Error: {}", error),
    // };

    // eprintln!("TMDB ID: {}", good_movie_resp[0].tmdb_id);

    // if let Some(mut good_movie) = radarr::AddMoviePayload::from_movie_response(&good_movie_resp[0]) {
        // good_movie.set_monitored(true);
        // good_movie.set_root_folder_path("/storage/Movies");
        // client.add_movie(&good_movie).expect("Failed to add movie");
    // } else {
        // panic!("Failed to get good movie");
    // }

    // let cinema_id = Cinema::to_cinema_id("new-mission").unwrap();
    // let body = Cinema::get_calendar_data(&cinema_id).expect("expected thing");
    // let (_cinema, films) = Cinema::from_calendar_data(&body).expect("expected thing");

    // for film in films {
        // if &film.show_type != "Terror Tuesday" {
            // continue;
        // }

        // let title = &film.name;

        // match client.search(title) {
            // Ok(Some(results)) => {
                // let num_results = results.len();
                // println!("Got back {} results for {}", num_results, title);
                // if let Some(best) = best_matches(title, results) {
                    // println!("Best results {}/{} for {}", best.len(), num_results, title);
                // } else {
                    // eprintln!("Found no exact matches for {}", title); 
                // }
            // },
            // Ok(None) => eprintln!("Got no results for `{}`.", film.name),
            // Err(error) => panic!("Error: {}", error),
        // }
    // }
// }

// /// given the results from the radarr api, return an array of best matches
// /// best matches are exact matches or exact alternativeNames matches
// fn best_matches(term: &str, results: Vec<radarr::SearchResult>) -> Option<Vec<radarr::SearchResult>> {
    // let matches: Vec<radarr::SearchResult> = results.into_iter()
        // .filter(|result| {
            // result.title.to_lowercase() == term.to_lowercase() 
                // || result.alternative_titles.iter()
                    // .any(|title| title.to_lowercase() == term.to_lowercase())
        // })
        // .collect();

    // if matches.len() == 0 {
        // return None;
    // }

    // Some(matches)
// }

// // fn url_for(uri: &str, config: &RadarrConfig, query_string: &str) -> &str {
    // // &format!("{}://{}/api/{}?{}", config.protocol, config.hostname, uri, query_string)
// // }


// // fn root_folder(config: &Config) -> Result<RadarrRootFolder, Box<dyn Error>> {
    // // let query_string: String = form_urlencoded::Serializer::new(String::new())
        // // .append_pair("apikey", &config.api_token)
        // // .finish();

    // // let url: &str = url_for("rootfolder", config, query_string);
// // }
