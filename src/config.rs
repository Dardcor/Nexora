use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchEngine {
    DuckDuckGo,
    BraveSearch,
    SearXNG(String),
}

impl SearchEngine {
    pub fn build_search_url(&self, query: &str) -> String {
        let encoded_query = url::form_urlencoded::byte_serialize(query.as_bytes()).collect::<String>();
        match self {
            SearchEngine::DuckDuckGo => format!("https://duckduckgo.com/?q={}", encoded_query),
            SearchEngine::BraveSearch => format!("https://search.brave.com/search?q={}", encoded_query),
            SearchEngine::SearXNG(instance) => format!("{}/search?q={}", instance.trim_end_matches('/'), encoded_query),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NexoraConfig {
    pub search_engine: SearchEngine,
    pub dark_mode: bool,
    pub hibernation_timeout_secs: i64,
    pub shield_enabled: bool,
    pub anti_fingerprint_enabled: bool,
    pub homepage: String,
}

impl Default for NexoraConfig {
    fn default() -> Self {
        Self {
            search_engine: SearchEngine::DuckDuckGo,
            dark_mode: true,
            hibernation_timeout_secs: 300, // 5 minutes default
            shield_enabled: true,
            anti_fingerprint_enabled: true,
            homepage: "nexora://newtab".to_string(),
        }
    }
}
