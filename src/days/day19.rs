pub fn router() -> axum::Router {
    axum::Router::new().route("/19", axum::routing::get(axum::http::StatusCode::OK))
}
