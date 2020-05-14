use actix_web::{HttpResponse};

pub async fn fetch() -> HttpResponse {
    HttpResponse::Ok().body("confirmed".to_string())
}

pub async fn fetch_by_country() -> HttpResponse {
    HttpResponse::Ok().body("fetch by country".to_string())
}