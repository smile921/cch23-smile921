use axum::{response::IntoResponse, Json};

use serde::Deserialize;
use serde::Serialize;

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct Reindeer {
    name: String,
    strength: i32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct Reindeer2 {
    name: String,
    strength: i32,
    speed: f64,
    height: i32,
    antler_width: i32,
    snow_magic_power: i32,
    favorite_food: String,
    #[serde(alias = "cAnD13s_3ATeN-yesT3rdAy")]
    consume: i32,
}

#[derive(Deserialize, Serialize)]
struct ResultResp {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

impl ResultResp {
    fn new(deer1: Vec<Reindeer2>) -> ResultResp {
        let deer = deer1.as_slice();
        let f_speed = |a: &Reindeer2, b: &Reindeer2| {
            if a.speed > b.speed {
                a
            } else {
                b
            }
        };
        let fastest_deer = deer
            .into_iter()
            .map(|reinder| reinder)
            .reduce(f_speed)
            .unwrap();
        let f_tall = |a: &Reindeer2, b: &Reindeer2| {
            if a.height > b.height {
                a
            } else {
                b
            }
        };
        let tallest_deer = deer.into_iter().map(|a| a).reduce(f_tall).unwrap();

        let f_magic = |a: &Reindeer2, b: &Reindeer2| {
            if a.snow_magic_power > b.snow_magic_power {
                a
            } else {
                b
            }
        };
        let magician_deer = deer
            .into_iter()
            .map(|reinder| reinder)
            .reduce(f_magic)
            .unwrap();

        let f_consume = |a: &Reindeer2, b: &Reindeer2| {
            if a.consume > b.consume {
                a
            } else {
                b
            }
        };
        let consume_deer = deer
            .into_iter()
            .map(|reinder| reinder)
            .reduce(f_consume)
            .unwrap();
        let fastest = format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest_deer.strength, fastest_deer.name
        );
        let tallest = format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest_deer.name, tallest_deer.antler_width
        );
        let magician = format!(
            "{} could blast you away with a snow magic power of {}",
            magician_deer.name, magician_deer.snow_magic_power
        );
        let consumer = format!(
            "{} ate lots of candies, but also some {}",
            consume_deer.name, consume_deer.favorite_food
        );
        let result = ResultResp {
            tallest,
            fastest,
            magician,
            consumer,
        };
        result
    }
}

pub fn router() -> axum::Router {
    axum::Router::new()
        .nest("/", router1())
        .nest("/", router2())
}

pub fn router1() -> axum::Router {
    axum::Router::new().route("/4/strength", axum::routing::post(day_four))
}

pub fn router2() -> axum::Router {
    axum::Router::new().route("/4/contest", axum::routing::post(day_four_task2))
}

async fn day_four(Json(payload): Json<Vec<Reindeer>>) -> impl IntoResponse {
    let sum = payload
        .into_iter()
        .map(|reinder| reinder.strength)
        .reduce(|a, b| a + b)
        .unwrap_or_default();

    (axum::http::StatusCode::OK, sum.to_string()).into_response()
}

async fn day_four_task2(Json(payload): Json<Vec<Reindeer2>>) -> impl IntoResponse {
    let result = ResultResp::new(payload);
    (
        axum::http::StatusCode::OK,
        serde_json::to_string(&result).unwrap(),
    )
        .into_response()
}
