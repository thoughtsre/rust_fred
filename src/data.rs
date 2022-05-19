use serde::{Serialize, Deserialize, Deserializer, de};
use crate::base::{QueryTraits, ResultTrait, 
    check_units, check_frequency, check_agg_mtd,
    check_output_type};
use crate::client::{FreddoClient};
use std::fs::write;
use serde_json;
use serde_json::Value;

pub struct Query {
    series_id: Option<String>,
    realtime_start: Option<String>,
    realtime_end: Option<String>,
    limit: Option<usize>,
    offset: Option<usize>,
    observation_start: Option<String>,
    observation_end: Option<String>,
    units: Option<String>,
    frequency: Option<String>,
    aggregation_method: Option<String>,
    output_type: Option<usize>,
    vintage_dates: Option<Vec<String>>,
}

impl Query {

    pub fn new() -> Self {
        Self {
            series_id: None,
            realtime_start: None,
            realtime_end: None,
            limit: None,
            offset: None,
            observation_start: None,
            observation_end: None,
            units: None,
            frequency: None,
            aggregation_method: None,
            output_type: None,
            vintage_dates: None,
        }    
    }

    pub fn set_series_id(&mut self, series_id: String) -> &mut Query {
        self.series_id = Some(series_id);

        self
    }

    pub fn set_limit(&mut self, limit: usize) -> &mut Query {

        self.limit = Some(limit);

        self
    }

    pub fn set_offset(&mut self, offset: usize) -> &mut Query {
        
        self.offset = Some(offset);

        self
    }

    pub fn set_realtime_start(&mut self, realtime_start: String) -> &mut Query {
        
        self.realtime_start = Some(realtime_start);

        self
    }

    pub fn set_realtime_end(&mut self, realtime_end: String) -> &mut Query {
        
        self.realtime_end = Some(realtime_end);

        self
    }

    pub fn set_observation_start(&mut self, obs_start: String) -> &mut Query {
        
        self.observation_start = Some(obs_start);

        self
    }

    pub fn set_observation_end(&mut self, obs_end: String) -> &mut Query {
        
        self.observation_end = Some(obs_end);

        self
    }

    pub fn set_units(&mut self, unit: String) -> &mut Query {
        self.units = Some(check_units(unit).unwrap());

        self
    }

    pub fn set_frequency(&mut self, freq: String) -> &mut Query {
        self.frequency = Some(check_frequency(freq).unwrap());

        self
    }

    pub fn set_aggregation(&mut self, agg: String) -> &mut Query {
        self.aggregation_method = Some(check_agg_mtd(agg).unwrap());

        self
    }

    pub fn set_output_type(&mut self, output_type: usize) -> &mut Query {
        self.output_type = Some(check_output_type(output_type).unwrap());

        self
    }

    pub fn set_vintage_dates(&mut self, vintage_dates: Vec<String>) -> &mut Query {
        self.vintage_dates = Some(vintage_dates);

        self
    }

}

impl QueryTraits for Query {
    fn build_query_param_str(&self) -> Result<String, String> {
        let mut q_str = "series/observations?".to_string();

        match &self.series_id {
            Some(val) => {q_str += format!("series_id={}", val).as_str()},
            None => {return Err("No series_id specified!".to_string())},
        };

        if let Some(val) = &self.realtime_start {
            q_str += format!("&realtime_start={}",val).as_str();
        }

        if let Some(val) = &self.realtime_end {
            q_str += format!("&realtime_end={}", val).as_str();
        }

        if let Some(val) = &self.limit {
            q_str += format!("&limit={}", val).as_str();
        }

        if let Some(val) = &self.offset {
            q_str += format!("&offset={}", val).as_str();
        }

        if let Some(val) = &self.observation_start {
            q_str += format!("&observation_start={}",val).as_str();
        }

        if let Some(val) = &self.observation_end {
            q_str += format!("&observation_end={}", val).as_str();
        }

        if let Some(val) = &self.units {
            q_str += format!("&units={}", val).as_str();
        }

        if let Some(val) = &self.frequency {
            q_str += format!("&frequency={}", val).as_str();
        }

        if let Some(val) = &self.aggregation_method {
            q_str += format!("&aggregation_method={}", val).as_str();
        }

        if let Some(val) = &self.output_type {
            q_str += format!("&output_type={}", val).as_str();
        }

        if let Some(val) = &self.vintage_dates {
            q_str += format!("&vintage_dates={}", val.join(",")).as_str();
        }
        
        Ok(q_str)

    }

    /// Execute query
    /// 
    /// # Examples
    /// 
    /// ```
    /// use freddo::data;
    /// use freddo::client::FreddoClient;
    /// use freddo::base::{QueryTraits};
    /// use std::env;
    /// use std::fs;
    /// 
    /// let api_key = fs::read_to_string("API_KEY.txt").expect("Something wrong reading the file!");
    /// env::set_var("FRED_API_KEY", api_key);
    /// let client = FreddoClient::new().unwrap();
    /// 
    /// let mut query = data::Query::new();
    /// query.set_series_id("GNPCA".to_string())
    ///     .set_limit(10);
    /// 
    /// let result = query.execute(&client).unwrap();
    /// result.print_value();
    /// ```
    fn execute(&self, client: &FreddoClient) -> Result<Box<dyn ResultTrait>, String> {

        let response = client.send_query(self);

        match response.status() {
            reqwest::StatusCode::OK => {
                match response.json::<SeriesJSON>() {
                    Ok(json) => {
                        Ok(Box::new(json))
                    },
                    Err(msg) => Err(msg.to_string())
                }
            },
            _ => Err("Something went wrong".to_string())
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
struct DataPoint {
    realtime_start: String,
    realtime_end: String,
    date: String,
    #[serde(deserialize_with="value_as_float")]
    value: f64
}

fn value_as_float<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f64, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => s.parse().map_err(de::Error::custom)?,
        _ => return Err(de::Error::custom("wrong type"))
    })
}

///Data type describing the response of the REST API
#[derive(Serialize, Deserialize, Debug)]
pub struct SeriesJSON {
    realtime_start: String,
    realtime_end: String,
    observation_start: String,
    observation_end: String,
    units: String,
    output_type: usize,
    file_type: String,
    order_by: String,
    sort_order: String,
    count: usize,
    offset: usize,
    limit: usize,
    observations: Vec<DataPoint>
}

impl ResultTrait for SeriesJSON {

    fn print_value(&self) {
        println!("{:?}", self);
    }

    fn write_to_file(&self, output_path: String) {
        write(output_path, serde_json::to_string_pretty(self).unwrap()).unwrap();
    }

}

