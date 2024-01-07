use std::ptr::NonNull;

use axum::{Json, response::IntoResponse, extract::Query};
use serde::Deserialize;

#[derive(Deserialize,Debug)]
struct Pagination {
    #[serde(default)]
    offset: usize, 
    #[serde(default)]
    limit: Option<usize>, 
    #[serde(default)]
    split: Option<usize>,
}

pub fn router() -> axum::Router {
    axum::Router::new().route("/5", axum::routing::post(task5_post))
}
 
async fn task5_post(Query(pagination):Query<Pagination>,Json(payload):Json<Vec<String>>) -> impl IntoResponse {
    match (pagination.limit, pagination.split) {
        (None,None) => {let res =payload.get(pagination.offset..)
                .unwrap_or_default()
                .to_vec();
                Json(res).into_response()
        },
        (Some(limit),None) => {
            let res = payload.get(pagination.offset..pagination.offset+limit)
                .unwrap_or_default()
                .to_vec();
                Json(res).into_response()
            },
        (Some(limit),Some(split)) =>{

            let res = payload.get(pagination.offset..pagination.offset+limit)
                .unwrap_or_default()
                .chunks(split)
                .map(|s| s.into())
                .collect::<Vec<Vec<String>>>();
                Json(res).into_response()
        },
        (None,Some(split))=> {
            let res = payload.chunks(split)
                .map(|s| s.into())
                .collect::<Vec<Vec<String>>>();
                Json(res).into_response()
            }
        
    }
    // let start= pagination.offset ;
    // let mut end = start+pagination.limit;
    // if payload.len() <end {
    //     end = payload.len();
    // }
    // // let res = &payload[start .. end];
    // let res =payload.get(start..end)
    //         .unwrap_or_default()
    //         .to_vec() ;
    // Json(res).into_response() 


}