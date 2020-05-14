use crate::utils::*;
use serde_json::{Value};
use actix_web::{HttpResponse};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize)]
struct Response {
    confirmed: i32,
    deaths: i32,
    recovered: i32
}

pub async fn fetch() -> HttpResponse {
    let url = Url::parse_with_params(&endpoint::level("county"), 
    &[
        ("f", "json"), 
        ("where", "1=1"),
        ("cacheHint", "true"),
        ("outStatistics", &query::sum_statistics(&"confirmed")),
    ]).unwrap();
    let get_confirmed: Value = fetcher::fetch(url.as_str()).unwrap();
    let confirmed_value: String = get_confirmed["features"][0]["attributes"]["Value"].to_string();

    let url = Url::parse_with_params(&endpoint::level("county"), 
    &[
        ("f", "json"), 
        ("where", "1=1"),
        ("cacheHint", "true"),
        ("outStatistics", &query::sum_statistics(&"deaths")),
    ]).unwrap();
    let get_deaths: Value = fetcher::fetch(url.as_str()).unwrap();
    let deaths_value: String = get_deaths["features"][0]["attributes"]["Value"].to_string();

    let url = Url::parse_with_params(&endpoint::level("county"), 
    &[
        ("f", "json"), 
        ("where", "1=1"),
        ("cacheHint", "true"),
        ("outStatistics", &query::sum_statistics(&"recovered")),
    ]).unwrap();
    let get_recovered: Value = fetcher::fetch(url.as_str()).unwrap();
    let recovered_value: String = get_recovered["features"][0]["attributes"]["Value"].to_string();
    
    let response = Response {
        confirmed: confirmed_value.parse::<i32>().unwrap(),
        deaths: deaths_value.parse::<i32>().unwrap(),
        recovered: recovered_value.parse::<i32>().unwrap()
    };

    HttpResponse::Ok().json(response)
}