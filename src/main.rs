use std::error::Error;
use std::process;
use std::fmt::Debug;

use serde_json::json;

mod radarr;

extern crate clap;
use clap::{Arg, App, SubCommand, ArgMatches};

enum DataSource {
    Stdin,
    File(String),
    Data(String),
}

// how this should work:
// iterate over every cinema from alamo
// filter out all movies of show-type terror tuesday, weird wednesday, video vortext
// search radarr API
// from those results, find best match. best match would be exact match on title or any
// "alternativeTitles"
// add any movies that are not already added.

fn main() {
    let app = App::new("radarr")
        .version(clap::crate_version!())
        .author("Spike Grobstein <me@spike.cx>")
        .about(clap::crate_description!())
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
                    )
        .subcommand(SubCommand::with_name("show")
                    .about("Show the movie with the given ID")
                    .arg(Arg::with_name("movie_id")
                         .help("The ID of the movie to show")
                         .required(true)
                         )
                    )
        .subcommand(SubCommand::with_name("add")
                    .about("Add the given movie")
                    .arg(Arg::with_name("file")
                         .help("Path to file containing search result json")
                         .long("file")
                         .short("f")
                         .takes_value(true)
                         )
                    .arg(Arg::with_name("data")
                         .help("Raw JSON data of search result")
                         .long("data")
                         .short("d")
                         .takes_value(true)
                         )
                    )
        .subcommand(SubCommand::with_name("delete")
                    .about("Delete the movie with the given ID")
                    .arg(Arg::with_name("movie_id")
                         .help("The ID of the movie to delete")
                         .required(true)
                         )
                    .arg(Arg::with_name("delete_files")
                         .help("Also delete the files on disk associated with this movie")
                         .required(false)
                         .long("delete-files")
                         .short("d")
                         )
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
    } else if let Some(search_matches) = matches.subcommand_matches("search") {
        let term = search_matches.value_of("term").unwrap();
        handle_resp(&matches, client.search(term)?)?;
    } else if let Some(_matches) = matches.subcommand_matches("list") {
        handle_resp(&matches, client.list_movies()?)?;
    } else if let Some(show_matches) = matches.subcommand_matches("show") {
        if let Ok(movie_id) = show_matches.value_of("movie_id").unwrap().parse::<u32>() {
            handle_resp(&matches, client.get_movie(movie_id)?)?;
        } else {
            // TODO return a proper error
            eprintln!("Failed to parse movie_id.");
            process::exit(1);
        }
    } else if let Some(add_matches) = matches.subcommand_matches("add") {
        let data_source = get_data_source(add_matches.value_of("file"), add_matches.value_of("data"));

        if data_source.is_none() {
            panic!("Invalid usage");
        }

        let data_source = data_source.unwrap();

        eprintln!("Gotta read data.");
        // let search_result = get_search_result_from(data)?;

        // let add_movie_payload = radarr::AddMoviePayload::from_movie_response(search_result)?;
        // let resp = client.add_movie(add_movie_payload)?;
        // handle_resp(&matches, resp);
    } else if let Some(del_matches) = matches.subcommand_matches("delete") {
        let delete_files = del_matches.is_present("delete_files");

        if let Ok(movie_id) = del_matches.value_of("movie_id").unwrap().parse::<u32>() {
            handle_resp(&matches, client.delete_movie(movie_id, delete_files)?)?;
        } else {
            // TODO return a proper error
            eprintln!("Failed to parse movie_id.");
            process::exit(1);
        }
    } else {
        panic!("Unhandled subcommand. No bueno.")
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_data_source_does_not_allow_both_args() {
        let result = get_data_source(Some("foo"), Some("bar"));

        assert!(result.is_none())
    }

    #[test]
    fn get_data_source_returns_stdin_when_both_none() {
        let result = get_data_source(None, None);
        let source = result.unwrap();

        match source {
            DataSource::Stdin => {},
            _ => panic!("Expected DataSource::Stdin"),
        }
    }

    #[test]
    fn get_data_source_returns_file_path_when_path_passed() {
        let result = get_data_source(Some("/foo/bar"), None).unwrap();

        match result {
            DataSource::File(file_path) => assert_eq!(file_path, "/foo/bar"),
            _ => panic!("Expected DataSource::File"),
        }
    }

    #[test]
    fn get_data_source_returns_stdin_when_file_path_is_dash() {
        let result = get_data_source(Some("-"), None).unwrap();

        match result {
            DataSource::Stdin => {},
            _ => panic!("Expected DataSource::Stdin"),
        }
    }

    #[test]
    fn get_data_source_returns_data_when_data_is_passed() {
        let result = get_data_source(None, Some("foo")).unwrap();

        match result {
            DataSource::Data(data) => assert_eq!(data, "foo"),
            _ => panic!("Expected DataSource::Data"),
        }
    }
}
fn get_data_source(file_path: Option<&str>, data: Option<&str>) -> Option<DataSource> {
    match (file_path, data) {
        (Some(_), Some(_)) => {
            eprintln!("nothing.");
            None
        },
        (None, Some(data)) => {
            // got raw data
            eprintln!("passed data");
            Some(DataSource::Data(String::from(data)))
        },
        (file_path, None) => {
            let file_path = file_path.unwrap_or("-");

            if file_path == "-" {
                // read from stdin 
                eprintln!("stdin!");
                Some(DataSource::Stdin)
            } else {
                // got just a file path
                eprintln!("file");
                Some(DataSource::File(String::from(file_path)))
            }
        },
    }
}

// fn get_data(matches: &ArgMatches) -> Result<String, Box<dyn Error>> {
    // match (matches.value_of("file_path"), matches.value_of("data")) {
    // }
    // Ok(String::from(""))
// }

// fn get_search_result_from(data: String) -> Result<radarr::MovieResponse, Box<dyn Error> {
    // let movie_response = serde_json::from_str(&data)?;

    // Ok(movie_response)
// }

fn handle_resp<T: Debug + serde::Serialize>(matches: &ArgMatches, resp: radarr::Response<T>) -> Result<(), Box<dyn Error>> {
    if matches.is_present("json") {
        // let json = serde_json::to_string(&resp.data)?;
        println!("{}", json!(&resp.data));
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
