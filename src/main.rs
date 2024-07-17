use actix_web::{get, http::header::ContentType, middleware, web::{self}, App, HttpResponse, HttpServer};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct WeatherResponse {
    temperature: i32,
    feeling: String
}

fn gen_weather() -> WeatherResponse {
    let t = rand::thread_rng().gen_range(-30..50);
    let text = match t {
        ..=-31 => "Extremely cold",
        -30..=-10 => "Freezing",
        -9..=5 => "Cold",
        6..=14 => "Chilly",
        15..=25 => "Warm",
        26..=35 => "Hot",
        36.. => "Scorching"
    };

    WeatherResponse {
        temperature: t,
        feeling: text.to_owned()
    }
}

#[get("/")]
async fn weather_measurement() -> HttpResponse {
    let w = gen_weather();
    let resp = serde_json::to_string(&w).unwrap();

    HttpResponse::Ok().insert_header(ContentType(mime::APPLICATION_JSON)).body(resp)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("Starting HTTP server at http://localhost:{}", port);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::JsonConfig::default().limit(4096))
            .service(weather_measurement)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use actix_web::{dev::Service, http, test};

    use super::*;

    #[actix_web::test]
    async fn index_returns_ok(){
        let app =
        test::init_service(App::new().service(weather_measurement)).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let response = app.call(req).await.unwrap();

        assert_eq!(response.status(), http::StatusCode::OK);
    }
}