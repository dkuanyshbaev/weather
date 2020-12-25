use crate::weather;

// One day data
pub async fn day_handler(city: String) -> Result<impl warp::Reply, warp::Rejection> {
    let t = weather::day(&city).await?;

    Ok(warp::reply::json(&format!(
        "day temperature at {}: {}",
        city, t
    )))
}

// One week data
pub async fn week_handler(city: String) -> Result<impl warp::Reply, warp::Rejection> {
    let t = weather::week(&city).await?;

    Ok(warp::reply::json(&format!(
        "week temperature at {}: {:?}",
        city, t
    )))
}
