use actix_web::{get, web, App, HttpResponse, HttpServer};
use qrcode::QrCode;
use qrcode::render::svg;

#[get("/")]
async fn index() -> HttpResponse {
    let code = QrCode::new(b"https://buckychu.im").unwrap();
    let image = code.render()
        .min_dimensions(200, 200)
        .dark_color(svg::Color("#800000"))
        .light_color(svg::Color("#ffff80"))
        .build();

    HttpResponse::Ok()
        .content_type("image/svg+xml")
        .body(image)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
