use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MetaWeatherLocation {
    pub title: String,
    pub location_type: String,
    pub woeid: i32,
    pub latt_long: String,
}

#[derive(Deserialize, Debug)]
pub struct MetaWeatherDay {
    pub id: i64,
    pub weather_state_name: String,
    pub weather_state_abbr: String,
    pub wind_direction_compass: String,
    pub created: String,
    pub applicable_date: String,
    pub min_temp: f32,
    pub max_temp: f32,
    pub the_temp: f32,
    pub wind_speed: f32,
    pub wind_direction: f32,
    pub air_pressure: f32,
    pub humidity: f32,
    pub visibility: f32,
    pub predictability: f32,
}

#[derive(Deserialize, Debug)]
pub struct MetaWeatherData {
    pub consolidated_weather: Vec<MetaWeatherDay>,
}

#[derive(Deserialize, Debug)]
pub struct Coord {
    pub lon: f32,
    pub lat: f32,
}

#[derive(Deserialize, Debug)]
pub struct Weather {
    pub id: i32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Deserialize, Debug)]
pub struct Main {
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub pressure: i32,
    pub humidity: i32,
}

#[derive(Deserialize, Debug)]
pub struct Wind {
    pub speed: f32,
    pub deg: i32,
}

#[derive(Deserialize, Debug)]
pub struct Clouds {
    pub all: i32,
}

#[derive(Deserialize, Debug)]
pub struct Sys {
    pub r#type: i32,
    pub id: i32,
    pub country: String,
    pub sunrise: i64,
    pub sunset: i64,
}

#[derive(Deserialize, Debug)]
pub struct OpenWeatherData {
    pub coord: Coord,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,
    pub visibility: i64,
    pub wind: Wind,
    pub clouds: Clouds,
    pub dt: i64,
    pub sys: Sys,
    pub timezone: i32,
    pub id: i32,
    pub name: String,
    pub cod: i32,
}
