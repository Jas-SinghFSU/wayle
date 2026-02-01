use chrono::{DateTime, NaiveDate, NaiveTime, Utc};

use super::types::{ApiResponse, DayData, HourData};
use crate::{
    error::{Error, Result},
    model::{CurrentWeather, DailyForecast, HourlyForecast, WeatherCondition},
    types::{
        Distance, Percentage, Precipitation, Pressure, Speed, Temperature, UvIndex, WindDirection,
    },
};

pub const PROVIDER: &str = "visual-crossing";

pub fn build_current(data: &ApiResponse) -> Result<CurrentWeather> {
    let current = &data.current_conditions;

    Ok(CurrentWeather {
        temperature: temperature(current.temp)?,
        feels_like: temperature(current.feelslike)?,
        condition: condition_from_icon(&current.icon),
        humidity: percentage(current.humidity),
        wind_speed: speed(current.windspeed)?,
        wind_direction: wind_dir(current.winddir),
        wind_gust: speed(current.windgust.unwrap_or(0.0))?,
        uv_index: uv(current.uvindex),
        cloud_cover: percentage(current.cloudcover),
        pressure: pressure(current.pressure)?,
        visibility: visibility(current.visibility)?,
        dewpoint: temperature(current.dew)?,
        precipitation: precip(current.precip.unwrap_or(0.0))?,
        is_day: is_daytime(&current.datetime, &current.sunrise, &current.sunset),
    })
}

pub fn build_hourly(data: &ApiResponse, count: usize) -> Result<Vec<HourlyForecast>> {
    let now = Utc::now();
    let mut forecasts = Vec::with_capacity(count);
    let mut collected = 0;

    for day in &data.days {
        let date = parse_date(&day.datetime)?;

        for hour in &day.hours {
            if collected >= count {
                break;
            }

            let time = parse_time(&hour.datetime)?;
            let datetime = date.and_time(time).and_utc();

            if datetime < now {
                continue;
            }

            forecasts.push(build_hourly_forecast(
                hour,
                datetime,
                &day.sunrise,
                &day.sunset,
            )?);
            collected += 1;
        }

        if collected >= count {
            break;
        }
    }

    Ok(forecasts)
}

fn build_hourly_forecast(
    hour: &HourData,
    datetime: DateTime<Utc>,
    sunrise: &str,
    sunset: &str,
) -> Result<HourlyForecast> {
    Ok(HourlyForecast {
        time: datetime,
        temperature: temperature(hour.temp)?,
        feels_like: temperature(hour.feelslike)?,
        condition: condition_from_icon(&hour.icon),
        humidity: percentage(hour.humidity),
        wind_speed: speed(hour.windspeed)?,
        wind_direction: wind_dir(hour.winddir),
        wind_gust: speed(hour.windgust.unwrap_or(0.0))?,
        rain_chance: percentage(hour.precipprob.unwrap_or(0.0)),
        uv_index: uv(hour.uvindex),
        cloud_cover: percentage(hour.cloudcover),
        pressure: pressure(hour.pressure)?,
        visibility: visibility(hour.visibility)?,
        dewpoint: temperature(hour.dew)?,
        precipitation: precip(hour.precip.unwrap_or(0.0))?,
        is_day: is_daytime(&hour.datetime, sunrise, sunset),
    })
}

pub fn build_daily(data: &ApiResponse, count: usize) -> Result<Vec<DailyForecast>> {
    let mut forecasts = Vec::with_capacity(count);

    for day in data.days.iter().take(count) {
        forecasts.push(build_daily_forecast(day)?);
    }

    Ok(forecasts)
}

fn build_daily_forecast(day_data: &DayData) -> Result<DailyForecast> {
    let date = parse_date(&day_data.datetime)?;
    let sunrise = parse_time(&day_data.sunrise)?;
    let sunset = parse_time(&day_data.sunset)?;

    let temp_high = temperature(day_data.tempmax)?;
    let temp_low = temperature(day_data.tempmin)?;
    let avg = (temp_high.celsius() + temp_low.celsius()) / 2.0;

    Ok(DailyForecast {
        date,
        condition: condition_from_icon(&day_data.icon),
        temp_high,
        temp_low,
        temp_avg: Temperature::new(avg)
            .ok_or_else(|| Error::parse(PROVIDER, "invalid avg temp"))?,
        humidity_avg: percentage(day_data.humidity),
        wind_speed_max: speed(day_data.windspeed)?,
        rain_chance: percentage(day_data.precipprob.unwrap_or(0.0)),
        uv_index_max: uv(day_data.uvindex),
        precipitation_sum: precip(day_data.precip.unwrap_or(0.0))?,
        sunrise,
        sunset,
    })
}

fn parse_date(s: &str) -> Result<NaiveDate> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| Error::parse(PROVIDER, e.to_string()))
}

fn parse_time(s: &str) -> Result<NaiveTime> {
    NaiveTime::parse_from_str(s, "%H:%M:%S")
        .or_else(|_| NaiveTime::parse_from_str(s, "%H:%M"))
        .map_err(|e| Error::parse(PROVIDER, e.to_string()))
}

fn temperature(celsius: f64) -> Result<Temperature> {
    Temperature::new(celsius as f32).ok_or_else(|| Error::parse(PROVIDER, "invalid temperature"))
}

fn percentage(percent: f64) -> Percentage {
    Percentage::saturating(percent.clamp(0.0, 100.0) as u8)
}

fn speed(kmh: f64) -> Result<Speed> {
    Speed::new(kmh.max(0.0) as f32).ok_or_else(|| Error::parse(PROVIDER, "invalid speed"))
}

fn wind_dir(degrees: f64) -> WindDirection {
    WindDirection::saturating(degrees.max(0.0) as u16)
}

fn uv(index: f64) -> UvIndex {
    UvIndex::saturating(index.clamp(0.0, 15.0) as u8)
}

fn pressure(hpa: f64) -> Result<Pressure> {
    Pressure::new(hpa.max(0.0) as f32).ok_or_else(|| Error::parse(PROVIDER, "invalid pressure"))
}

fn visibility(km: f64) -> Result<Distance> {
    Distance::new(km.max(0.0) as f32).ok_or_else(|| Error::parse(PROVIDER, "invalid visibility"))
}

fn precip(mm: f64) -> Result<Precipitation> {
    Precipitation::new(mm.max(0.0) as f32)
        .ok_or_else(|| Error::parse(PROVIDER, "invalid precipitation"))
}

fn is_daytime(current: &str, sunrise: &str, sunset: &str) -> bool {
    let Ok(current_time) = parse_time(current) else {
        return true;
    };
    let Ok(sunrise_time) = parse_time(sunrise) else {
        return true;
    };
    let Ok(sunset_time) = parse_time(sunset) else {
        return true;
    };

    current_time >= sunrise_time && current_time < sunset_time
}

fn condition_from_icon(icon: &str) -> WeatherCondition {
    match icon {
        "clear-day" | "clear-night" => WeatherCondition::Clear,
        "partly-cloudy-day" | "partly-cloudy-night" => WeatherCondition::PartlyCloudy,
        "cloudy" => WeatherCondition::Cloudy,
        "fog" => WeatherCondition::Fog,
        "wind" => WeatherCondition::Windy,
        "rain" => WeatherCondition::Rain,
        "showers-day" | "showers-night" => WeatherCondition::LightRain,
        "thunder-rain" | "thunder-showers-day" | "thunder-showers-night" => {
            WeatherCondition::Thunderstorm
        }
        "snow" => WeatherCondition::Snow,
        "snow-showers-day" | "snow-showers-night" => WeatherCondition::LightSnow,
        "sleet" => WeatherCondition::Sleet,
        "hail" => WeatherCondition::Hail,
        _ => WeatherCondition::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_date_valid() {
        let result = parse_date("2024-01-15");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string(), "2024-01-15");
    }

    #[test]
    fn parse_time_with_seconds() {
        let result = parse_time("14:30:00");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string(), "14:30:00");
    }

    #[test]
    fn parse_time_without_seconds() {
        let result = parse_time("14:30");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string(), "14:30:00");
    }

    #[test]
    fn condition_mapping() {
        assert_eq!(condition_from_icon("clear-day"), WeatherCondition::Clear);
        assert_eq!(
            condition_from_icon("partly-cloudy-night"),
            WeatherCondition::PartlyCloudy
        );
        assert_eq!(condition_from_icon("rain"), WeatherCondition::Rain);
        assert_eq!(
            condition_from_icon("thunder-rain"),
            WeatherCondition::Thunderstorm
        );
        assert_eq!(
            condition_from_icon("unknown-icon"),
            WeatherCondition::Unknown
        );
    }

    #[test]
    fn is_daytime_detection() {
        assert!(is_daytime("12:00:00", "06:00:00", "18:00:00"));
        assert!(!is_daytime("05:00:00", "06:00:00", "18:00:00"));
        assert!(!is_daytime("19:00:00", "06:00:00", "18:00:00"));
    }
}
