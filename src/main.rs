use actix_web::{get, web, App, HttpResponse, HttpServer};
use image::{DynamicImage, Luma};
use qrcode::QrCode;
// use qrcode::render::svg;

// #[get("/")]
// async fn index() -> HttpResponse {
//     let code = QrCode::new(b"https://buckychu.im").unwrap();
//     let image = code.render()
//         .min_dimensions(200, 200)
//         .dark_color(svg::Color("#800000"))
//         .light_color(svg::Color("#ffff80"))
//         .build();

//     HttpResponse::Ok()
//         .content_type("image/svg+xml")
//         .body(image)
// }
#[derive(serde::Deserialize)]
struct Info {
    url: String,
}

#[get("/generate_qr")]
async fn index(data: web::Query<Info>) -> HttpResponse {
    let code = match QrCode::new(data.url.as_bytes()) {
        Ok(c) => c,
        Err(_) => return HttpResponse::BadRequest().body("你輸入的字串無法處理"),
    };

    let image = code.render::<Luma<u8>>().build();

    let mut buffer = Vec::new();
    match DynamicImage::ImageLuma8(image).write_to(&mut buffer, image::ImageOutputFormat::Png) {
        Ok(_) => (),
        Err(_) => return HttpResponse::InternalServerError().body("無法生成圖像"),
    }

    HttpResponse::Ok().content_type("image/png").body(buffer)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
