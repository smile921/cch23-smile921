use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing;
use axum::{Json, Router};
use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct ResultRet {
    elf: usize,
}

pub fn router() -> Router {
    Router::new().route("/6", routing::post(task_one))
}

pub async fn task_one( payload: String ) -> impl IntoResponse {
    let key = "elf";
    
    let cnt = payload.as_str().split(key).count()-1;
    let ret = ResultRet { elf: cnt };

    (StatusCode::OK, serde_json::to_string(&ret).unwrap()).into_response()
}
