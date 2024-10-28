use warp::{Reply, Rejection};
use std::sync::{Arc, Mutex};
use warp::http::{HeaderMap, StatusCode};
use warp::reject::Reject;
use warp::reply;
use crate::application_service::application_service::ApplicationServiceState;

pub async fn put_transaction(
    txn_id: String,
    state: Arc<Mutex<ApplicationServiceState>>,
) -> Result<impl Reply, Rejection> {
    // Access and modify state here if necessary
    let mut state = state.lock().unwrap();
    // state.some_field = "Updated transaction state".to_string(); // Example modification

    Ok(warp::reply::json(&format!("Transaction received: {}", txn_id)))
}

pub async fn get_room(
    room_alias: String,
    state: Arc<Mutex<ApplicationServiceState>>,
) -> Result<impl Reply, Rejection> {
    let state = state.lock().unwrap();
    // Use the state here if necessary
    Ok(warp::reply::json(&format!("Room alias requested: {}", room_alias)))
}

pub async fn get_user(
    user_id: String,
    state: Arc<Mutex<ApplicationServiceState>>,
) -> Result<impl Reply, Rejection> {
    let state = state.lock().unwrap();
    // Use the state here if necessary
    Ok(warp::reply::json(&format!("User ID requested: {}", user_id)))
}

pub async fn post_ping(
    headers: HeaderMap,
    state: Arc<Mutex<ApplicationServiceState>>,
) -> Result<impl Reply, Rejection> {
    // Check the server token
    check_server_token(headers, state.clone()).await?;

    // Respond with a blank JSON object and OK status
    Ok(reply::with_status(reply::json(&serde_json::json!({})), StatusCode::OK))
}

pub async fn get_live(
    state: Arc<Mutex<ApplicationServiceState>>,
) -> Result<impl Reply, Rejection> {
    let state = state.lock().unwrap();
    let status = if state.live {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    };

    Ok(reply::with_status(reply::json(&serde_json::json!({})), status))
}

pub async fn get_ready(
    state: Arc<Mutex<ApplicationServiceState>>,
) -> Result<impl Reply, Rejection> {
    let state = state.lock().unwrap();
    let status = if state.ready {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    };

    Ok(reply::with_status(reply::json(&serde_json::json!({})), status))
}

pub async fn check_server_token(
    headers: HeaderMap,
    state: Arc<Mutex<ApplicationServiceState>>,
) -> Result<(), warp::Rejection> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok());

    match auth_header {
        Some(auth) if auth.starts_with("Bearer ") => {
            let token = &auth["Bearer ".len()..];
            let state = state.lock().unwrap();
            if token == state.registration.server_token {
                Ok(())
            } else {
                Err(warp::reject::custom(AuthError {
                    message: "Incorrect access token",
                    status: StatusCode::FORBIDDEN,
                }))
            }
        }
        _ => Err(warp::reject::custom(AuthError {
            message: "Missing or invalid access token",
            status: StatusCode::FORBIDDEN,
        })),
    }
}

#[derive(Debug)]
struct AuthError {
    message: &'static str,
    status: StatusCode,
}

impl Reject for AuthError {}

impl AuthError {
    fn response(self) -> warp::reply::WithStatus<warp::reply::Json> {
        warp::reply::with_status(
            warp::reply::json(&serde_json::json!({
                "error_code": "ERR_UNKNOWN_TOKEN",
                "message": self.message,
            })),
            self.status,
        )
    }
}
