use std::env;
use std::fs;
use freddo::{data, search, info};
use freddo::client::FreddoClient;
use freddo::base::{QueryTraits};

fn main() {

    let api_key = fs::read_to_string("API_KEY.txt").expect("Something wrong reading the file!");

    env::set_var("FRED_API_KEY", api_key);

    let client = FreddoClient::new().unwrap();

    // let mut query = data::Query::new();
    
    // query.series_id("GNPCA".to_string());

    // let result = query.execute(&client).unwrap();

    // result.write_to_file("test.json".to_string());

    // let result = client.get_data(query);

    // println!("{:?}", result.unwrap());

    // let mut query = search::Query::new();

    // query.set_search_text(vec!["GDP".to_owned(), "energy".to_owned()])
    //     .set_limit(10);

    let mut query = info::Query::new();
    query.set_series_id("GNPCA".to_string())
    .set_limit(10);

    let result = query.execute(&client).unwrap();

    result.print_value();

    result.write_to_file("test.json".to_string());

    
}

