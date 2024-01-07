use axum::{Json, response::IntoResponse, extract::Query};
use serde::Deserialize;

#[derive(Deserialize)]
struct Pagination {
    offset: usize,
    limit: usize,
}

pub fn router() -> axum::Router {
    axum::Router::new().route("/5", axum::routing::post(task5_post))
}
 
async fn task5_post(pagination:Query<Pagination>,Json(payload):Json<Vec<String>>) -> impl IntoResponse {
    let start= pagination.offset ;
    let mut end = start+pagination.limit;
    if payload.len() <end {
        end = payload.len();
    }
    // let res = &payload[start .. end];
    let res =payload.get(start..end)
            .unwrap_or_default()
            .to_vec() ;
    Json(res).into_response() 


}