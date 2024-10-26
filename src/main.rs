#![allow(dead_code)]

mod db;
mod json;
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server::run_server().await
}
