use serde::{Serialize, Deserialize};
use crate::base::{QueryTraits, ResultTrait, check_order_by,
    check_sort_order, check_filter_variable};
use crate::client::{FreddoClient};
use std::fs::write;
use serde_json;

pub struct Query {
    search_text: Option<String>,
    search_type: Option<String>,
    realtime_start: Option<String>,
    realtime_end: Option<String>,
    limit: Option<usize>,
    offset: Option<usize>,
    order_by: Option<String>,
    sort_order: Option<String>,
    filter_variable: Option<String>,
    filter_value: Option<String>,
    tag_names: Option<Vec<String>>,
    exclude_tag_names: Option<Vec<String>>,
}

impl Query {

    pub fn new() -> Self {
        Self {
            search_text: None,
            search_type: None,
            realtime_start: None,
            realtime_end: None,
            limit: None,
            offset: None,
            order_by: None,
            sort_order: None,
            filter_variable: None,
            filter_value: None,
            tag_names: None,
            exclude_tag_names: None,
        }    
    }

    pub fn set_search_text(&mut self, search_text: Vec<String>) -> &mut Query {
        self.search_text = Some(search_text.join("%20"));
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

    pub fn set_order(&mut self, order_by: String) -> &mut Query {
        self.order_by = Some(check_order_by(order_by).unwrap());

        self
    }

    pub fn set_sort(&mut self, sort_order: String) -> &mut Query {
        self.sort_order = Some(check_sort_order(sort_order).unwrap());

        self
    }

    pub fn set_filter_variable(&mut self, filter_var: String) -> &mut Query {
        self.filter_variable = Some(check_filter_variable(filter_var).unwrap());

        self
    }

    pub fn set_filter_value(&mut self, filter_val: String) -> &mut Query {
        match self.filter_variable {
            Some(_) => {
                self.filter_value = Some(filter_val)
            },
            None => {
                println!("filter variable must be set first!");
            }
        };

        self
    }

    pub fn set_tags(&mut self, tags: Vec<String>) -> &mut Query {
        self.tag_names = Some(tags);

        self
    }

    pub fn exclude_tags(&mut self, tags: Vec<String>) -> &mut Query {
        self.exclude_tag_names = Some(tags);

        self
    }
}

impl QueryTraits for Query {
    fn build_query_param_str(&self) -> Result<String, String> {
        let mut q_str = "series/search?".to_string();

        match &self.search_text {
            Some(val) => {q_str += format!("search_text={}", val).as_str()},
            None => {return Err("No search text specified!".to_string())},
        };

        if let Some(val) = &self.search_type {
            q_str += format!("&search_type={}",val).as_str();
        }

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

        if let Some(val) = &self.order_by {
            q_str += format!("&order_by={}", val).as_str();
        }

        if let Some(val) = &self.sort_order {
            q_str += format!("&sort_order={}", val).as_str();
        }

        if let Some(val) = &self.filter_variable {
            q_str += format!("&filter_variable={}", val).as_str();
        }

        if let Some(val) = &self.filter_value {
            q_str += format!("&filter_value={}", val).as_str();
        }

        if let Some(val) = &self.tag_names {
            q_str += format!("&tag_names={}", val.join(";")).as_str();
        }

        if let Some(val) = &self.exclude_tag_names {
            q_str += format!("&exclude_tag_names={}", val.join(";")).as_str();
        }
        
        Ok(q_str)
    }

    /// Execute query
    /// 
    /// # Examples
    /// 
    /// ```
    /// use freddo::search;
    /// use freddo::client::FreddoClient;
    /// use freddo::base::{QueryTraits};
    /// use std::env;
    /// use std::fs;
    /// 
    /// let api_key = fs::read_to_string("API_KEY.txt").expect("Something wrong reading the file!");
    /// env::set_var("FRED_API_KEY", api_key);
    /// let client = FreddoClient::new().unwrap();
    /// 
    /// let mut query = search::Query::new();
    /// query.set_search_text(vec!["GDP".to_owned(), "energy".to_owned()])
    /// .set_limit(1);
    /// 
    /// let result = query.execute(&client).unwrap();
    /// result.print_value();
    /// ```
    fn execute(&self, client: &FreddoClient) -> Result<Box<dyn ResultTrait>, String> {

        let response = client.send_query(self);

        match response.status() {
            reqwest::StatusCode::OK => {
                match response.json::<SearchJSON>() {
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
    popularity: usize,
    group_popularity: usize,
    notes: String
}

///Data type describing the response of the REST API
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchJSON {
    realtime_start: String,
    realtime_end: String,
    order_by: String,
    sort_order: String,
    count: usize,
    offset: usize,
    limit: usize,
    seriess: Vec<SeriesInfo>
}

impl ResultTrait for SearchJSON {

    fn print_value(&self) {
        println!("{:?}", self);
    }

    fn write_to_file(&self, output_path: String) {
        write(output_path, serde_json::to_string_pretty(self).unwrap()).unwrap();
    }

}