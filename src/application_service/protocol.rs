use warp::http::{Response, StatusCode};
use serde::Serialize;
use warp::reply::Reply;

fn write_blank_ok() -> impl warp::Reply {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body("{}")
        .unwrap()
}

fn respond<T: Serialize>(data: &T) -> Result<impl Reply, warp::Rejection> {
    match serde_json::to_string(data) {
        Ok(data_str) => Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(data_str)
            .unwrap()),
        Err(_) => Err(warp::reject::custom(SerializationError)),
    }
}


#[derive(Debug)]
struct SerializationError;

impl warp::reject::Reject for SerializationError {}