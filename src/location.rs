use clap::{crate_name, crate_version};
use reqwest::blocking::ClientBuilder;
use serde::Deserialize;

pub struct Location {
    pub display_name: String,
    pub lat: f32,
    pub lon: f32,
}

impl Location {
    pub fn from_name(
        city: String,
        country: String,
        state: Option<String>,
        postal_code: Option<String>,
    ) -> Location {
        let mut params = vec![
            ("city", city),
            ("country", country),
            ("format", "jsonv2".to_string()),
        ];
        if let Some(s) = state {
            params.push(("state", s.clone()));
        }
        if let Some(pc) = postal_code {
            params.push(("postalcode", pc.clone()));
        }

        let resp = ClientBuilder::new()
            .build()
            .unwrap()
            .get("https://nominatim.openstreetmap.org/search")
            .query(&params)
            .header("Accept-Language", "en-US,en;q=0.9")
            .header(
                "User-Agent",
                format!("{} {}", crate_name!(), crate_version!()),
            )
            .send()
            .expect("Unable to convert location to coordinates");
        if let Err(status) = resp.error_for_status_ref() {
            panic!("{}", status);
        }

        let locations = resp
            .json::<Vec<NominatimResponse>>()
            .expect("Encountered error while parsing Nominatim response");

        if locations.is_empty() {
            panic!("No matching locations found!");
        } else {
            let loc = &locations[0];
            Location {
                display_name: loc.display_name.clone(),
                lat: loc.lat.parse().unwrap(),
                lon: loc.lon.parse().unwrap(),
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct NominatimResponse {
    place_id: u32,
    licence: String,
    osm_type: String,
    osm_id: u32,
    lat: String,
    lon: String,
    category: String,
    #[serde(rename = "type")]
    typ: String,
    place_rank: u8,
    importance: f32,
    #[serde(rename = "addresstype")]
    address_type: String,
    name: String,
    display_name: String,
    #[serde(rename = "boundingbox")]
    bounding_box: Vec<String>,
}
