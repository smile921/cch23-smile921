use axum::{http::{HeaderMap, header::COOKIE}, response::IntoResponse, Json};
use axum_extra::{TypedHeader, headers::Cookie};
use base64::{engine::general_purpose, Engine};
use serde_json::Value;

pub fn router() -> axum::Router {
    axum::Router::new().route("/7/decode", axum::routing::get(get_cookie_recipe))
}

#[allow(unused)]
async fn get_cookie_recipe_v01(headers:HeaderMap)-> impl IntoResponse {
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

async fn get_cookie_recipe(cookie: TypedHeader<Cookie>) -> impl IntoResponse {

    let raw_recipe= cookie.get("recipe").unwrap_or_default();
    println!("recipe {:?}",&raw_recipe);
    let bytes = general_purpose::STANDARD.decode(raw_recipe).unwrap();
    let json_recipe = std::str::from_utf8(&bytes).unwrap();
    println!("decoded {:?}",json_recipe);
    let recipe:Value = serde_json::from_str(json_recipe).unwrap();
    Json( recipe
        )
        .into_response()

}