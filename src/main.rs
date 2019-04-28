use std::error::Error;
use std::process;
use std::fmt::Debug;

use serde_json::json;

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
                    .arg(Arg::with_name("no-search")
                         .help("Do not search for the movie. Just add it.")
                         .long("no-search")
                         .takes_value(false)
                         )
                    .arg(Arg::with_name("root-folder")
                         .help("Root folder to download movie. Default is to pick first in available root folders.")
                         .long("root-folder")
                         .short("r")
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
        let data_source = radarr::DataSource::from_matches(&add_matches); //(add_matches.value_of("file"), add_matches.value_of("data"));

        if data_source.is_none() {
            panic!("Invalid usage");
        }

        // parse the payload from wherever we're getting it.
        let data_source = data_source.unwrap();
        let data = data_source.read()?;
        let result: radarr::SearchResult = serde_json::from_str(&data)?;


        let mut payload = radarr::AddMoviePayload::from_movie_response(&result).unwrap();
        if ! add_matches.is_present("no-search") {
            payload.set_search_for_movie(true);
        }

        let root_folder = 
            if let Some(root_folder) = add_matches.value_of("root-folder") {
                root_folder.to_string()
            } else {
                let root_folders = client.root_folder()?;
                root_folders.data[0].path.to_owned()
            };

        payload.set_root_folder_path(&root_folder);

        let resp = client.add_movie(&payload)?;
        handle_resp(&matches, resp)?;
        
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

