# radarr

> A client library for the radarr API written in rust

This project contains a work-in-progress rust library for interacting with the [radarr](https://radarr.video/)
API. This is one of the first projects I've written in rust, so I'm sure there's room for improvement.

The goal of this project is to make interacting witht he radarr API as straight-forward as possible.

## Getting started

The basic flow of setting up a client to make calls to the radarr API involves:

 1. create an instance of the `radarr::Config` struct containing the hostname and api token
 2. create an instance of the `radarr::Client`, passing said configuration
 3. make calls and have fun

A quick example:

```rust
use radarr;

fn main() {
  let config = radarr::Config::new_with_defaults("XXXXXXXXXXXXXXXXXXXXX").unwrap();
  let client = radarr::Client::new(config).expect("Failed to create radarr client");

  // now we can make calls:
  let health = client.health().expect("Failed to connect to radarr host");

  println!("Connected: {}", health.message);
}
```

Alternatively, if you have environment variables set, you can use `radarr::Config::new_from_env_with_defaults()` to
initialize a config from that. It expects the following variables:

 * `RADARR_API_TOKEN` -- the API token. Without this, the `Config.api_token` will be `None`
 * `RADARR_API_HOSTNAME` -- the hostname for the radar instance. Defaults to `localhost`. This may contain a
     port specifier. For example `radarr.example.com` or `localhost:8585` are valid values.
 * `RADARR_API_PROTOCOL` -- the protocol to use for the api. Defaults to `http`.

Additional documentation is forthcoming.

## License

This is MIT licensed. See `LICENSE.md`.

