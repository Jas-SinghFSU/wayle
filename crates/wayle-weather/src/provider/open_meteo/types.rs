#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ForecastRequest {
    pub latitude: f64,
    pub longitude: f64,
    pub hourly: &'static str,
    pub daily: &'static str,
    pub temperature_unit: &'static str,
    pub wind_speed_unit: &'static str,
    pub timezone: &'static str,
    pub forecast_days: u8,
}

#[derive(Debug, Serialize)]
pub struct GeocodingRequest<'a> {
    pub name: &'a str,
    pub count: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
}

#[derive(Debug, Deserialize)]
pub struct GeocodingResponse {
    #[serde(default)]
    pub results: Vec<GeoLocation>,
}

#[derive(Debug, Deserialize)]
pub struct GeoLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub name: String,
    pub admin1: Option<String>,
    pub country: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub timezone: Option<String>,
    pub hourly: HourlyData,
    pub daily: DailyData,
}

#[derive(Debug, Deserialize)]
pub struct HourlyData {
    pub time: Vec<String>,
    #[serde(default)]
    pub temperature_2m: Vec<f64>,
    #[serde(default)]
    pub relative_humidity_2m: Vec<f64>,
    #[serde(default)]
    pub apparent_temperature: Vec<f64>,
    #[serde(default)]
    pub precipitation_probability: Vec<f64>,
    #[serde(default)]
    pub precipitation: Vec<f64>,
    #[serde(default)]
    pub weather_code: Vec<f64>,
    #[serde(default)]
    pub cloud_cover: Vec<f64>,
    #[serde(default)]
    pub pressure_msl: Vec<f64>,
    #[serde(default)]
    pub visibility: Vec<f64>,
    #[serde(default)]
    pub wind_speed_10m: Vec<f64>,
    #[serde(default)]
    pub wind_direction_10m: Vec<f64>,
    #[serde(default)]
    pub wind_gusts_10m: Vec<f64>,
    #[serde(default)]
    pub dew_point_2m: Vec<f64>,
    #[serde(default)]
    pub uv_index: Vec<f64>,
    #[serde(default)]
    pub is_day: Vec<f64>,
}

#[derive(Debug, Deserialize)]
pub struct DailyData {
    pub time: Vec<String>,
    #[serde(default)]
    pub weather_code: Vec<f64>,
    #[serde(default)]
    pub temperature_2m_max: Vec<f64>,
    #[serde(default)]
    pub temperature_2m_min: Vec<f64>,
    #[serde(default)]
    pub relative_humidity_2m_mean: Vec<f64>,
    #[serde(default)]
    pub sunrise: Vec<String>,
    #[serde(default)]
    pub sunset: Vec<String>,
    #[serde(default)]
    pub uv_index_max: Vec<f64>,
    #[serde(default)]
    pub precipitation_sum: Vec<f64>,
    #[serde(default)]
    pub precipitation_probability_max: Vec<f64>,
    #[serde(default)]
    pub wind_speed_10m_max: Vec<f64>,
}
