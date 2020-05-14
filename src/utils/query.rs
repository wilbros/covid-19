use serde_json::json;
use std::str;

pub fn sum_statistics(field: &str) -> String {
    let query = json!([{
        "statisticType": "sum",
        "onStatisticField": field,
        "outStatisticFieldName": "Value"
    }]);

    return String::from(query.to_string());
}