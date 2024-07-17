use actix_web::{get, http::header::ContentType, middleware, web::{self, get}, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
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
        -30..=-10 => "Freezing",
        -9..=5 => "Cold",
        6..=14 => "Chilly",
        15..=25 => "Warm",
        26..=35 => "Hot",
        36.. => "Scorching",
        _ => "unknown"
    };

    WeatherResponse {
        temperature: t.to_owned(),
        feeling: text.to_owned()
    }
}

#[get("/")]
async fn hello() -> impl Responder {
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
            .service(hello)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}