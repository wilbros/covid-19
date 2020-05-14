use actix_web::{HttpResponse};

pub async fn fetch() -> HttpResponse {
    HttpResponse::Ok().body("countries".to_string())
}

pub async fn fetch_by_country() -> HttpResponse {
    HttpResponse::Ok().body("countries summary".to_string())
}