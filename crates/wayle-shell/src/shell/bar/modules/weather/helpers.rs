use serde_json::json;
use wayle_config::schemas::modules::TemperatureUnit as ConfigTempUnit;
use wayle_weather::{Temperature, TemperatureUnit, Weather, WeatherCondition};

use crate::i18n::t;

pub(super) struct FormatContext<'a> {
    pub(super) format: &'a str,
    pub(super) weather: &'a Weather,
    pub(super) units: TemperatureUnit,
}

pub(super) fn format_label(ctx: &FormatContext<'_>) -> String {
    let current = &ctx.weather.current;
    let daily_today = ctx.weather.daily.first();

    let temp = format_temp_value(current.temperature, ctx.units);
    let temp_unit = temp_unit_symbol(ctx.units);
    let feels_like = format_temp_value(current.feels_like, ctx.units);
    let condition = condition_label(current.condition);
    let humidity = format!("{}%", current.humidity.get());
    let wind_speed = format_speed(current.wind_speed, ctx.units);
    let wind_dir = current.wind_direction.cardinal();
    let high = daily_today
        .map(|daily| format_temp_value(daily.temp_high, ctx.units))
        .unwrap_or_default();
    let low = daily_today
        .map(|daily| format_temp_value(daily.temp_low, ctx.units))
        .unwrap_or_default();

    let template_ctx = json!({
        "temp": temp,
        "temp_unit": temp_unit,
        "feels_like": feels_like,
        "condition": condition,
        "humidity": humidity,
        "wind_speed": wind_speed,
        "wind_dir": wind_dir,
        "high": high,
        "low": low,
    });
    wayle_common::template::render(ctx.format, template_ctx).unwrap_or_default()
}

pub(crate) fn condition_label(condition: WeatherCondition) -> String {
    match condition {
        WeatherCondition::Clear => t!("weather-clear"),
        WeatherCondition::PartlyCloudy => t!("weather-partly-cloudy"),
        WeatherCondition::Cloudy => t!("weather-cloudy"),
        WeatherCondition::Overcast => t!("weather-overcast"),
        WeatherCondition::Mist => t!("weather-mist"),
        WeatherCondition::Fog => t!("weather-fog"),
        WeatherCondition::LightRain => t!("weather-light-rain"),
        WeatherCondition::Rain => t!("weather-rain"),
        WeatherCondition::HeavyRain => t!("weather-heavy-rain"),
        WeatherCondition::Drizzle => t!("weather-drizzle"),
        WeatherCondition::LightSnow => t!("weather-light-snow"),
        WeatherCondition::Snow => t!("weather-snow"),
        WeatherCondition::HeavySnow => t!("weather-heavy-snow"),
        WeatherCondition::Sleet => t!("weather-sleet"),
        WeatherCondition::Thunderstorm => t!("weather-thunderstorm"),
        WeatherCondition::Windy => t!("weather-windy"),
        WeatherCondition::Hail => t!("weather-hail"),
        WeatherCondition::Unknown => t!("weather-unknown"),
    }
}

fn format_temp_value(temp: Temperature, units: TemperatureUnit) -> String {
    let value = match units {
        TemperatureUnit::Metric => temp.celsius(),
        TemperatureUnit::Imperial => temp.fahrenheit(),
    };
    format!("{value:.0}")
}

fn temp_unit_symbol(units: TemperatureUnit) -> &'static str {
    match units {
        TemperatureUnit::Metric => "°C",
        TemperatureUnit::Imperial => "°F",
    }
}

fn format_speed(speed: wayle_weather::Speed, units: TemperatureUnit) -> String {
    match units {
        TemperatureUnit::Metric => format!("{:.0} km/h", speed.kmh()),
        TemperatureUnit::Imperial => format!("{:.0} mph", speed.mph()),
    }
}

pub(super) fn convert_temp_unit(config_unit: ConfigTempUnit) -> TemperatureUnit {
    match config_unit {
        ConfigTempUnit::Metric => TemperatureUnit::Metric,
        ConfigTempUnit::Imperial => TemperatureUnit::Imperial,
    }
}

pub(super) fn condition_icon(condition: WeatherCondition, is_day: bool) -> &'static str {
    match condition {
        WeatherCondition::Clear if is_day => "ld-sun-symbolic",
        WeatherCondition::Clear => "ld-moon-symbolic",
        WeatherCondition::PartlyCloudy if is_day => "ld-cloud-sun-symbolic",
        WeatherCondition::PartlyCloudy => "ld-cloud-moon-symbolic",
        WeatherCondition::Cloudy => "ld-cloudy-symbolic",
        WeatherCondition::Overcast => "ld-cloud-symbolic",
        WeatherCondition::Mist => "ld-haze-symbolic",
        WeatherCondition::Fog => "ld-cloud-fog-symbolic",
        WeatherCondition::LightRain if is_day => "ld-cloud-sun-rain-symbolic",
        WeatherCondition::LightRain => "ld-cloud-moon-rain-symbolic",
        WeatherCondition::Rain => "ld-cloud-rain-symbolic",
        WeatherCondition::HeavyRain => "ld-cloud-rain-wind-symbolic",
        WeatherCondition::Drizzle => "ld-cloud-drizzle-symbolic",
        WeatherCondition::LightSnow | WeatherCondition::Snow | WeatherCondition::HeavySnow => {
            "ld-cloud-snow-symbolic"
        }
        WeatherCondition::Sleet => "ld-cloud-hail-symbolic",
        WeatherCondition::Thunderstorm => "ld-cloud-lightning-symbolic",
        WeatherCondition::Windy => "ld-wind-symbolic",
        WeatherCondition::Hail => "ld-cloud-hail-symbolic",
        WeatherCondition::Unknown => "ld-cloud-symbolic",
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveTime, Utc};
    use wayle_weather::{
        Astronomy, CurrentWeather, DailyForecast, Location,
        types::{Distance, Percentage, Precipitation, Pressure, Speed, UvIndex, WindDirection},
    };

    use super::*;

    fn sample_weather() -> Weather {
        Weather {
            current: CurrentWeather {
                temperature: Temperature::new(22.5).unwrap(),
                feels_like: Temperature::new(24.0).unwrap(),
                condition: WeatherCondition::PartlyCloudy,
                humidity: Percentage::saturating(65),
                wind_speed: Speed::new(15.0).unwrap(),
                wind_direction: WindDirection::saturating(270),
                wind_gust: Speed::new(25.0).unwrap(),
                uv_index: UvIndex::saturating(5),
                cloud_cover: Percentage::saturating(40),
                pressure: Pressure::new(1013.0).unwrap(),
                visibility: Distance::new(10.0).unwrap(),
                dewpoint: Temperature::new(15.0).unwrap(),
                precipitation: Precipitation::new(0.0).unwrap(),
                is_day: true,
            },
            hourly: vec![],
            daily: vec![DailyForecast {
                date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
                condition: WeatherCondition::PartlyCloudy,
                temp_high: Temperature::new(28.0).unwrap(),
                temp_low: Temperature::new(18.0).unwrap(),
                temp_avg: Temperature::new(23.0).unwrap(),
                humidity_avg: Percentage::saturating(60),
                wind_speed_max: Speed::new(20.0).unwrap(),
                rain_chance: Percentage::saturating(10),
                uv_index_max: UvIndex::saturating(6),
                precipitation_sum: Precipitation::new(0.0).unwrap(),
                sunrise: NaiveTime::from_hms_opt(6, 30, 0).unwrap(),
                sunset: NaiveTime::from_hms_opt(18, 45, 0).unwrap(),
            }],
            location: Location {
                city: "Test City".to_string(),
                region: None,
                country: "Test Country".to_string(),
                lat: 0.0,
                lon: 0.0,
            },
            astronomy: Astronomy {
                sunrise: NaiveTime::from_hms_opt(6, 30, 0).unwrap(),
                sunset: NaiveTime::from_hms_opt(18, 45, 0).unwrap(),
            },
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn format_label_default_format() {
        let weather = sample_weather();
        let result = format_label(&FormatContext {
            format: "{{ temp }}{{ temp_unit }}",
            weather: &weather,
            units: TemperatureUnit::Metric,
        });

        assert_eq!(result, "22°C");
    }

    #[test]
    fn format_label_imperial_units() {
        let weather = sample_weather();
        let result = format_label(&FormatContext {
            format: "{{ temp }}{{ temp_unit }}",
            weather: &weather,
            units: TemperatureUnit::Imperial,
        });

        assert_eq!(result, "72°F");
    }

    #[test]
    fn format_label_with_condition() {
        let weather = sample_weather();
        let result = format_label(&FormatContext {
            format: "{{ temp }}° {{ condition }}",
            weather: &weather,
            units: TemperatureUnit::Metric,
        });

        assert!(result.starts_with("22° "));
        assert!(!result.contains("{{ condition }}"));
    }

    #[test]
    fn format_label_all_placeholders() {
        let weather = sample_weather();
        let result = format_label(&FormatContext {
            format: "{{ temp }}{{ temp_unit }} (feels {{ feels_like }}{{ temp_unit }}) {{ condition }} H:{{ high }} L:{{ low }} {{ humidity }} {{ wind_dir }} {{ wind_speed }}",
            weather: &weather,
            units: TemperatureUnit::Metric,
        });

        assert!(result.starts_with("22°C (feels 24°C) "));
        assert!(result.contains("H:28 L:18 65% W 15 km/h"));
        assert!(!result.contains("{{ condition }}"));
    }

    #[test]
    fn format_label_imperial_wind_speed() {
        let weather = sample_weather();
        let result = format_label(&FormatContext {
            format: "{{ wind_speed }}",
            weather: &weather,
            units: TemperatureUnit::Imperial,
        });

        assert_eq!(result, "9 mph");
    }

    #[test]
    fn condition_icon_clear_day_vs_night() {
        assert_eq!(
            condition_icon(WeatherCondition::Clear, true),
            "ld-sun-symbolic"
        );
        assert_eq!(
            condition_icon(WeatherCondition::Clear, false),
            "ld-moon-symbolic"
        );
    }

    #[test]
    fn condition_icon_partly_cloudy_day_vs_night() {
        assert_eq!(
            condition_icon(WeatherCondition::PartlyCloudy, true),
            "ld-cloud-sun-symbolic"
        );
        assert_eq!(
            condition_icon(WeatherCondition::PartlyCloudy, false),
            "ld-cloud-moon-symbolic"
        );
    }

    #[test]
    fn condition_icon_light_rain_day_vs_night() {
        assert_eq!(
            condition_icon(WeatherCondition::LightRain, true),
            "ld-cloud-sun-rain-symbolic"
        );
        assert_eq!(
            condition_icon(WeatherCondition::LightRain, false),
            "ld-cloud-moon-rain-symbolic"
        );
    }

    #[test]
    fn condition_icon_cloudy_same_day_and_night() {
        let day = condition_icon(WeatherCondition::Cloudy, true);
        let night = condition_icon(WeatherCondition::Cloudy, false);
        assert_eq!(day, night);
        assert_eq!(day, "ld-cloudy-symbolic");
    }

    #[test]
    fn condition_icon_all_snow_variants_same() {
        let light = condition_icon(WeatherCondition::LightSnow, true);
        let moderate = condition_icon(WeatherCondition::Snow, true);
        let heavy = condition_icon(WeatherCondition::HeavySnow, true);
        assert_eq!(light, moderate);
        assert_eq!(moderate, heavy);
        assert_eq!(light, "ld-cloud-snow-symbolic");
    }

    #[test]
    fn convert_temp_unit_metric() {
        assert_eq!(
            convert_temp_unit(ConfigTempUnit::Metric),
            TemperatureUnit::Metric
        );
    }

    #[test]
    fn convert_temp_unit_imperial() {
        assert_eq!(
            convert_temp_unit(ConfigTempUnit::Imperial),
            TemperatureUnit::Imperial
        );
    }
}
