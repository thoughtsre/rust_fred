use crate::client::FreddoClient;

pub trait QueryTraits {
    fn build_query_param_str(&self) -> Result<String, String>;

    fn execute(&self, client: &FreddoClient) -> Result<Box<dyn ResultTrait>, String>;
}

pub trait ResultTrait {

    fn print_value(&self);

    fn write_to_file(&self, output_path: String);

}

pub fn check_units(unit: String) -> Result<String, String> {
    let allowed_units: Vec<&str> = vec!["lin", "chg", "ch1",
        "pch", "pc1", "pca", 
        "cch", "cca", "log"];

    if allowed_units.contains(&unit.to_lowercase().as_str()) {
        Ok(unit.to_lowercase())
    } else {
        Err("Invalid unit specified!".to_owned())
    }

}

pub fn check_frequency(freq: String) -> Result<String, String> {

    let allowed_freqs: Vec<&str> = vec!["d", "w", "bw", "m", "q", 
        "sa", "a", "wef", "weth", 
        "wew", "wetu", "wem", "wesu", 
        "wesa", "bwew", "bwem"];

    if allowed_freqs.contains(&freq.to_lowercase().as_str()) {
        Ok(freq.to_lowercase())
    } else {
        Err("Invalid frequency specified!".to_owned())
    }

}

pub fn check_agg_mtd(method: String) -> Result<String, String> {

    let allowed_methods: Vec<&str> = vec!["avg", "sum", "eop"];

    if allowed_methods.contains(&method.to_lowercase().as_str()) {
        Ok(method.to_lowercase())
    } else {
        Err("Invalid method specified!".to_owned())
    }
}

pub fn check_output_type(output_type: usize) -> Result<usize, String> {

    let allowed_types: Vec<usize> = vec![1, 2, 3, 4];

    if allowed_types.contains(&output_type) {
        Ok(output_type)
    } else {
        Err("Invalid output type specified!".to_owned())
    }
}

pub fn check_search_type(search_type: String) -> Result<String, String> {

    let allowed_types: Vec<&str> = vec!["full_text", "series_id"];

    if allowed_types.contains(&search_type.to_lowercase().as_str()) {
        Ok(search_type.to_lowercase())
    } else {
        Err("Invalid search type specified!".to_owned())
    }
}

pub fn check_order_by(order_by: String) -> Result<String, String> {

    let allowed_order: Vec<&str> = vec!["search_rank", "series_id", "title", "units", 
        "frequency", "seasonal_adjustment", "realtime_start", "realtime_end", 
        "last_updated", "observation_start", "observation_end", "popularity", 
        "group_popularity"];

    if allowed_order.contains(&order_by.to_lowercase().as_str()) {
        Ok(order_by.to_lowercase())
    } else {
        Err("Invalid order specified!".to_owned())
    }
}

pub fn check_sort_order(sort_order: String) -> Result<String, String> {

    let allowed_sort: Vec<&str> = vec!["asc", "desc"];

    if allowed_sort.contains(&sort_order.to_lowercase().as_str()) {
        Ok(sort_order.to_lowercase())
    } else {
        Err("Invalid sort order specified!".to_owned())
    }
}

pub fn check_filter_variable(filter_var: String) -> Result<String, String> {
    
    let allowed_filter: Vec<&str> = vec!["frequency", "units", "seasonal_adjustment"];

    if allowed_filter.contains(&filter_var.to_lowercase().as_str()) {
        Ok(filter_var.to_lowercase())
    } else {
        Err("Invalid filter variable specified!".to_owned())
    }
}

#[cfg(test)]
mod base_tests {

    use super::*;

    fn incorrect_param<T>(f: fn(T) -> Result<T, String>, val:T) {
        assert!(f(val).is_err())
    }

    #[test]
    fn incorrect_units() {
        incorrect_param(check_units, "abs".to_string())
    }

    #[test]
    fn incorrect_freqs() {
        incorrect_param(check_frequency, "hahaha".to_string())
    }

    #[test]
    fn incorrect_agg_mtd() {
        incorrect_param(check_agg_mtd, "nonono".to_string())
    }

    #[test]
    fn incorrect_output_type() {
        incorrect_param(check_output_type, 6)
    }

    #[test]
    fn incorrect_search_type() {
        incorrect_param(check_search_type, "i don't Know".to_string())
    }

    #[test]
    fn incorrect_order(){
        incorrect_param(check_order_by, "no order".to_string())
    }

    #[test]
    fn incorrect_sort(){
        incorrect_param(check_sort_order, "steady".to_string())
    }

    #[test]
    fn incorrect_filter_var() {
        incorrect_param(check_filter_variable, "today".to_string())
    }
}