use warp::{Reply, Rejection};
use std::sync::{Arc, Mutex};
use warp::http::{HeaderMap, StatusCode};
use warp::reject::Reject;
use crate::application_service::application_service::ApplicationServiceState;
use warp::reply;
use ruma::api::appservice::ping::send_ping::v1::{Request as PingRequest, Response as PingResponse};
use ruma::api::{IncomingRequest, OutgoingResponse};

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
    ping_request: PingRequest,                      // Request body as Bytes
    state: Arc<Mutex<ApplicationServiceState>>,
) -> Result<impl Reply, Rejection> {
    // Check the server token using the extracted headers
    // check_server_token(headers.clone(), state.clone()).await?;

    // Construct the `http::Request` for `PingRequest`
    // let http_request = Request::builder()
    //     .method("POST")
    //     .header("Content-Type", "application/json")
    //     .body(body).unwrap();
    //
    //
    // // Convert the `http::Request` into a `PingRequest`
    // let _ping_request =
    //     PingRequest::try_from_http_request(http_request, &[]);


    // Create a PingResponse and convert it to an HTTP response
    let ping_response = PingResponse::new();
    // let http_response: Response<Vec<u8>> = ping_response
    //     .try_into_http_response()
    //     .unwrap();

    // let mut warp_response = Response::new(http_response.body().clone().into());
    //
    // *warp_response.status_mut() = http_response.status();
    // for (header_name, header_value) in http_response.headers() {
    //     warp_response.headers_mut().insert(header_name, header_value.clone());
    // }

    Ok(warp::reply::json(&ping_response))
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

#[derive(Debug)]
struct InternalServerError {
    message: &'static str,
    status: StatusCode,
}

impl Reject for AuthError {}

impl Reject for InternalServerError {}

impl AuthError {
    fn response(self) -> warp::reply::WithStatus<warp::reply::Json> {
        // warp::reply::with_status(
        //     warp::reply::json(&serde_json::json!({
        //         "error_code": "ERR_UNKNOWN_TOKEN",
        //         "message": self.message,
        //     })),
        //     self.status,
        // )
        todo!()
    }
}

impl InternalServerError {
    fn response(self) -> warp::reply::WithStatus<warp::reply::Json> {
        // warp::reply::with_status(
        //     warp::reply::json(&serde_json::json!({
        //         "error_code": "ERR_UNKNOWN_TOKEN",
        //         "message": self.message,
        //     })),
        //     self.status,
        // )
        todo!()
    }
}
