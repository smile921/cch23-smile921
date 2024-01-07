use axum::{http::{HeaderMap, header::COOKIE}, response::IntoResponse, Json};

pub fn router() -> axum::Router {
    axum::Router::new().route("/7/decode", axum::routing::get(get_cookie_recipe))
}

async fn get_cookie_recipe(headers:HeaderMap)-> impl IntoResponse {
    let key= "recipe";
    let cookies= headers.get(COOKIE);
    let cookie_str = match cookies {
        None => "",
        Some(cookie) => {
            cookies
                .and_then(|s| s.to_str().ok())
                .unwrap_or("")
        }
    }; 
    Json(cookie_str).into_response()
}