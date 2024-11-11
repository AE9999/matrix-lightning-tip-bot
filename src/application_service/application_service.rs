use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::mpsc;
use warp::Filter;
use reqwest::cookie::Jar;
use reqwest::{Client, ClientBuilder};
use std::sync::Mutex;
use matrix_sdk::{ruma, StateStore};
use ruma::api::client::sync::sync_events::DeviceLists;
use serde::{Deserialize, Serialize};
use crate::application_service::registration::Registration;
use crate::application_service::txnid::TransactionIDCache;

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

// Build the router with Warp filters
fn build_router(state: Arc<Mutex<ApplicationServiceState>>) -> impl Filter<Extract = ()> + Clone {
    let state_filter = warp::any().map(move || state.clone());

    warp::any()
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