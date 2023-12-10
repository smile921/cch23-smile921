use axum::{extract::Path, response::IntoResponse};

pub fn router() -> axum::Router {
    axum::Router::new().route("/2/:num1/:num2", axum::routing::get(day_one))
}

async fn day_one(Path((num1, num2)): Path<(i32, i32)>) -> impl IntoResponse {
    let mut num = 0;
    num = num1 ^ num2; // Perform the bitwise xor operation
    num = num.pow(3);
    let ret = format!("{}", num);
    (axum::http::StatusCode::OK, ret).into_response()
}
