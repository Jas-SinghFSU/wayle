use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use tracing::warn;

use super::types::{ApiResponse, HourlyData};
use crate::{
    error::{Error, Result},
    model::{CurrentWeather, DailyForecast, HourlyForecast, WeatherCondition},
    types::{
        Distance, Percentage, Precipitation, Pressure, Speed, Temperature, UvIndex, WindDirection,
    },
};

pub const PROVIDER: &str = "open-meteo";

pub fn build_current(data: &ApiResponse) -> Result<CurrentWeather> {
    let hourly = &data.hourly;
    let idx = find_current_hour_index(&hourly.time);

    let is_day_value = is_day(&hourly.is_day, idx)?;
    let wmo_code = raw_u8(&hourly.weather_code, idx)?;

    Ok(CurrentWeather {
        temperature: temperature(&hourly.temperature_2m, idx)?,
        feels_like: temperature(&hourly.apparent_temperature, idx)?,
        condition: WeatherCondition::from_wmo_code(wmo_code),
        humidity: percentage(&hourly.relative_humidity_2m, idx)?,
        wind_speed: speed(&hourly.wind_speed_10m, idx)?,
        wind_direction: wind_dir(&hourly.wind_direction_10m, idx)?,
        wind_gust: speed(&hourly.wind_gusts_10m, idx)?,
        uv_index: uv(&hourly.uv_index, idx)?,
        cloud_cover: percentage(&hourly.cloud_cover, idx)?,
        pressure: pressure(&hourly.pressure_msl, idx)?,
        visibility: visibility_from_meters(&hourly.visibility, idx)?,
        dewpoint: temperature(&hourly.dew_point_2m, idx)?,
        precipitation: precip(&hourly.precipitation, idx)?,
        is_day: is_day_value,
    })
}

pub fn build_hourly(hourly: &HourlyData, count: usize) -> Result<Vec<HourlyForecast>> {
    let start = find_current_hour_index(&hourly.time);
    let end = (start + count).min(hourly.time.len());

    let mut forecasts = Vec::with_capacity(count);
    for hour_idx in start..end {
        forecasts.push(HourlyForecast {
            time: parse_iso_datetime(&hourly.time[hour_idx])?,
            temperature: temperature(&hourly.temperature_2m, hour_idx)?,
            feels_like: temperature(&hourly.apparent_temperature, hour_idx)?,
            condition: WeatherCondition::from_wmo_code(raw_u8(&hourly.weather_code, hour_idx)?),
            humidity: percentage(&hourly.relative_humidity_2m, hour_idx)?,
            wind_speed: speed(&hourly.wind_speed_10m, hour_idx)?,
            wind_direction: wind_dir(&hourly.wind_direction_10m, hour_idx)?,
            wind_gust: speed(&hourly.wind_gusts_10m, hour_idx)?,
            rain_chance: percentage(&hourly.precipitation_probability, hour_idx)?,
            uv_index: uv(&hourly.uv_index, hour_idx)?,
            cloud_cover: percentage(&hourly.cloud_cover, hour_idx)?,
            pressure: pressure(&hourly.pressure_msl, hour_idx)?,
            visibility: visibility_from_meters(&hourly.visibility, hour_idx)?,
            dewpoint: temperature(&hourly.dew_point_2m, hour_idx)?,
            precipitation: precip(&hourly.precipitation, hour_idx)?,
            is_day: is_day(&hourly.is_day, hour_idx)?,
        });
    }
    Ok(forecasts)
}

pub fn build_daily(data: &ApiResponse, count: usize) -> Result<Vec<DailyForecast>> {
    let daily = &data.daily;
    let end = daily.time.len().min(count);

    let mut forecasts = Vec::with_capacity(count);
    for day_idx in 0..end {
        let date = parse_date(&daily.time[day_idx])?;
        let sunrise = parse_time_from_iso(&daily.sunrise[day_idx])?;
        let sunset = parse_time_from_iso(&daily.sunset[day_idx])?;

        let temp_high = temperature(&daily.temperature_2m_max, day_idx)?;
        let temp_low = temperature(&daily.temperature_2m_min, day_idx)?;
        let avg = (temp_high.celsius() + temp_low.celsius()) / 2.0;

        forecasts.push(DailyForecast {
            date,
            condition: WeatherCondition::from_wmo_code(raw_u8(&daily.weather_code, day_idx)?),
            temp_high,
            temp_low,
            temp_avg: Temperature::new(avg)
                .ok_or_else(|| Error::parse(PROVIDER, "invalid avg temp"))?,
            humidity_avg: percentage(&daily.relative_humidity_2m_mean, day_idx)?,
            wind_speed_max: speed(&daily.wind_speed_10m_max, day_idx)?,
            rain_chance: percentage(&daily.precipitation_probability_max, day_idx)?,
            uv_index_max: uv(&daily.uv_index_max, day_idx)?,
            precipitation_sum: precip(&daily.precipitation_sum, day_idx)?,
            sunrise,
            sunset,
        });
    }
    Ok(forecasts)
}

pub fn find_current_hour_index(times: &[String]) -> usize {
    let now = Local::now().naive_local();
    for (hour_idx, time_str) in times.iter().enumerate() {
        if let Ok(datetime) = parse_iso_datetime(time_str)
            && datetime > now
        {
            return hour_idx.saturating_sub(1);
        }
    }
    warn!("no future hour found in forecast, using fallback index 0");
    0
}

pub fn parse_iso_datetime(s: &str) -> Result<NaiveDateTime> {
    NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M")
        .map_err(|err| Error::parse(PROVIDER, err.to_string()))
}

pub fn parse_date(s: &str) -> Result<NaiveDate> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|err| Error::parse(PROVIDER, err.to_string()))
}

pub fn parse_time_from_iso(s: &str) -> Result<NaiveTime> {
    let dt = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M")
        .map_err(|err| Error::parse(PROVIDER, err.to_string()))?;
    Ok(dt.time())
}

fn raw_f64(arr: &[f64], idx: usize) -> Result<f64> {
    arr.get(idx)
        .copied()
        .ok_or_else(|| Error::parse(PROVIDER, "missing data"))
}

fn raw_u8(arr: &[f64], idx: usize) -> Result<u8> {
    raw_f64(arr, idx).map(|raw| raw.clamp(0.0, 255.0) as u8)
}

fn temperature(arr: &[f64], idx: usize) -> Result<Temperature> {
    let celsius = raw_f64(arr, idx)? as f32;
    Temperature::new(celsius).ok_or_else(|| Error::parse(PROVIDER, "invalid temperature"))
}

fn percentage(arr: &[f64], idx: usize) -> Result<Percentage> {
    let percent = raw_f64(arr, idx)?.clamp(0.0, 100.0) as u8;
    Ok(Percentage::saturating(percent))
}

fn speed(arr: &[f64], idx: usize) -> Result<Speed> {
    let kmh = raw_f64(arr, idx)?.max(0.0) as f32;
    Speed::new(kmh).ok_or_else(|| Error::parse(PROVIDER, "invalid speed"))
}

fn wind_dir(arr: &[f64], idx: usize) -> Result<WindDirection> {
    let degrees = raw_f64(arr, idx)?.max(0.0) as u16;
    Ok(WindDirection::saturating(degrees))
}

fn uv(arr: &[f64], idx: usize) -> Result<UvIndex> {
    let index = raw_f64(arr, idx)?.clamp(0.0, 15.0) as u8;
    Ok(UvIndex::saturating(index))
}

fn pressure(arr: &[f64], idx: usize) -> Result<Pressure> {
    let hpa = raw_f64(arr, idx)?.max(0.0) as f32;
    Pressure::new(hpa).ok_or_else(|| Error::parse(PROVIDER, "invalid pressure"))
}

fn visibility_from_meters(arr: &[f64], idx: usize) -> Result<Distance> {
    let meters = raw_f64(arr, idx)?.max(0.0) as f32;
    Distance::from_meters(meters).ok_or_else(|| Error::parse(PROVIDER, "invalid visibility"))
}

fn precip(arr: &[f64], idx: usize) -> Result<Precipitation> {
    let mm = raw_f64(arr, idx)?.max(0.0) as f32;
    Precipitation::new(mm).ok_or_else(|| Error::parse(PROVIDER, "invalid precipitation"))
}

fn is_day(arr: &[f64], idx: usize) -> Result<bool> {
    raw_f64(arr, idx).map(|value| value > 0.5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_iso_datetime_valid() {
        let result = parse_iso_datetime("2024-01-15T14:30");
        assert!(result.is_ok());
        let dt = result.unwrap();
        assert_eq!(dt.format("%Y-%m-%d %H:%M").to_string(), "2024-01-15 14:30");
    }

    #[test]
    fn parse_iso_datetime_invalid() {
        let result = parse_iso_datetime("not-a-date");
        assert!(result.is_err());
    }

    #[test]
    fn parse_date_valid() {
        let result = parse_date("2024-01-15");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string(), "2024-01-15");
    }

    #[test]
    fn parse_time_from_iso_valid() {
        let result = parse_time_from_iso("2024-01-15T14:30");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string(), "14:30:00");
    }

    #[test]
    fn percentage_clamps_values() {
        let arr = vec![150.0, -10.0, 50.0];
        assert_eq!(percentage(&arr, 0).unwrap().get(), 100);
        assert_eq!(percentage(&arr, 1).unwrap().get(), 0);
        assert_eq!(percentage(&arr, 2).unwrap().get(), 50);
    }

    #[test]
    fn wind_direction_wraps() {
        let arr = vec![0.0, 90.0, 180.0, 270.0, 400.0];
        assert_eq!(wind_dir(&arr, 0).unwrap().degrees(), 0);
        assert_eq!(wind_dir(&arr, 1).unwrap().degrees(), 90);
        assert_eq!(wind_dir(&arr, 4).unwrap().degrees(), 40);
    }

    #[test]
    fn uv_clamps_high_values() {
        let arr = vec![0.0, 11.0, 20.0];
        assert_eq!(uv(&arr, 0).unwrap().get(), 0);
        assert_eq!(uv(&arr, 1).unwrap().get(), 11);
        assert_eq!(uv(&arr, 2).unwrap().get(), 15);
    }

    #[test]
    fn visibility_converts_meters_to_km() {
        let arr = vec![10000.0, 500.0];
        let vis1 = visibility_from_meters(&arr, 0).unwrap();
        let vis2 = visibility_from_meters(&arr, 1).unwrap();
        assert!((vis1.km() - 10.0).abs() < 0.01);
        assert!((vis2.km() - 0.5).abs() < 0.01);
    }
}
