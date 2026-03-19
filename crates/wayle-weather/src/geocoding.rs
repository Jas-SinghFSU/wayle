use serde::Deserialize;

use crate::{
    error::{Error, Result},
    model::{Location, LocationQuery},
};

const GEOCODING_URL: &str = "https://geocoding-api.open-meteo.com/v1/search";
const PROVIDER: &str = "geocoding";

/// Resolves a location query to a concrete [`Location`] with coordinates.
///
/// For coordinate queries, returns a minimal `Location` with lat/lon filled
/// and empty name fields. For city queries, performs forward geocoding via
/// the Open-Meteo geocoding API.
///
/// # Errors
///
/// Returns [`Error::LocationNotFound`] if the city query produces no results.
/// Returns [`Error::Http`] or [`Error::Parse`] on network or deserialization failures.
pub(crate) async fn resolve(client: &reqwest::Client, query: &LocationQuery) -> Result<Location> {
    match query {
        LocationQuery::Coordinates { lat, lon } => Ok(Location {
            city: String::new(),
            region: None,
            country: String::new(),
            lat: *lat,
            lon: *lon,
        }),
        LocationQuery::City { name, country } => {
            forward_geocode(client, name, country.as_deref()).await
        }
    }
}

async fn forward_geocode(
    client: &reqwest::Client,
    name: &str,
    country: Option<&str>,
) -> Result<Location> {
    let mut request = client
        .get(GEOCODING_URL)
        .query(&[("name", name), ("count", "1")]);

    if let Some(country_code) = country {
        request = request.query(&[("countryCode", country_code)]);
    }

    let resp = request
        .send()
        .await
        .map_err(|err| Error::http(PROVIDER, err))?;

    if !resp.status().is_success() {
        return Err(Error::status(PROVIDER, resp.status()));
    }

    let geo: GeocodingResponse = resp
        .json()
        .await
        .map_err(|err| Error::parse(PROVIDER, err.to_string()))?;

    let result = geo
        .results
        .into_iter()
        .next()
        .ok_or_else(|| Error::LocationNotFound {
            query: name.to_owned(),
        })?;

    Ok(Location {
        city: result.name,
        region: result.admin1,
        country: result.country,
        lat: result.latitude,
        lon: result.longitude,
    })
}

#[derive(Debug, Deserialize)]
struct GeocodingResponse {
    #[serde(default)]
    results: Vec<GeocodingResult>,
}

#[derive(Debug, Deserialize)]
struct GeocodingResult {
    latitude: f64,
    longitude: f64,
    name: String,
    admin1: Option<String>,
    country: String,
}
