use std::convert::Infallible;
use std::convert::TryFrom;
use std::time::Duration;
use weather_underground as wu;
// use openweather_async::{OpenWeather, Units};

pub async fn day(city: &String, date: &String) -> Result<f32, Infallible> {
    let t1 = day_weather_source_1(city, date).await?;
    let t2 = day_weather_source_2(city, date).await?;

    Ok((t1 + t2) / 2.0)
}

pub async fn week(city: &String) -> Result<Vec<f32>, Infallible> {
    let v1 = week_weather_source_1(city).await?;
    let v2 = week_weather_source_2(city).await?;

    Ok(v1.iter().zip(v2).map(|(a, b)| (a + b) / 2.0).collect())
}

async fn day_weather_source_1(_city: &String, _date: &String) -> Result<f32, Infallible> {
    let client = wu::create_client(Duration::from_secs(2)).unwrap();
    let api_key = wu::fetch_api_key(&client).await.unwrap();
    let unit = wu::Unit::Metric;
    let location = "IVLADI1";

    let result = wu::fetch_observation(&client, api_key.as_str(), location, &unit)
        .await
        .unwrap();
    if let Some(response) = result {
        let response = wu::ObservationResponse::try_from(response).unwrap();
        let a = response.observations.unwrap();
        let b = a[0].metric.as_ref().unwrap();
        let t = b.temp.unwrap();
        println!("t = {}", t);
    } else {
        println!("no data from server");
    }

    Ok(8.0)
}

async fn day_weather_source_2(_city: &String, _date: &String) -> Result<f32, Infallible> {
    //     let token = "3747e0afa07e05391851a943d0a18237";
    //     let weather: OpenWeather = OpenWeather::new(&token, Units::Metric).await?;
    //     let report = weather.get_by_city("Tokyo").await?;
    //     println!("{:?}", report);
    //     println!("{:?}", report.main);
    //     println!("{:?}", report.wind.speed);

    Ok(7.0)
}

async fn week_weather_source_1(_city: &String) -> Result<Vec<f32>, Infallible> {
    Ok(vec![3.0, 8.0, 4.0, 7.0, 6.0])
}

async fn week_weather_source_2(_city: &String) -> Result<Vec<f32>, Infallible> {
    Ok(vec![3.0, 8.0, 4.0, 7.0, 6.0])
}
