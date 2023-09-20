use actix_web::{get, post, web, HttpResponse};
use dotenv::dotenv;
use image::{DynamicImage, Luma};
use qrcode::render::svg;
use qrcode::QrCode;
use regex::Regex;
use std::env;

use crate::models::Info;

const MIN_DIMENSION: u32 = 100;
const MAX_DIMENSION: u32 = 2000;

#[derive(Debug)]
enum MyError {
    MissingField(String),
    ReqwestError(reqwest::Error),
}

impl From<reqwest::Error> for MyError {
    fn from(err: reqwest::Error) -> MyError {
        MyError::ReqwestError(err)
    }
}

impl Default for Info {
    fn default() -> Self {
        Self {
            url: None,
            phone: None,
            email: None,
            address: None,
            background: None,
            dimension: None,
            foreground: None,
        }
    }
}

fn is_valid_color(color: &str) -> bool {
    let re = Regex::new(r"^#[0-9a-fA-F]{6}$").unwrap();
    re.is_match(color)
}

async fn get_coordinates(address: &str) -> Result<(f64, f64), MyError> {
    dotenv().ok();

    let api_key = env::var("GOOGLE_MAPS_API_KEY").expect("GOOGLE_MAPS_API_KEY must be set");
    let url = format!(
        "https://maps.googleapis.com/maps/api/geocode/json?address={}&key={}",
        address, api_key
    );

    let response: serde_json::Value = reqwest::get(&url).await?.json().await?;

    let lat = response["results"][0]["geometry"]["location"]["lat"]
        .as_f64()
        .ok_or(MyError::MissingField(
            "Latitude is missing or not a float".to_string(),
        ))?;
    let lng = response["results"][0]["geometry"]["location"]["lng"]
        .as_f64()
        .ok_or(MyError::MissingField(
            "Longitude is missing or not a float".to_string(),
        ))?;

    Ok((lat, lng))
}

async fn get_code_data(data: &Info) -> Option<Vec<u8>> {
    if let Some(url) = &data.url {
        return Some(url.as_bytes().to_vec());
    }
    if let Some(phone) = &data.phone {
        return Some(format!("tel:{}", phone).as_bytes().to_vec());
    }
    if let Some(email) = &data.email {
        return Some(format!("mailto:{}", email).as_bytes().to_vec());
    }
    if let Some(address) = &data.address {
        if let Ok((lat, lng)) = get_coordinates(address).await {
            return Some(format!("geo:{},{}", lat, lng).as_bytes().to_vec());
        }
    }
    None
}

#[get("/generate_qr")]
async fn index(data: web::Query<Info>) -> HttpResponse {
    let code_data = match get_code_data(&data).await {
        Some(data) => data,
        None => return HttpResponse::BadRequest().body("缺少有效數據"),
    };

    let code = match QrCode::new(code_data) {
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

    let code_data = match get_code_data(&data).await {
        Some(data) => data,
        None => return HttpResponse::BadRequest().body("缺少有效數據"),
    };

    let code = match QrCode::new(code_data) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[test]
    fn test_is_valid_color() {
        assert!(is_valid_color("#FFFFFF"));
        assert!(is_valid_color("#000000"));
        assert!(!is_valid_color("#GGGGGG"));
        assert!(!is_valid_color("FFFFFF"));
    }

    #[actix_rt::test]
    async fn test_get_coordinates() {
        let api_key = match env::var("GOOGLE_MAPS_API_KEY") {
            Ok(key) => key,
            Err(_) => {
                println!("Warning: 讀取不到 GOOGLE_MAPS_API_KEY，跳出測試");
                return;
            }
        };
        let mut server = Server::new();
        let mock = server
            .mock(
                "GET",
                format!(
                    "https://maps.googleapis.com/maps/api/geocode/json?address=test&key={}",
                    api_key
                )
                .as_str(),
            )
            .with_status(200)
            .with_body(
                r#"{
        "results": [
            {
                "geometry": {
                    "location": {
                        "lat": 40.0,
                        "lng": -100.0
                    }
                }
            }
        ]
    }"#,
            )
            .create();

        let result = get_coordinates("test").await;

        match result {
            Ok((lat, lng)) => {
                assert_eq!(lat, 40.0);
                assert_eq!(lng, -100.0);
            }
            Err(e) => {
                println!("Debug: Error returned: {:?}", e);
                panic!("get_coordinates returned an error");
            }
        }

        mock.assert();
    }

    #[actix_rt::test]
    async fn test_get_code_data() {
        let mut info = Info::default();

        info.url = Some("https://example.com".to_string());
        let result = get_code_data(&info).await;
        assert_eq!(result, Some("https://example.com".as_bytes().to_vec()));

        info.url = None;
        info.phone = Some("1234567890".to_string());
        let result = get_code_data(&info).await;
        assert_eq!(result, Some("tel:1234567890".as_bytes().to_vec()));

        info.phone = None;
        info.email = Some("test@example.com".to_string());
        let result = get_code_data(&info).await;
        assert_eq!(result, Some("mailto:test@example.com".as_bytes().to_vec()));

        info.email = None;
        info.address = Some("277 Bedford Avenue, Brooklyn, NY 11211, USA".to_string());

        info.address = None;
        let result = get_code_data(&info).await;
        assert_eq!(result, None);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::{test, App};
    use serde_json::json;

    #[actix_rt::test]
    async fn test_index() {
        let mut app = test::init_service(App::new().service(index)).await;

        let req = test::TestRequest::get()
            .uri("/generate_qr?url=https://example.com&email=test@example.com&dimension=300")
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_generate_svg_function() {
        let mut app = test::init_service(App::new().service(generate_svg)).await;

        let payload = json!({
            "url": "https://example.com",
        });

        let req = test::TestRequest::post()
            .uri("/generate_qr_svg")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
