use serde::{Deserialize, Serialize};
use rand::{distributions::Alphanumeric, Rng};


#[derive(Debug, Serialize, Deserialize)]
pub struct Registration {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "as_token")]
    pub app_token: String,

    #[serde(rename = "hs_token")]
    pub server_token: String,

    #[serde(rename = "sender_localpart")]
    pub sender_localpart: String,

    #[serde(rename = "rate_limited", skip_serializing_if = "Option::is_none")]
    pub rate_limited: Option<bool>,

    #[serde(rename = "namespaces")]
    pub namespaces: Namespaces,

    #[serde(rename = "protocols", skip_serializing_if = "Vec::is_empty", default)]
    pub protocols: Vec<String>,

    #[serde(rename = "de.sorunome.msc2409.push_ephemeral", skip_serializing_if = "Option::is_none")]
    pub soru_ephemeral_events: Option<bool>,

    #[serde(rename = "push_ephemeral", skip_serializing_if = "Option::is_none")]
    pub ephemeral_events: Option<bool>,

    #[serde(rename = "org.matrix.msc3202", skip_serializing_if = "Option::is_none")]
    pub msc3202: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Namespaces {
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<NamespaceList>,

    #[serde(rename = "aliases", skip_serializing_if = "Option::is_none")]
    pub room_aliases: Option<NamespaceList>,

    #[serde(rename = "rooms", skip_serializing_if = "Option::is_none")]
    pub room_ids: Option<NamespaceList>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Namespace {
    #[serde(rename = "regex")]
    pub regex: String,

    #[serde(rename = "exclusive")]
    pub exclusive: bool,
}

// Define NamespaceList as a vector of Namespace structs
type NamespaceList = Vec<Namespace>;


impl Registration {
    fn create_registration() -> Self {
        let app_token: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(64)
            .map(char::from)
            .collect();

        let server_token: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(64)
            .map(char::from)
            .collect();

        Registration {
            id: String::new(),
            url: String::new(),
            app_token,
            server_token,
            sender_localpart: String::new(),
            rate_limited: None,
            namespaces: Namespaces {
                user_ids: None,
                room_aliases: None,
                room_ids: None,
            },
            protocols: Vec::new(),
            soru_ephemeral_events: None,
            ephemeral_events: None,
            msc3202: None,
        }
    }
}