use std::time::Duration;
use std::env;
use reqwest;
use reqwest::blocking::{Client, Response};
use crate::base::{QueryTraits};

const FRED_BASE_URL: &str = "https://api.stlouisfed.org/fred/";

pub struct FreddoClient {
    client: Client,
    base_url: &'static str,
    api_key: String,
}

impl FreddoClient {
    pub fn new() -> Result<Self, String> {

        let client = match Client::builder().timeout(Duration::from_secs(10)).build() {
            Ok(c) => c,
            Err(msg) => return Err(msg.to_string()),
        };

        let base_url = FRED_BASE_URL;

        let api_key = match env::var("FRED_API_KEY") {
            Ok(key) => key,
            Err(_) => return Err("No API Key Found! Please set FRED_API_KEY env var.".to_string()),
        };

        Ok(Self {
            client,
            base_url,
            api_key
        })

    }

    pub fn get_client(&self) -> &Client {
        &self.client
    }

    pub fn get_query_str(&self, query_params:String) -> String {
        format!("{}/{}&api_key={}&file_type=json", self.base_url, query_params, self.api_key)
    }

    pub fn send_query(&self, query: &impl QueryTraits) -> Response {

        let query_params = query.build_query_param_str().unwrap();

        let query_str = self.get_query_str(query_params);
        
        let response = self.client
            .get(query_str)
            .send()
            .unwrap();

        response
    }
}
