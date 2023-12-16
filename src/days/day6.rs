use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing;
use axum::{Json, Router};
use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct ResultRet {
    elf: usize,

    #[serde(rename(serialize = "elf on a shelf"))]
    elf_on_shelf: usize,

    #[serde(rename(serialize = "self with no elf on it"))]
    shelf_no_elf: usize,
}

pub fn router() -> Router {
    Router::new().route("/6", routing::post(task_one))
}

pub async fn task_one(payload: String) -> impl IntoResponse {
    let key = "elf";

    let cnt = payload.as_str().split(key).count() - 1;
    let on_it = payload.as_str().split("elf on a shelf").count() - 1;
    let not_on = payload.as_str().split("shelf").count() - 1;
    let ret = ResultRet {
        elf: cnt,
        elf_on_shelf: on_it,
        shelf_no_elf: not_on - on_it,
    };

    (StatusCode::OK, serde_json::to_string(&ret).unwrap()).into_response()
}
