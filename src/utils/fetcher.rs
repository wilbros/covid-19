use curl::easy::{Easy, List};
use serde_json::{Result, Value};
use std::str;

pub fn fetch(endpoint: &str) -> Result<Value> {
    let mut easy = Easy::new();
    easy.url(endpoint).unwrap();

    let mut headers = List::new();
    headers.append("Referer: https://www.arcgis.com/apps/opsdashboard/index.html").unwrap();
    easy.http_headers(headers).unwrap();

    let mut data = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    let data = str::from_utf8(&data).unwrap();
    let v: Value = serde_json::from_str(data)?;
    Ok(v)
}