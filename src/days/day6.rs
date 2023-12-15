use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing;
use axum::{Json, Router};
use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct ResultRet {
    elf: isize,
}

pub fn router() -> Router {
    Router::new().route("/6", routing::post(task_one))
}

pub async fn task_one( _payload: String ) -> impl IntoResponse {
    
    let ret = ResultRet { elf: 1 };

    (StatusCode::OK, serde_json::to_string(&ret).unwrap()).into_response()
}
