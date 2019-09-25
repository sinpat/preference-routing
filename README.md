[![Build Status](https://travis-ci.com/sinpat/preference-routing.svg?branch=master)](https://travis-ci.com/sinpat/preference-routing)

# preference-routing
Specification of Trajectories and Learning of User Preferences

## Config
The application requires a [config file](config.toml) to run.

The file has to define the following properties:
- **port**: The port which is used by the server
- **database_path**: Used to save the application data

## Compile

`cargo build --release`

## Test

`cargo test`

## Run

`./target/release/preference-routing [path/to/graph/file]`
