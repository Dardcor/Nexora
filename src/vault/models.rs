use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryItem {
    pub id: String,
    pub url: String,
    pub title: String,
    pub visit_time: DateTime<Utc>,
    pub visit_count: u32,
    pub space_id: String,
}

impl HistoryItem {
    pub fn new(url: String, title: String, space_id: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            url,
            title,
            visit_time: Utc::now(),
            visit_count: 1,
            space_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkItem {
    pub id: String,
    pub url: String,
    pub title: String,
    pub folder: String,
    pub created_at: DateTime<Utc>,
    pub tags: Vec<String>,
}

impl BookmarkItem {
    pub fn new(url: String, title: String, folder: String, tags: Vec<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            url,
            title,
            folder,
            created_at: Utc::now(),
            tags,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserSpace {
    pub id: String,
    pub name: String,
    pub color: String,
    pub icon: String,
    pub is_default: bool,
}

impl Default for BrowserSpace {
    fn default() -> Self {
        Self {
            id: "default-space".to_string(),
            name: "Personal".to_string(),
            color: "#3B82F6".to_string(),
            icon: "user".to_string(),
            is_default: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabSession {
    pub id: String,
    pub url: String,
    pub title: String,
    pub space_id: String,
    pub scroll_y: u32,
    pub last_active: DateTime<Utc>,
    pub is_hibernated: bool,
}
