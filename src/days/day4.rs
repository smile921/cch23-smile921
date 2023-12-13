use axum::{Json, response::IntoResponse};

use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug,Clone,Deserialize)]
pub struct Reindeer {
    name: String,
    strength: i32,
}


#[allow(dead_code)]
#[derive(Debug,Clone,Deserialize)]
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


pub fn router() -> axum::Router {
    axum::Router::new().nest("/", router1())
    .nest("/", router2())
}

pub fn router1() -> axum::Router {
    axum::Router::new().route("/4/strength", axum::routing::post(day_four))
}

pub fn router2() -> axum::Router {
    axum::Router::new().route("/4/contest", axum::routing::post(day_four_task2))
}


async fn day_four(Json(payload): Json<Vec<Reindeer>>) -> impl IntoResponse {
     
    let sum = payload.into_iter().map(|reinder| reinder.strength).reduce(|a,b| a+b ).unwrap_or_default();

    ( axum::http::StatusCode::OK,
        sum.to_string() ,
    ).into_response()

}


async fn day_four_task2(Json(payload): Json<Vec<Reindeer2>>) -> impl IntoResponse {
    let f_speed = |a:Reindeer2,b:Reindeer2|{
        if(a.speed>b.speed){
            a
        } else {
            b
        }
    };
    let fastest = payload.into_iter().map(|reinder| reinder).reduce(f_speed).unwrap();

    // let f_height = |a:Reindeer2,b:Reindeer2|{
    //     if(a.height>b.height){
    //         a
    //     } else {
    //         b
    //     }
    // };
    // let tallest = payload.into_iter().map(|reinder| reinder).reduce(f_height).unwrap();
    let result = format!("Speeding past the finish line with a strength of {} is {}",fastest.strength,fastest.name);
    ( axum::http::StatusCode::OK,
        result,
    ).into_response()

}