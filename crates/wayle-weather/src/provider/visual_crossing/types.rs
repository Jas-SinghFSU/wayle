#![allow(dead_code)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub resolved_address: String,
    pub timezone: String,
    pub current_conditions: CurrentConditions,
    pub days: Vec<DayData>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentConditions {
    pub datetime: String,
    pub temp: f64,
    pub feelslike: f64,
    pub humidity: f64,
    pub dew: f64,
    pub precip: Option<f64>,
    pub precipprob: Option<f64>,
    pub windspeed: f64,
    pub winddir: f64,
    pub windgust: Option<f64>,
    pub pressure: f64,
    pub visibility: f64,
    pub cloudcover: f64,
    pub uvindex: f64,
    pub conditions: String,
    pub icon: String,
    pub sunrise: String,
    pub sunset: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DayData {
    pub datetime: String,
    pub tempmax: f64,
    pub tempmin: f64,
    pub temp: f64,
    pub feelslike: f64,
    pub humidity: f64,
    pub dew: f64,
    pub precip: Option<f64>,
    pub precipprob: Option<f64>,
    pub windspeed: f64,
    pub windgust: Option<f64>,
    pub winddir: f64,
    pub pressure: f64,
    pub cloudcover: f64,
    pub visibility: f64,
    pub uvindex: f64,
    pub sunrise: String,
    pub sunset: String,
    pub conditions: String,
    pub icon: String,
    #[serde(default)]
    pub hours: Vec<HourData>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HourData {
    pub datetime: String,
    pub temp: f64,
    pub feelslike: f64,
    pub humidity: f64,
    pub dew: f64,
    pub precip: Option<f64>,
    pub precipprob: Option<f64>,
    pub windspeed: f64,
    pub winddir: f64,
    pub windgust: Option<f64>,
    pub pressure: f64,
    pub visibility: f64,
    pub cloudcover: f64,
    pub uvindex: f64,
    pub conditions: String,
    pub icon: String,
}
