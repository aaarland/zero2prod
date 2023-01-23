use actix_web::App;
use actix_web::{dev::Server, web, HttpServer};
use std::net::TcpListener;

use crate::routes::*;

pub fn run(listner: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listner)?
    .run();

    Ok(server)
}
