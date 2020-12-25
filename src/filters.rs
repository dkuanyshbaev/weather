use crate::handlers;
use warp::Filter;

// the weather filters combined
pub fn weather() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    day_forecast().or(week_forecast())
}

// GET /day/london
pub fn day_forecast() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("day")
        .and(warp::get())
        .and(warp::path::param())
        .and_then(handlers::day_handler)
}

// GET /week/london
pub fn week_forecast() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("week")
        .and(warp::get())
        .and(warp::path::param())
        .and_then(handlers::week_handler)
}
