use actix_web::{App, HttpServer};

mod models;
mod api;
use api::{generate_svg, index};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(generate_svg))
        .bind("127.0.0.1:8080")?
        // .bind("localhost:3000")?
        .run()
        .await
}
