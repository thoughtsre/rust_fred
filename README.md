# Freddo

This is an opinionated library for accessing the Federal Reserve of St Louis data API. It may not implement all the functionalities of the API.

For a like-for-like implementation of the API in Rust, see [`fred-rs`](https://crates.io/crates/fred-rs).

## Pre-requisites
- You will need an API key that can be generated [here](https://fred.stlouisfed.org/docs/api/api_key.html)
- To use the library, store the API key as the environment variable `FRED_API_KEY`

## General structure
- A `client` stores the API key and the base URLs that are needed to interact with the API
- The use of the library starts with the construction of a `query`
- Each `query` belongs to one of the three types:
    + [`data`](data.rs) which obtains the actual observations
    + [`search`](search.rs) which allows the user to search for data series using key words
    + [`info`](info.rs) which obtains the metadata related to a data series
- Each query has an `execute` method to carry out the request