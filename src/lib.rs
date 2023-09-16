use reqwest::Client as ReqwestClient;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use postgrest;
use url::Url;
use reqwest::header::{HeaderMap, HeaderValue, IntoHeaderName};

#[derive(Debug)]
pub struct SupabaseClient {
    supabase_url: Url,
    supabase_key: String,
    realtime_url: Url,
    auth_url: Url,
    storage_url: Url,
    functions_url: Url,
    headers: HashMap<String, String>,
}

impl SupabaseClient {
    pub fn new(supabase_url: &str, supabase_key: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let supabase_url = Url::parse(supabase_url)?;
        let realtime_url = Url::parse(&format!("{}/realtime/v1", supabase_url))?;
        let auth_url = Url::parse(&format!("{}/auth/v1", supabase_url))?;
        let storage_url = Url::parse(&format!("{}/storage/v1", supabase_url))?;
        let functions_url = Url::parse(&format!("{}/functions/v1", supabase_url))?;

        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", supabase_key));
        headers.insert("apikey".to_string(), supabase_key.to_string());

        Ok(Self {
            supabase_url,
            supabase_key: supabase_key.to_string(),
            realtime_url,
            auth_url,
            storage_url,
            functions_url,
            headers,
        })
    }

    pub fn from<T>(&self, relation: T) -> PostgrestQueryBuilder
    where
        T: ToString,
    {
        let relation_name = relation.to_string();
        // Implement the PostgrestQueryBuilder and return it
        PostgrestQueryBuilder::new(relation_name, &self.supabase_url, &self.headers)
    }

    // Implement other methods such as `schema`, `rpc`, `channel`, `getChannels`, etc.
}

#[derive(Debug)]
pub struct SupabaseAuthClientOptions {
    pub auto_refresh_token: bool,
    pub persist_session: bool,
    pub detect_session_in_url: bool,
    pub flow_type: String,
}

impl SupabaseAuthClientOptions {
    fn default() -> Self {
        Self {
            auto_refresh_token: true,
            persist_session: true,
            detect_session_in_url: true,
            flow_type: "implicit".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct PostgrestQueryBuilder {
    relation: String,
    base_url: Url,
    headers: HashMap<String, String>,
    // Add other fields as needed
}

impl PostgrestQueryBuilder {
    fn new(relation: String, supabase_url: &Url, headers: &HashMap<String, String>) -> Self {
        let base_url = supabase_url.join("/rest/v1").expect("Invalid URL");
        Self {
            relation,
            base_url,
            headers: headers.clone(),
        }
    }

    // Implement methods for building queries
}

// OPTIONS
//need to implement realtime from @supabase/realtime-js
#[derive(Debug)]
pub struct RealtimeClientOptions {}


#[derive(Debug, Deserialize, Serialize)]
pub struct DefaultDbOptions {
    pub schema: String,
}

impl DefaultDbOptions {
    fn default() -> Self {
        Self {
            schema: "public".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct SupabaseStorageClientOptions {}

