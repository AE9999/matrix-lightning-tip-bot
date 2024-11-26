use std::{collections::HashMap, sync::Arc, time::Duration};
use std::future::Future;
use tokio::sync::mpsc;
use reqwest::cookie::Jar;
use reqwest::{Client, ClientBuilder};
use std::sync::Mutex;
use http::Method;
use ruma::api::client::sync::sync_events::DeviceLists;
use serde::{Deserialize, Serialize};
use warp::{Filter, Rejection};
use crate::application_service::registration::Registration;
use crate::application_service::txnid::TransactionIDCache;
use bytes::Bytes;
use ruma::api::{
    IncomingRequest,
};
use crate::application_service::http_methods::query_user_id_handler;

#[derive(Debug)]
struct HttpError(http::Error);

impl warp::reject::Reject for HttpError {}

#[derive(Debug)]
struct RumaFromHttpRequestError(ruma::api::error::FromHttpRequestError);

impl warp::reject::Reject for RumaFromHttpRequestError {}

type Event = String;

pub struct ApplicationServiceState {
    pub clients: HashMap<Arc<ruma::UserId>, Client>,  // Wrap UserId in Arc
    pub intents: HashMap<Arc<ruma::UserId>, String>,   // Wrap UserId in Arc
    pub registration: Registration,
    pub http_client: Client,
    pub event_channel: mpsc::Sender<Event>,
    pub to_device_events: mpsc::Sender<Event>,
    pub txn_idc_cache: TransactionIDCache,
    // otk_counts: mpsc::Sender<OTKCount>, not supported
    pub device_lists: mpsc::Sender<DeviceLists>,
    pub user_agent: String,
    pub live: bool,
    pub ready: bool,
}

impl ApplicationServiceState {
    async fn new(create_opts: CreateOpts) -> Self {
        // Configure the HTTP client with a cookie jar
        let jar = Arc::new(Jar::default());
        let http_client = ClientBuilder::new()
            .cookie_provider(jar.clone())
            .timeout(Duration::from_secs(180))
            .build()
            .expect("Failed to create HTTP client");

        // Create channels
        let (event_tx, _event_rx) = mpsc::channel::<Event>(128);
        let (to_device_tx, _to_device_rx) = mpsc::channel::<Event>(128);
        //let (otk_counts_tx, _otk_counts_rx) = mpsc::channel::<OTKCount>(64);
        let (device_lists_tx, _device_lists_rx) = mpsc::channel::<DeviceLists>(128);

        // Initialize state
        ApplicationServiceState {
            clients: HashMap::new(),
            intents: HashMap::new(),
            registration: create_opts.registration,
            http_client,
            event_channel: event_tx,
            to_device_events: to_device_tx,
            txn_idc_cache: TransactionIDCache::new(128),
            //otk_counts: otk_counts_tx,
            device_lists: device_lists_tx,
            user_agent: "mautrix".to_string(),
            live: true,
            ready: false,
        }
    }
}

// RumaHandler trait with associated types
pub trait RumaHandler {
    type Req: IncomingRequest + Send + 'static;
    type Resp: Serialize + Send + 'static;
    type Error: warp::reject::Reject + Send + 'static;
    type Fut: Future<Output = Result<Self::Resp, Self::Error>> + Send + 'static;

    fn add_to_filter(
        self,
        filter: warp::filters::BoxedFilter<(impl warp::Reply,)>,
        state: Arc<Mutex<ApplicationServiceState>>,
    ) -> warp::filters::BoxedFilter<(impl warp::Reply,)>;
}

// Helper trait to tie associated types to F
pub trait HandlerTypes {
    type Req: IncomingRequest + Send + 'static;
    type Resp: Serialize + Send + 'static;
    type Error: warp::reject::Reject + Send + 'static;
    type Fut: Future<Output = Result<Self::Resp, Self::Error>> + Send + 'static;
}

// Implement HandlerTypes for F
impl<F, Req, Resp, Error, Fut> HandlerTypes for F
where
    F: Fn(Arc<Mutex<ApplicationServiceState>>, Req) -> Fut,
    Req: IncomingRequest + Send + 'static,
    Resp: Serialize + Send + 'static,
    Error: warp::reject::Reject + Send + 'static,
    Fut: Future<Output = Result<Resp, Error>> + Send + 'static,
{
    type Req = Req;
    type Resp = Resp;
    type Error = Error;
    type Fut = Fut;
}

// Helper function to convert HTTP method to Warp filter
fn method_to_filter(method: Method) -> warp::filters::BoxedFilter<()> {
    match method {
        Method::GET => warp::get().boxed(),
        Method::POST => warp::post().boxed(),
        Method::PUT => warp::put().boxed(),
        Method::DELETE => warp::delete().boxed(),
        _ => panic!("Unsupported HTTP method: {:?}", method),
    }
}

// Helper function to parse the incoming request into the Req type
fn ruma_request_filter<Req>() -> impl Filter<Extract = (Req,), Error = Rejection> + Clone
where
    Req: IncomingRequest + Send + 'static,
{
    warp::body::bytes().and_then(|body: Bytes| async move {
        let http_request = http::Request::builder()
            .body(body)
            .map_err(|e| warp::reject::custom(HttpError(e)))?;

        match Req::try_from_http_request(http_request, &[]) {
            Ok(req) => Ok(req),
            Err(err) => Err(warp::reject::custom(RumaFromHttpRequestError(err))),
        }
    })
}

// Updated macro
macro_rules! impl_ruma_handler {
    ( $($ty:ident),* $(,)? ) => {
        impl<F, $($ty,)*> RumaHandler for F
        where
            F: HandlerTypes
                + Fn($($ty,)* Arc<Mutex<ApplicationServiceState>>, F::Req) -> F::Fut
                + Clone
                + Send
                + 'static,
            $(
                $ty: Filter<Extract = ($ty::Extract,), Error = warp::Rejection> + Send + Clone + 'static,
            )*
        {
            type Req = F::Req;
            type Resp = F::Resp;
            type Error = F::Error;
            type Fut = F::Fut;

            fn add_to_filter(
                self,
                filter: warp::filters::BoxedFilter<(impl warp::Reply,)>,
                state: Arc<Mutex<ApplicationServiceState>>,
            ) -> warp::filters::BoxedFilter<(impl warp::Reply,)> {
                let meta = <Self::Req as IncomingRequest>::METADATA;

                let mut combined_filter = filter;

                for path in meta.history.all_paths() {
                    let handler = self.clone();
                    let state = state.clone();
                    let method_filter = method_to_filter(meta.method);

                    let endpoint = warp::path(path)
                        .and(method_filter)
                        .and(warp::any().map(move || state.clone()))
                        $(.and($ty.clone()))*
                        .and(ruma_request_filter::<Self::Req>())
                        .and_then(move |$($ty,)* state, req| async move {
                            handler($($ty,)* state, req)
                                .await
                                .map(|response| warp::reply::json(&response))
                                .map_err(warp::reject::custom)
                        })
                        .boxed();

                    combined_filter = combined_filter
                        .or(endpoint)
                        .unify()
                        .boxed();
                }

                combined_filter
            }
        }
    }
}

// Apply the macro
impl_ruma_handler!();

// Invoke the macro
// impl_ruma_handler!(T1);
// impl_ruma_handler!(T1, T2);
// impl_ruma_handler!(T1, T2, T3);
// impl_ruma_handler!(T1, T2, T3, T4);
// impl_ruma_handler!(T1, T2, T3, T4, T5);
// impl_ruma_handler!(T1, T2, T3, T4, T5, T6);
// impl_ruma_handler!(T1, T2, T3, T4, T5, T6, T7);
// impl_ruma_handler!(T1, T2, T3, T4, T5, T6, T7, T8);

// Build the router with Warp filters
pub fn build_router(
    state: Arc<Mutex<ApplicationServiceState>>,
) -> impl warp::Filter + Clone {
    // Starting with a base filter (e.g., a health check)

    // Starting with a base filter (e.g., a health check)
    let base_filter = warp::path!("health")
        .and(warp::any().map(move || state.clone()))
        .map(|state: Arc<Mutex<ApplicationServiceState>>| {
            // Example health check with access to state
            let _ = state.lock().unwrap(); // Just to demonstrate state usage
            warp::reply::json(&"OK")
        });

    // Add filters using the macro-defined Ruma handlers
    base_filter
        .add_to_filter(query_user_id_handler, state.clone())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HostConfig {
    #[serde(rename = "hostname")]
    pub hostname: String,

    #[serde(rename = "port")]
    pub port: Option<u16>, // Port is optional if using a Unix socket
}

pub struct CreateOpts {
    // Required fields
    registration: Registration, // Using Arc to represent a shared Registration instance
    homeserver_domain: String,
    homeserver_url: String,
    host_config: HostConfig,
}