use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};

use super::types::{ApiResponse, ForecastDay, HourData};
use crate::{
    error::{Error, Result},
    model::{CurrentWeather, DailyForecast, HourlyForecast, WeatherCondition},
    types::{
        Distance, Percentage, Precipitation, Pressure, Speed, Temperature, UvIndex, WindDirection,
    },
};

pub const PROVIDER: &str = "weatherapi";

pub fn build_current(data: &ApiResponse) -> Result<CurrentWeather> {
    let current = &data.current;

    Ok(CurrentWeather {
        temperature: temperature(current.temp_c)?,
        feels_like: temperature(current.feelslike_c)?,
        condition: condition_from_code(current.condition.code),
        humidity: percentage(current.humidity),
        wind_speed: speed(current.wind_kph)?,
        wind_direction: wind_dir(current.wind_degree),
        wind_gust: speed(current.gust_kph)?,
        uv_index: uv(current.uv),
        cloud_cover: percentage(current.cloud),
        pressure: pressure(current.pressure_mb)?,
        visibility: visibility(current.vis_km)?,
        dewpoint: temperature(current.dewpoint_c)?,
        precipitation: precip(current.precip_mm)?,
        is_day: current.is_day == 1,
    })
}

pub fn build_hourly(data: &ApiResponse, count: usize) -> Result<Vec<HourlyForecast>> {
    let now = Utc::now();
    let mut forecasts = Vec::with_capacity(count);
    let mut collected = 0;

    for day in &data.forecast.forecastday {
        for hour in &day.hour {
            if collected >= count {
                break;
            }

            let datetime = parse_datetime(&hour.time)?;
            if datetime < now {
                continue;
            }

            forecasts.push(build_hourly_forecast(hour, datetime)?);
            collected += 1;
        }

        if collected >= count {
            break;
        }
    }

    Ok(forecasts)
}

fn build_hourly_forecast(hour: &HourData, datetime: DateTime<Utc>) -> Result<HourlyForecast> {
    Ok(HourlyForecast {
        time: datetime,
        temperature: temperature(hour.temp_c)?,
        feels_like: temperature(hour.feelslike_c)?,
        condition: condition_from_code(hour.condition.code),
        humidity: percentage(hour.humidity),
        wind_speed: speed(hour.wind_kph)?,
        wind_direction: wind_dir(hour.wind_degree),
        wind_gust: speed(hour.gust_kph)?,
        rain_chance: percentage(hour.chance_of_rain),
        uv_index: uv(hour.uv),
        cloud_cover: percentage(hour.cloud),
        pressure: pressure(hour.pressure_mb)?,
        visibility: visibility(hour.vis_km)?,
        dewpoint: temperature(hour.dewpoint_c)?,
        precipitation: precip(hour.precip_mm)?,
        is_day: hour.is_day == 1,
    })
}

pub fn build_daily(data: &ApiResponse, count: usize) -> Result<Vec<DailyForecast>> {
    let mut forecasts = Vec::with_capacity(count);

    for day in data.forecast.forecastday.iter().take(count) {
        forecasts.push(build_daily_forecast(day)?);
    }

    Ok(forecasts)
}

fn build_daily_forecast(fd: &ForecastDay) -> Result<DailyForecast> {
    let day_data = &fd.day;
    let date = parse_date(&fd.date)?;
    let sunrise = parse_12h_time(&fd.astro.sunrise)?;
    let sunset = parse_12h_time(&fd.astro.sunset)?;

    Ok(DailyForecast {
        date,
        condition: condition_from_code(day_data.condition.code),
        temp_high: temperature(day_data.maxtemp_c)?,
        temp_low: temperature(day_data.mintemp_c)?,
        temp_avg: temperature(day_data.avgtemp_c)?,
        humidity_avg: percentage(day_data.avghumidity),
        wind_speed_max: speed(day_data.maxwind_kph)?,
        rain_chance: percentage(day_data.daily_chance_of_rain),
        uv_index_max: uv(day_data.uv),
        precipitation_sum: precip(day_data.totalprecip_mm)?,
        sunrise,
        sunset,
    })
}

fn parse_date(s: &str) -> Result<NaiveDate> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| Error::parse(PROVIDER, e.to_string()))
}

fn parse_datetime(s: &str) -> Result<DateTime<Utc>> {
    let naive = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M")
        .map_err(|e| Error::parse(PROVIDER, e.to_string()))?;
    Ok(Utc.from_utc_datetime(&naive))
}

fn parse_12h_time(s: &str) -> Result<NaiveTime> {
    NaiveTime::parse_from_str(s.trim(), "%I:%M %p")
        .or_else(|_| NaiveTime::parse_from_str(s.trim(), "%I:%M %P"))
        .map_err(|e| Error::parse(PROVIDER, e.to_string()))
}

fn temperature(celsius: f64) -> Result<Temperature> {
    Temperature::new(celsius as f32).ok_or_else(|| Error::parse(PROVIDER, "invalid temperature"))
}

fn percentage(percent: f64) -> Percentage {
    Percentage::saturating(percent.clamp(0.0, 100.0) as u8)
}

fn speed(kph: f64) -> Result<Speed> {
    Speed::new(kph.max(0.0) as f32).ok_or_else(|| Error::parse(PROVIDER, "invalid speed"))
}

fn wind_dir(degrees: f64) -> WindDirection {
    WindDirection::saturating(degrees.max(0.0) as u16)
}

fn uv(index: f64) -> UvIndex {
    UvIndex::saturating(index.clamp(0.0, 15.0) as u8)
}

fn pressure(mb: f64) -> Result<Pressure> {
    Pressure::new(mb.max(0.0) as f32).ok_or_else(|| Error::parse(PROVIDER, "invalid pressure"))
}

fn visibility(km: f64) -> Result<Distance> {
    Distance::new(km.max(0.0) as f32).ok_or_else(|| Error::parse(PROVIDER, "invalid visibility"))
}

fn precip(mm: f64) -> Result<Precipitation> {
    Precipitation::new(mm.max(0.0) as f32)
        .ok_or_else(|| Error::parse(PROVIDER, "invalid precipitation"))
}

fn condition_from_code(code: i32) -> WeatherCondition {
    match code {
        1000 => WeatherCondition::Clear,
        1003 => WeatherCondition::PartlyCloudy,
        1006 => WeatherCondition::Cloudy,
        1009 => WeatherCondition::Overcast,
        1030 | 1135 | 1147 => WeatherCondition::Mist,
        1021 | 1024 | 1027 => WeatherCondition::Fog,
        1051 | 1054 | 1057 | 1168 | 1171 => WeatherCondition::Drizzle,
        1063 | 1150 | 1153 | 1180 | 1183 => WeatherCondition::LightRain,
        1186 | 1189 | 1261 | 1264 => WeatherCondition::Rain,
        1192 | 1195 | 1240 | 1243 | 1246 | 1267 | 1270 => WeatherCondition::HeavyRain,
        1066 | 1210 | 1213 | 1255 => WeatherCondition::LightSnow,
        1039 | 1216 | 1219 => WeatherCondition::Snow,
        1114 | 1015 | 1042 | 1045 | 1048 | 1117 | 1222 | 1225 | 1258 => WeatherCondition::HeavySnow,
        1069 | 1072 | 1198 | 1201 | 1204 | 1207 | 1237 | 1249 | 1252 => WeatherCondition::Sleet,
        1087 | 1273 | 1276 | 1279 | 1282 => WeatherCondition::Thunderstorm,
        _ => WeatherCondition::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use chrono::Timelike;

    use super::*;

    #[test]
    fn parse_date_valid() {
        let result = parse_date("2024-01-15");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string(), "2024-01-15");
    }

    #[test]
    fn parse_datetime_valid() {
        let result = parse_datetime("2024-01-15 14:30");
        assert!(result.is_ok());
    }

    #[test]
    fn parse_12h_time_am() {
        let time = parse_12h_time("06:30 AM").unwrap();
        assert_eq!(time.hour(), 6);
        assert_eq!(time.minute(), 30);
    }

    #[test]
    fn parse_12h_time_pm() {
        let time = parse_12h_time("07:45 PM").unwrap();
        assert_eq!(time.hour(), 19);
        assert_eq!(time.minute(), 45);
    }

    #[test]
    fn condition_code_mapping() {
        assert_eq!(condition_from_code(1000), WeatherCondition::Clear);
        assert_eq!(condition_from_code(1003), WeatherCondition::PartlyCloudy);
        assert_eq!(condition_from_code(1195), WeatherCondition::HeavyRain);
        assert_eq!(condition_from_code(1276), WeatherCondition::Thunderstorm);
        assert_eq!(condition_from_code(9999), WeatherCondition::Unknown);
    }

    #[test]
    fn condition_code_mist_variants() {
        assert_eq!(condition_from_code(1030), WeatherCondition::Mist);
        assert_eq!(condition_from_code(1135), WeatherCondition::Mist);
        assert_eq!(condition_from_code(1147), WeatherCondition::Mist);
    }

    #[test]
    fn condition_code_sleet_variants() {
        assert_eq!(condition_from_code(1069), WeatherCondition::Sleet);
        assert_eq!(condition_from_code(1072), WeatherCondition::Sleet);
        assert_eq!(condition_from_code(1198), WeatherCondition::Sleet);
        assert_eq!(condition_from_code(1201), WeatherCondition::Sleet);
        assert_eq!(condition_from_code(1237), WeatherCondition::Sleet);
    }

    #[test]
    fn condition_code_snow_light_moderate_heavy() {
        assert_eq!(condition_from_code(1066), WeatherCondition::LightSnow);
        assert_eq!(condition_from_code(1210), WeatherCondition::LightSnow);
        assert_eq!(condition_from_code(1216), WeatherCondition::Snow);
        assert_eq!(condition_from_code(1039), WeatherCondition::Snow);
        assert_eq!(condition_from_code(1114), WeatherCondition::HeavySnow);
        assert_eq!(condition_from_code(1117), WeatherCondition::HeavySnow);
        assert_eq!(condition_from_code(1225), WeatherCondition::HeavySnow);
    }
}
