use warp::{Reply, Rejection};
use std::sync::{Arc, Mutex};
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
    // let state = state.lock().unwrap();
    // Use the state here if necessary
    Ok(warp::reply::json(&format!("Room alias requested: {}", room_alias)))
}

pub async fn get_user(
    user_id: String,
    state: Arc<Mutex<ApplicationServiceState>>,
) -> Result<impl Reply, Rejection> {
    // let state = state.lock().unwrap();
    // Use the state here if necessary
    Ok(warp::reply::json(&format!("User ID requested: {}", user_id)))
}

pub async fn post_ping(
    state: Arc<Mutex<ApplicationServiceState>>,
) -> Result<impl Reply, Rejection> {
    let mut state = state.lock().unwrap();
    // state.live = !state.live; // Toggle live state as an example
    Ok(warp::reply::json(&"Ping received"))
}

pub async fn get_live(
    state: Arc<Mutex<ApplicationServiceState>>,
) -> Result<impl Reply, Rejection> {
    let state = state.lock().unwrap();
    // let live_status = if state.live { "Service is live" } else { "Service is not live" };
    Ok(warp::reply::json(&"live_status"))
}

pub async fn get_ready(
    state: Arc<Mutex<ApplicationServiceState>>,
) -> Result<impl Reply, Rejection> {
    let state = state.lock().unwrap();
    // let ready_status = if state.ready { "Service is ready" } else { "Service is not ready" };
    Ok(warp::reply::json(&"ready_status"))
}