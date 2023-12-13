use axum::{Json, response::IntoResponse};

use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug,Clone,Deserialize)]
pub struct Reindeer {
    name: String,
    strength: i32,
}

pub fn router() -> axum::Router {
    axum::Router::new().route("/4/strength", axum::routing::post(day_four))
}


async fn day_four(Json(payload): Json<Vec<Reindeer>>) -> impl IntoResponse {
     
    let sum = payload.into_iter().map(|reinder| reinder.strength).reduce(|a,b| a+b ).unwrap_or_default();

    ( axum::http::StatusCode::OK,
        sum.to_string() ,
    ).into_response()

}