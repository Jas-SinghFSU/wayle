#![allow(dead_code)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub location: LocationData,
    pub current: CurrentData,
    pub forecast: ForecastWrapper,
}

#[derive(Debug, Deserialize)]
pub struct LocationData {
    pub name: String,
    pub region: String,
    pub country: String,
    pub lat: f64,
    pub lon: f64,
    pub tz_id: String,
}

#[derive(Debug, Deserialize)]
pub struct CurrentData {
    pub temp_c: f64,
    pub temp_f: f64,
    pub is_day: i32,
    pub condition: ConditionData,
    pub wind_kph: f64,
    pub wind_degree: f64,
    pub pressure_mb: f64,
    pub precip_mm: f64,
    pub humidity: f64,
    pub cloud: f64,
    pub feelslike_c: f64,
    pub feelslike_f: f64,
    pub vis_km: f64,
    pub uv: f64,
    pub gust_kph: f64,
    pub dewpoint_c: f64,
}

#[derive(Debug, Deserialize)]
pub struct ConditionData {
    pub text: String,
    pub code: i32,
}

#[derive(Debug, Deserialize)]
pub struct ForecastWrapper {
    pub forecastday: Vec<ForecastDay>,
}

#[derive(Debug, Deserialize)]
pub struct ForecastDay {
    pub date: String,
    pub day: DayData,
    pub astro: AstroData,
    pub hour: Vec<HourData>,
}

#[derive(Debug, Deserialize)]
pub struct DayData {
    pub maxtemp_c: f64,
    pub mintemp_c: f64,
    pub avgtemp_c: f64,
    pub maxwind_kph: f64,
    pub totalprecip_mm: f64,
    pub avghumidity: f64,
    pub daily_chance_of_rain: f64,
    pub condition: ConditionData,
    pub uv: f64,
}

#[derive(Debug, Deserialize)]
pub struct AstroData {
    pub sunrise: String,
    pub sunset: String,
}

#[derive(Debug, Deserialize)]
pub struct HourData {
    pub time: String,
    pub temp_c: f64,
    pub is_day: i32,
    pub condition: ConditionData,
    pub wind_kph: f64,
    pub wind_degree: f64,
    pub pressure_mb: f64,
    pub precip_mm: f64,
    pub humidity: f64,
    pub cloud: f64,
    pub feelslike_c: f64,
    pub vis_km: f64,
    pub uv: f64,
    pub gust_kph: f64,
    pub dewpoint_c: f64,
    pub chance_of_rain: f64,
}
