#![allow(unused_must_use)]

mod api;
mod config;
mod utils;

use actix_web::{HttpServer, App};
use listenfd::ListenFd;
use actix_cors::Cors;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(|| {
        App::new()
            .wrap(Cors::new()
                .send_wildcard()
                .max_age(3600)
                .finish())
            .configure(config::app::router)
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3000")?
    };

    server.run().await
}