use freddo::{data, search, info};
use freddo::client::FreddoClient;
use freddo::base::{QueryTraits};

/// This function searches for info related to `GNPCA`
fn main() {

    let client = FreddoClient::new().unwrap();

    let mut query = info::Query::new();
    query.set_series_id("GNPCA".to_string())
    .set_limit(10);

    let result = query.execute(&client).unwrap();

    result.print_value();

    result.write_to_file("test.json".to_string());

    
}

