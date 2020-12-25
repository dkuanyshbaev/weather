// Test task.
//
// This is a simple weather forecast application which can show the temperature for the city.
// Data sources: OpenWeather, MetaWeather.
//
// There are two endpoints:
// /day/<cityname> - for current day data
// /week/<cityname> - for 5 days data
//
// Example usage:
// curl http://localhost:4444/day/london
// curl http://localhost:4444/week/london

mod errors;
mod filters;
mod handlers;
mod types;
mod weather;

use std::env;
use warp::Filter;

#[tokio::main]
async fn main() {
    // Setting up logger
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    // Endpoints
    let routes = filters::weather()
        .with(warp::log("weather"))
        .with(warp::cors().allow_any_origin())
        .recover(errors::handle_rejection);

    // Starting server on port 4444
    warp::serve(routes).run(([127, 0, 0, 1], 4444)).await;
}
