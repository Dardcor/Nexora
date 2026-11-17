use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserTab {
    pub id: String,
    pub url: String,
    pub title: String,
    pub space_id: String,
    pub last_active: DateTime<Utc>,
    pub is_hibernated: bool,
    pub is_reader_mode: bool,
    pub scroll_y: u32,
    pub memory_saved_mb: f32,
}

impl BrowserTab {
    pub fn new(url: String, title: String, space_id: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            url,
            title,
            space_id,
            last_active: Utc::now(),
            is_hibernated: false,
            is_reader_mode: false,
            scroll_y: 0,
            memory_saved_mb: 0.0,
        }
    }

    /// Mark the tab as active, updating timestamp and waking from hibernation if needed.
    pub fn touch(&mut self) {
        self.last_active = Utc::now();
        if self.is_hibernated {
            self.is_hibernated = false;
        }
    }

    /// Put tab into deep hibernation, freeing up RAM.
    pub fn hibernate(&mut self) {
        if !self.is_hibernated {
            self.is_hibernated = true;
            // Estimated ~75MB RAM saved per hibernated webview tab
            self.memory_saved_mb += 75.0;
        }
    }
}
