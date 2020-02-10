#[macro_use]
extern crate log;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new().service(index));

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("host not set");
            let port = env::var("PORT").expect("port not set");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    info!("starting server");
    server.run().await
}
