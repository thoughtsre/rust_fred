use reqwest;
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct SeriesInfo {
    id: String,
    realtime_start: String,
    realtime_end: String,
    title: String,
    observation_start: String,
    observation_end: String,
    frequency: String,
    frequency_short: String,
    units: String,
    units_short: String,
    seasonal_adjustment: String,
    seasonal_adjustment_short: String,
    last_updated: String,
    popularity: i32,
    notes: String
}

#[derive(Serialize, Deserialize, Debug)]
struct SeriesJSON {
    realtime_start: String,
    realtime_end: String,
    seriess: Vec<SeriesInfo>
}

fn main() {
    
    let client = reqwest::blocking::Client::new();

    let API_KEY = fs::read_to_string("API_KEY.txt").expect("Something wrong reading the file!");

    let URL = format!("https://api.stlouisfed.org/fred/series?series_id=GNPCA&api_key={}&file_type=json", API_KEY);

    let response = client
        .get(URL)
        .send()
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<SeriesJSON>() {
                Ok(json) => println!("{:?}", json),
                Err(_) => panic!("Error!")
            }
        }

        _ => {
            println!("Something went wrong.")
        }
    }

    
}
