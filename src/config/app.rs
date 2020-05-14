
use crate::api::*;
use actix_web::web;

pub fn router(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api")
            .service(
                web::resource("")
                    .route(web::get().to(global::fetch))
            )
            .service(
                web::resource("/confirmed")
                    .route(web::get().to(confirmed::fetch))
            )
            .service(
                web::resource("/recovered")
                    .route(web::get().to(recovered::fetch))
            )
            .service(
                web::resource("/deaths")
                    .route(web::get().to(deaths::fetch))
            )
            .service(
                web::scope("/countries")
                    .service(
                        web::resource("")
                            .route(web::get().to(countries::fetch))
                    )
                    .service(
                        web::scope("/{country}")
                            .service(
                                web::resource("")
                                    .route(web::get().to(countries::fetch_by_country))
                            )
                            .service(
                                web::resource("/confirmed")
                                    .route(web::get().to(confirmed::fetch_by_country))
                            )
                            .service(
                                web::resource("/recovered")
                                    .route(web::get().to(recovered::fetch_by_country))
                            )
                            .service(
                                web::resource("/deaths")
                                    .route(web::get().to(deaths::fetch_by_country))
                            )
                    )
                
            )
    );
}