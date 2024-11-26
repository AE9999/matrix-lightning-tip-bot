use warp::{Rejection};
use std::sync::{Arc, Mutex};
use crate::application_service::application_service::ApplicationServiceState;
use ruma::api::appservice::query::query_user_id::v1::{Request as QueryUserIdRequest, Response as QueryUserIdResponse};

pub async fn query_user_id_handler(_state: Arc<Mutex<ApplicationServiceState>>,
                                   _req: QueryUserIdRequest) -> Result<QueryUserIdResponse, Rejection> {
    // Handle the request and return the response
    Ok(QueryUserIdResponse::new())
}