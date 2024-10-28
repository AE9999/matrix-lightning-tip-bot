use warp::{Reply, Rejection};
use std::sync::{Arc, Mutex};
use warp::http::{HeaderMap, StatusCode};
use warp::reject::Reject;
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
    state: Arc<Mutex<ApplicationServiceState>>,
) -> Result<impl Reply, Rejection> {
    let mut state = state.lock().unwrap();
    state.live = !state.live; // Toggle live state as an example
    Ok(warp::reply::json(&"Ping received"))
}

pub async fn get_live(
    state: Arc<Mutex<ApplicationServiceState>>,
) -> Result<impl Reply, Rejection> {
    let state = state.lock().unwrap();
    let live_status = if state.live { "Service is live" } else { "Service is not live" };
    Ok(warp::reply::json(&"live_status"))
}

pub async fn get_ready(
    state: Arc<Mutex<ApplicationServiceState>>,
) -> Result<impl Reply, Rejection> {
    let state = state.lock().unwrap();
    let ready_status = if state.ready { "Service is ready" } else { "Service is not ready" };
    Ok(warp::reply::json(&"ready_status"))
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
