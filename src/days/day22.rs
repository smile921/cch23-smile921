pub fn router() -> axum::Router {
    axum::Router::new().route("/22", axum::routing::get(axum::http::StatusCode::OK))
}
