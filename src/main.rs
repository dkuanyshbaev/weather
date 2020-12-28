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
    warp::serve(routes).run(([0, 0, 0, 0], 4444)).await;
}

#[cfg(test)]
mod tests {
    use super::filters;
    use warp::http::StatusCode;
    use warp::test::request;

    #[tokio::test]
    async fn test_day_weather() {
        let api = filters::weather();
        let resp = request()
            .method("GET")
            .path("/day/moscow")
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_week_weather() {
        let api = filters::weather();
        let resp = request()
            .method("GET")
            .path("/week/moscow")
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}
