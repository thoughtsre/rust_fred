//! Freddo is an opinionated way to access the Federal Reserve of St. Louis Data API.
//! 
//! Everything starts by instantiating a *Query*. 
//! 
//! There are **3 types of queries**.
//! 1. [`data`](data): Gets observation data from the API
//! 2. [`search`](search): Search for data series via search text
//! 3. [`info`](info): Gets metadata associated with a data series
//! 
//! Calling the `execute` method on a `query` will send the REST request via a [`client`](client) which stores session parameters and the API key.
pub mod client;
pub mod data;
pub mod search;
pub mod info;
pub mod base;