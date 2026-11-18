use parking_lot::RwLock;
use serde_json::Value;
use std::sync::Arc;

use crate::config::NexoraConfig;
use crate::engine::{BrowserTab, TabFreezer};
use crate::security::NexoraSecurityManager;
use crate::vault::NexoraDatabase;

pub struct NexoraApp {
    pub config: NexoraConfig,
    pub security: Arc<NexoraSecurityManager>,
    pub db: Arc<NexoraDatabase>,
    pub tabs: Arc<RwLock<Vec<BrowserTab>>>,
    pub active_tab_id: RwLock<Option<String>>,
    pub freezer: TabFreezer,
}

impl NexoraApp {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Self::new_with_mode(false)
    }

    pub fn new_with_mode(incognito: bool) -> Result<Self, Box<dyn std::error::Error>> {
        let config = NexoraConfig::default();
        let security = Arc::new(NexoraSecurityManager::new());
        let db = if incognito {
            Arc::new(NexoraDatabase::init_in_memory()?)
        } else {
            Arc::new(NexoraDatabase::init(None)?)
        };
        let shield_on = db.get_setting("shield_enabled", "true") == "true";
        security.shield.set_enabled(shield_on);

        let freezer = TabFreezer::new(config.hibernation_timeout_secs);

        let initial_tab = BrowserTab::new(
            "nexora://home".to_string(),
            "Nexora Home".to_string(),
            "personal".to_string(),
        );
        let initial_id = initial_tab.id.clone();

        let tabs = Arc::new(RwLock::new(vec![initial_tab]));
        let active_tab_id = RwLock::new(Some(initial_id));

        Ok(Self {
            config,
            security,
            db,
            tabs,
            active_tab_id,
            freezer,
        })
    }

    pub fn create_tab(&self, url: String, title: String, space_id: String) -> String {
        let tab = BrowserTab::new(url, title, space_id);
        let id = tab.id.clone();
        self.tabs.write().push(tab);
        *self.active_tab_id.write() = Some(id.clone());
        id
    }

    pub fn switch_tab(&self, id: &str) {
        let mut tabs = self.tabs.write();
        if let Some(tab) = tabs.iter_mut().find(|t| t.id == id) {
            tab.touch();
            *self.active_tab_id.write() = Some(id.to_string());
        }
    }

    pub fn close_tab(&self, id: &str) {
        let mut tabs = self.tabs.write();
        if tabs.len() <= 1 {
            return;
        }

        if let Some(pos) = tabs.iter().position(|t| t.id == id) {
            tabs.remove(pos);
            let next_index = if pos >= tabs.len() { tabs.len() - 1 } else { pos };
            let next_id = tabs[next_index].id.clone();
            *self.active_tab_id.write() = Some(next_id);
        }
    }

    pub fn navigate_active_tab(&self, raw_url: &str) -> Option<String> {
        let active_id = self.active_tab_id.read().clone()?;
        let filtered_url = self.security.inspect_and_filter_url(raw_url)?;

        let mut tabs = self.tabs.write();
        if let Some(tab) = tabs.iter_mut().find(|t| t.id == active_id) {
            tab.url = filtered_url.clone();
            tab.title = filtered_url.clone();
            tab.touch();

            let _ = self.db.record_visit(&filtered_url, &tab.title, &tab.space_id);
        }

        Some(filtered_url)
    }

    pub fn handle_ipc(&self, json_msg: &str) -> Option<String> {
        let v: Value = serde_json::from_str(json_msg).ok()?;
        let msg_type = v.get("type")?.as_str()?;

        match msg_type {
            "navigate" => {
                let url = v.get("url")?.as_str()?;
                self.navigate_active_tab(url)
            }
            "new_tab" => {
                let url = v.get("tab")?.get("url")?.as_str().unwrap_or("nexora://home");
                let title = v.get("tab")?.get("title")?.as_str().unwrap_or("Nexora Home");
                let space = v.get("tab")?.get("space")?.as_str().unwrap_or("personal");
                Some(self.create_tab(url.to_string(), title.to_string(), space.to_string()))
            }
            "switch_tab" => {
                let id = v.get("id")?.as_str()?;
                self.switch_tab(id);
                Some("switched".to_string())
            }
            "close_tab" => {
                let id = v.get("id")?.as_str()?;
                self.close_tab(id);
                Some("closed".to_string())
            }
            _ => None,
        }
    }
}
