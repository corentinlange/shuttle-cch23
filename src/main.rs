use std::collections::HashMap;

use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use serde::Deserialize;
use serde_json::to_string;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn error_500() -> impl IntoResponse {
    StatusCode::INTERNAL_SERVER_ERROR
}

fn extract_integers_from_sled(sled: String) -> Vec<u32> {
    sled.split('/')
        .filter_map(|s| s.parse::<u32>().ok())
        .collect()
}

fn cube_the_bits_operation(integers: Vec<u32>) -> u32 {
    let mut result: u32 = 0;
    for integer in integers {
        result = result ^ integer;
    }
    result.pow(3)
}

async fn cube_the_bits(Path(sled): Path<String>) -> impl IntoResponse {
    println!("{sled}");
    let integers = extract_integers_from_sled(sled);
    let result = cube_the_bits_operation(integers);
    result.to_string()
}

#[derive(Clone, Deserialize)]
struct Reindeer {
    name: String,
    strength: u32,
    speed: f32,
    height: u32,
    antler_width: u32,
    snow_magic_power: u32,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: u32,
}

async fn combined_strength(Json(payload): Json<Vec<Reindeer>>) -> impl IntoResponse {
    payload
        .into_iter()
        .map(|r| r.strength)
        .sum::<u32>()
        .to_string()
}

async fn contest(Json(payload): Json<Vec<Reindeer>>) -> impl IntoResponse {
    let fastest: Reindeer = payload
        .clone()
        .into_iter()
        .max_by(|a, b| {
            a.speed
                .partial_cmp(&b.speed)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap();
    let tallest: Reindeer = payload
        .clone()
        .into_iter()
        .max_by(|a, b| {
            a.height
                .partial_cmp(&b.height)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap();
    let magician: Reindeer = payload
        .clone()
        .into_iter()
        .max_by(|a, b| {
            a.snow_magic_power
                .partial_cmp(&b.snow_magic_power)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap();
    let consumer: Reindeer = payload
        .clone()
        .into_iter()
        .max_by(|a, b| {
            a.candies_eaten_yesterday
                .partial_cmp(&b.candies_eaten_yesterday)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap();

    let mut response = HashMap::new();
    response.insert(
        "fastest",
        format!(
            "Speeding past the finish line with a strength of 5 is {}",
            fastest.name
        ),
    );
    response.insert(
        "tallest",
        format!(
            "{} is standing tall with his 36 cm wide antlers",
            tallest.name
        ),
    );
    response.insert(
        "magician",
        format!(
            "{} could blast you away with a snow magic power of 9001",
            magician.name
        ),
    );
    response.insert(
        "consumer",
        format!("{} ate lots of candies, but also some grass", consumer.name),
    );

    let json_response = Json(response);
    let stringified_json = to_string(&json_response.0).expect("Failed to serialize");
    stringified_json
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(error_500))
        .route("/1/*sled", get(cube_the_bits))
        .route("/4/strength", post(combined_strength))
        .route("/4/contest", post(contest));

    Ok(router.into())
}
