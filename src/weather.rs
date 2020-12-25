use crate::errors;
use crate::types::*;

// Kelvins to Celsius
const K_TEMP: f32 = 273.15;
// OpenWeather key
const OPEN_WEATHER_API_KEY: &str = "3747e0afa07e05391851a943d0a18237";

// Calculate avg temperature for one day
pub async fn day(city: &String) -> Result<f32, warp::Rejection> {
    let t1 = day_weather_source_1(city).await?;
    let t2 = day_weather_source_2(city).await?;

    Ok((t1 + t2) / 2.0)
}

// Calculate avg temperature for week
pub async fn week(city: &String) -> Result<Vec<f32>, warp::Rejection> {
    let v1 = week_weather_source_1(city).await?;
    let v2 = week_weather_source_2(city).await?;

    Ok(v1.iter().zip(v2).map(|(a, b)| (a + b) / 2.0).collect())
}

// First data sourse for one day data
async fn day_weather_source_1(city: &String) -> Result<f32, warp::Rejection> {
    match get_meta_weather_city_id(city).await {
        Ok(city_id) => match get_meta_weather_t(city_id).await {
            Ok(t) => Ok(t),
            // TODO: convert error
            Err(_error) => Err(warp::reject::custom(errors::MetaWeatherError)),
        },
        // TODO: convert error
        Err(_error) => Err(warp::reject::custom(errors::MetaWeatherError)),
    }
}

// Second data sourse for one day data
async fn day_weather_source_2(city: &String) -> Result<f32, warp::Rejection> {
    match get_open_weather_t(city).await {
        Ok(t) => Ok(t),
        // TODO: convert error
        Err(_error) => Err(warp::reject::custom(errors::OpenWeatherError)),
    }
}

// First data sourse for one week data
async fn week_weather_source_1(city: &String) -> Result<Vec<f32>, warp::Rejection> {
    match get_meta_weather_city_id(city).await {
        Ok(city_id) => match get_meta_weather_week_t(city_id).await {
            Ok(tv) => Ok(tv),
            // TODO: convert error
            Err(_error) => Err(warp::reject::custom(errors::MetaWeatherError)),
        },
        // TODO: convert error
        Err(_error) => Err(warp::reject::custom(errors::MetaWeatherError)),
    }
}

// Second data sourse for one week data
async fn week_weather_source_2(city: &String) -> Result<Vec<f32>, warp::Rejection> {
    match get_open_weather_week_t(city).await {
        Ok(tv) => Ok(tv),
        // TODO: convert error
        Err(_error) => Err(warp::reject::custom(errors::OpenWeatherError)),
    }
}

// Get city id for MetaWeather
async fn get_meta_weather_city_id(city: &String) -> Result<i32, reqwest::Error> {
    match reqwest::get(&format!(
        "https://www.metaweather.com/api/location/search/?query={}",
        city
    ))
    .await
    {
        Ok(response) => match response.json::<Vec<MetaWeatherLocation>>().await {
            Ok(location) => Ok(location[0].woeid),
            Err(error) => Err(error),
        },
        Err(error) => Err(error),
    }
}

// Get MetaWeather data for one day
async fn get_meta_weather_t(city_id: i32) -> Result<f32, reqwest::Error> {
    match reqwest::get(&format!(
        "https://www.metaweather.com/api/location/{}",
        city_id
    ))
    .await
    {
        Ok(response) => match response.json::<MetaWeatherData>().await {
            Ok(data) => Ok(data.consolidated_weather[0].the_temp),
            Err(error) => Err(error),
        },
        Err(error) => Err(error),
    }
}

// Get OpenWeather data for one day
async fn get_open_weather_t(city: &String) -> Result<f32, reqwest::Error> {
    match reqwest::get(&format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
        city, OPEN_WEATHER_API_KEY
    ))
    .await
    {
        Ok(response) => match response.json::<OpenWeatherData>().await {
            Ok(data) => Ok(data.main.temp - K_TEMP),
            Err(error) => Err(error),
        },
        Err(error) => Err(error),
    }
}

// Get MetaWeather data for one week
async fn get_meta_weather_week_t(city_id: i32) -> Result<Vec<f32>, reqwest::Error> {
    match reqwest::get(&format!(
        "https://www.metaweather.com/api/location/{}",
        city_id
    ))
    .await
    {
        Ok(response) => match response.json::<MetaWeatherData>().await {
            Ok(data) => Ok(data
                .consolidated_weather
                .iter()
                .take(5)
                .map(|i| i.the_temp)
                .collect()),
            Err(error) => Err(error),
        },
        Err(error) => Err(error),
    }
}

// Get OpenWeather data for one week
async fn get_open_weather_week_t(city: &String) -> Result<Vec<f32>, reqwest::Error> {
    match reqwest::get(&format!(
        "https://api.openweathermap.org/data/2.5/forecast?q={}&cnt=40&appid={}",
        city, OPEN_WEATHER_API_KEY
    ))
    .await
    {
        Ok(response) => match response.json::<OpenWeatherForecast>().await {
            // We have a temperature for each 3h range from OpenWeather
            // Using 12:00 value as day value
            Ok(data) => Ok(data
                .list
                .iter()
                .skip(2)
                .step_by(8)
                .map(|i| i.main.temp - K_TEMP)
                .collect()),
            Err(error) => Err(error),
        },
        Err(error) => Err(error),
    }
}
