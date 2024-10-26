mod http_methods;
mod protocol;

pub mod application_service {
    use std::{collections::HashMap, sync::Arc, time::Duration};
    use tokio::sync::mpsc;
    use warp::Filter;
    use reqwest::cookie::Jar;
    use reqwest::{Client, ClientBuilder};
    use warp::http::Method;
    use warp::Rejection; // Import Rejection for error handling
    use std::sync::Mutex;
    use matrix_sdk::ruma;
    use warp::reply;
    use ruma::api::client::sync::sync_events::DeviceLists;

    use crate::application_service::http_methods::{get_live, get_ready, get_room, get_user, post_ping, put_transaction};

    type Event = String;

    pub struct ApplicationServiceState {
        clients: HashMap<Arc<ruma::UserId>, Client>,  // Wrap UserId in Arc
        intents: HashMap<Arc<ruma::UserId>, String>,   // Wrap UserId in Arc
        http_client: Client,
        event_channel: mpsc::Sender<Event>,
        to_device_events: mpsc::Sender<Event>,
        // otk_counts: mpsc::Sender<OTKCount>, not supported
        device_lists: mpsc::Sender<DeviceLists>,
        user_agent: String,
        live: bool,
        ready: bool,
    }

    impl ApplicationServiceState {
        async fn new() -> Self {
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
                http_client,
                event_channel: event_tx,
                to_device_events: to_device_tx,
                //otk_counts: otk_counts_tx,
                device_lists: device_lists_tx,
                user_agent: "mautrix".to_string(),
                live: true,
                ready: false,
            }
        }
    }

    // Build the router with Warp filters
    fn build_router(state: Arc<Mutex<ApplicationServiceState>>) -> impl Filter<Extract = impl warp::Reply> + Clone {
        let state_filter = warp::any().map(move || state.clone());

        warp::path("_matrix")
            .and(
                warp::path("app")
                    .and(warp::path("v1"))
                    .and(
                        warp::path("transactions")
                            .and(warp::path::param())
                            .and(state_filter.clone())
                            .and(warp::put())
                            .and_then(put_transaction)
                            .or(warp::path("rooms")
                                .and(warp::path::param())
                                .and(state_filter.clone())
                                .and(warp::get())
                                .and_then(get_room))
                            .or(warp::path("users")
                                .and(warp::path::param())
                                .and(state_filter.clone())
                                .and(warp::get())
                                .and_then(get_user))
                            .or(warp::path("ping")
                                .and(warp::post())
                                .and(state_filter.clone())
                                .and_then(post_ping)),
                    ),
            )
            .or(warp::path("_matrix")
                    .and(warp::path("mau"))
                    .and(
                        warp::path("live")
                            .and(warp::get())
                            .and(state_filter.clone())
                            .and_then(get_live),
                    )
                    .or(warp::path("ready")
                        .and(warp::get())
                        .and(state_filter.clone())
                        .and_then(get_ready)),
            )
    }

}