use serde::{Serialize, Deserialize};
use crate::base::{QueryTraits, ResultTrait};
use crate::client::{FreddoClient};
use std::fs::write;

pub struct Query {
    series_id: Option<String>,
    realtime_start: Option<String>,
    realtime_end: Option<String>,
    limit: Option<usize>,
    offset: Option<usize>

}

impl Query {

    pub fn new() -> Self {
        Self {
            series_id: None,
            realtime_start: None,
            realtime_end: None,
            limit: None,
            offset: None
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

    fn build_series_info_query_str(&self) -> Result<String, String> {
        let mut q_str = "series?".to_string();

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

        Ok(q_str)
    }

    fn build_categories_query_str(&self) -> Result<String, String> {
        let mut q_str = "series/categories?".to_string();

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

        Ok(q_str)
    }

    fn build_tags_query_str(&self) -> Result<String, String> {
        let mut q_str = "series/tags?".to_string();

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

        Ok(q_str)
    }

    fn build_vintage_dates_query_str(&self) -> Result<String, String> {
        let mut q_str = "series/vintagedates?".to_string();

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

        Ok(q_str)
    }
}

impl QueryTraits for Query {

    fn build_query_param_str(&self) -> Result<String, String> {

        Ok("Not implemented".to_string())

    }

    /// Execute query
    /// 
    /// # Examples
    /// 
    /// ```
    /// use freddo::info;
    /// use freddo::client::FreddoClient;
    /// use freddo::base::{QueryTraits};
    /// use std::env;
    /// use std::fs;
    /// 
    /// let api_key = fs::read_to_string("API_KEY.txt").expect("Something wrong reading the file!");
    /// env::set_var("FRED_API_KEY", api_key);
    /// let client = FreddoClient::new().unwrap();
    /// 
    /// let mut query = info::Query::new();
    /// query.set_series_id("GNPCA".to_string())
    ///     .set_limit(10);
    /// 
    /// let result = query.execute(&client).unwrap();
    /// result.print_value();
    /// ```
    fn execute(&self, client: &FreddoClient) -> Result<Box<dyn ResultTrait>, String> {

        let info_query = client.get_query_str(self.build_series_info_query_str().unwrap());

        let info_response = client.get_client()
            .get(info_query)
            .send()
            .unwrap()
            .json::<SeriesInfo>()
            .unwrap();

        let cat_query = client.get_query_str(self.build_categories_query_str().unwrap());

        let cat_response = client.get_client()
            .get(cat_query)
            .send()
            .unwrap()
            .json::<CategoriesInfo>()
            .unwrap();

        let tag_query = client.get_query_str(self.build_tags_query_str().unwrap());

        let tag_response = client.get_client()
            .get(tag_query)
            .send()
            .unwrap()
            .json::<TagsInfo>()
            .unwrap();

        let vintage_date_query = client.get_query_str(self.build_vintage_dates_query_str().unwrap());

        let vintage_date_response = client.get_client()
            .get(vintage_date_query)
            .send()
            .unwrap()
            .json::<VintageDateInfo>()
            .unwrap();

        let final_result = FinalResults {
            realtime_start: info_response.realtime_start,
            realtime_end: info_response.realtime_end,
            info: info_response.seriess,
            categories: cat_response.categories,
            tags: tag_response.tags,
            vintage_dates: vintage_date_response.vintage_dates
        };

        Ok(Box::new(final_result))
    }
}

///Data type describing the response of the REST API
#[derive(Serialize, Deserialize, Debug)]
struct SeriesInfo {
    realtime_start: String,
    realtime_end: String,
    seriess: Vec<BasicInfo>
}

#[derive(Serialize, Deserialize, Debug)]
struct BasicInfo {
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
    notes: String
}

#[derive(Serialize, Deserialize, Debug)]
struct CategoriesInfo {
    categories: Vec<Category>
}

#[derive(Serialize, Deserialize, Debug)]
struct Category {
    id: usize,
    name: String,
    parent_id: usize
}

#[derive(Serialize, Deserialize, Debug)]
struct TagsInfo {
    realtime_start: String,
    realtime_end: String,
    order_by: String,
    sort_order: String,
    count: usize,
    offset: usize,
    limit: usize,
    tags: Vec<Tag>
}

#[derive(Serialize, Deserialize, Debug)]
struct Tag {
    name: String,
    group_id: String,
    notes: Option<String>,
    created: Option<String>,
    popularity: usize,
    series_count: usize
}

#[derive(Serialize, Deserialize, Debug)]
struct VintageDateInfo {
    realtime_start: String,
    realtime_end: String,
    order_by: String,
    sort_order: String,
    count: usize,
    offset: usize,
    limit: usize,
    vintage_dates: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct FinalResults {
    realtime_start: String,
    realtime_end: String,
    info: Vec<BasicInfo>,
    categories: Vec<Category>,
    tags: Vec<Tag>,
    vintage_dates: Vec<String>
}

impl ResultTrait for FinalResults {

    fn print_value(&self) {
        println!("{:?}", self);
    }

    fn write_to_file(&self, output_path: String) {
        write(output_path, serde_json::to_string_pretty(self).unwrap()).unwrap();
    }

}