use actix_web::{http, App, HttpServer};
use actix_cors::Cors;

mod models;
mod api;
use api::{generate_svg, index};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            // .allowed_origin("http://localhost:3000")
            .allowed_origin_fn(|_, _| true)
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new().wrap(cors).service(index).service(generate_svg)
    })
        // .bind("127.0.0.1:8080")?
        // .bind("localhost:3000")?
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
