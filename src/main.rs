// extern crate alamo_movies;
use alamo_movies::cinema::Cinema;

mod radarr;

// how this should work:
// iterate over every cinema from alamo
// filter out all movies of show-type terror tuesday, weird wednesday, video vortext
// search radarr API
// from those results, find best match. best match would be exact match on title or any
// "alternativeTitles"
// add any movies that are not already added.

fn main() {
    let config = radarr::Config::new_from_env().unwrap();
    let client = radarr::Client::new(config);

    let status = client.status().expect("Failed to connect");

    eprintln!("Status: {:#?}", status);

    let health = client.health().expect("Failed to get health");
    eprintln!("Health: {:#?}", health);

    let ping = client.ping().expect("Failed to ping");
    eprintln!("Ping: {:#?}", ping);

    let root_folders = client.root_folder().expect("Failed to get root folders");
    eprintln!("Root folders: {:#?}", root_folders);

    let movies = client.list_movies().expect("failed to list movies");
    eprintln!("movies: {}", movies.len());

    let movie = client.movie(63).expect("Failed to get alien");
    eprintln!("movie: {:#?}", movie);

    let good_movie_resp = match client.search("affinity") {
        Ok(Some(movies)) => movies,
        Ok(None) => panic!("No movie results."),
        Err(error) => panic!("Error: {}", error),
    };

    eprintln!("TMDB ID: {}", good_movie_resp[0].tmdb_id);

    if let Some(mut good_movie) = radarr::AddMoviePayload::from_movie_response(&good_movie_resp[0]) {
        good_movie.set_monitored(true);
        good_movie.set_root_folder_path("/storage/Movies");
        client.add_movie(&good_movie).expect("Failed to add movie");
    } else {
        panic!("Failed to get good movie");
    }

    let cinema_id = Cinema::to_cinema_id("new-mission").unwrap();
    let body = Cinema::get_calendar_data(&cinema_id).expect("expected thing");
    let (_cinema, films) = Cinema::from_calendar_data(&body).expect("expected thing");

    for film in films {
        if &film.show_type != "Terror Tuesday" {
            continue;
        }

        let title = &film.name;

        match client.search(title) {
            Ok(Some(results)) => {
                let num_results = results.len();
                println!("Got back {} results for {}", num_results, title);
                if let Some(best) = best_matches(title, results) {
                    println!("Best results {}/{} for {}", best.len(), num_results, title);
                } else {
                    eprintln!("Found no exact matches for {}", title); 
                }
            },
            Ok(None) => eprintln!("Got no results for `{}`.", film.name),
            Err(error) => panic!("Error: {}", error),
        }
    }
}

/// given the results from the radarr api, return an array of best matches
/// best matches are exact matches or exact alternativeNames matches
fn best_matches(term: &str, results: Vec<radarr::SearchResult>) -> Option<Vec<radarr::SearchResult>> {
    let matches: Vec<radarr::SearchResult> = results.into_iter()
        .filter(|result| {
            result.title.to_lowercase() == term.to_lowercase() 
                || result.alternative_titles.iter()
                    .any(|title| title.to_lowercase() == term.to_lowercase())
        })
        .collect();

    if matches.len() == 0 {
        return None;
    }

    Some(matches)
}

// fn url_for(uri: &str, config: &RadarrConfig, query_string: &str) -> &str {
    // &format!("{}://{}/api/{}?{}", config.protocol, config.hostname, uri, query_string)
// }


// fn root_folder(config: &Config) -> Result<RadarrRootFolder, Box<dyn Error>> {
    // let query_string: String = form_urlencoded::Serializer::new(String::new())
        // .append_pair("apikey", &config.api_token)
        // .finish();

    // let url: &str = url_for("rootfolder", config, query_string);
// }
