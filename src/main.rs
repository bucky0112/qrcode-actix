use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use image::{DynamicImage, Luma};
use qrcode::render::svg;
use qrcode::QrCode;
use regex::Regex;

fn is_valid_color(color: &str) -> bool {
    let re = Regex::new(r"^#[0-9a-fA-F]{6}$").unwrap();
    re.is_match(color)
}

const MIN_DIMENSION: u32 = 100;
const MAX_DIMENSION: u32 = 2000;

#[derive(serde::Deserialize)]
struct Info {
    url: String,
    foreground: Option<String>,
    background: Option<String>,
    dimension: Option<u32>,
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

#[post("/generate_qr_svg")]
async fn generate_svg(data: web::Json<Info>) -> HttpResponse {
    let fg_color_str = match &data.foreground {
        Some(color) if is_valid_color(color) => color,
        _ => "#000000",
    };

    let bg_color_str = match &data.background {
        Some(color) if is_valid_color(color) => color,
        _ => "#FFFFFF",
    };

    let code = match QrCode::new(data.url.as_bytes()) {
        Ok(c) => c,
        Err(_) => return HttpResponse::BadRequest().body("你輸入的字串無法處理"),
    };

    let size = match &data.dimension {
        Some(dimension) if *dimension >= MIN_DIMENSION && *dimension <= MAX_DIMENSION => *dimension,
        _ => 200,
    };

    let image = code
        .render()
        .min_dimensions(size, size)
        .dark_color(svg::Color(fg_color_str))
        .light_color(svg::Color(bg_color_str))
        .build();

    HttpResponse::Ok().content_type("image/svg+xml").body(image)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(generate_svg))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
