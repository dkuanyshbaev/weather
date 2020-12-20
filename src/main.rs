mod errors;
mod filters;
mod handlers;
mod types;
mod weather;

use std::env;
use warp::Filter;

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let routes = filters::weather()
        .with(warp::log("weather"))
        .with(warp::cors().allow_any_origin())
        .recover(errors::handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 4444)).await;
}
