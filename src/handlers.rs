use crate::weather;

pub async fn day_handler(city: String, date: String) -> Result<impl warp::Reply, warp::Rejection> {
    let t = weather::day(&city, &date).await?;

    Ok(warp::reply::json(&format!(
        "day forecast for city: {} and date: {}, t = {}",
        city, date, t
    )))
}

pub async fn week_handler(city: String) -> Result<impl warp::Reply, warp::Rejection> {
    let t = weather::week(&city).await?;

    Ok(warp::reply::json(&format!(
        "week forecast for city: {}, t = {:?}",
        city, t
    )))
}
