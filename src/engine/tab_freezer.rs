use chrono::{Duration, Utc};
use parking_lot::RwLock;
use std::sync::Arc;
use super::tab::BrowserTab;

pub struct TabFreezer {
    hibernation_timeout_secs: i64,
}

impl TabFreezer {
    pub fn new(hibernation_timeout_secs: i64) -> Self {
        Self {
            hibernation_timeout_secs,
        }
    }

    pub fn sweep_and_hibernate(
        &self,
        tabs: &Arc<RwLock<Vec<BrowserTab>>>,
        active_tab_id: Option<&str>,
    ) -> (usize, f32) {
        let now = Utc::now();
        let timeout = Duration::seconds(self.hibernation_timeout_secs);
        let mut count = 0;
        let mut total_saved_mb = 0.0;

        let mut tabs_guard = tabs.write();
        for tab in tabs_guard.iter_mut() {
            if let Some(active_id) = active_tab_id {
                if tab.id == active_id {
                    continue;
                }
            }

            if !tab.is_hibernated && (now - tab.last_active) >= timeout {
                tab.hibernate();
                count += 1;
                total_saved_mb += 75.0;
            }
        }

        (count, total_saved_mb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_hibernation_sweep() {
        let tab1 = BrowserTab::new("https://rust-lang.org".to_string(), "Rust".to_string(), "default".to_string());
        let tab2 = BrowserTab::new("https://github.com/Dardcor/Nexora".to_string(), "Nexora".to_string(), "default".to_string());
        let active_id = tab1.id.clone();

        let tabs = Arc::new(RwLock::new(vec![tab1, tab2]));

        let freezer = TabFreezer::new(0);

        let (hibernated_count, memory_saved) = freezer.sweep_and_hibernate(&tabs, Some(&active_id));
        
        assert_eq!(hibernated_count, 1);
        assert_eq!(memory_saved, 75.0);

        let read_tabs = tabs.read();
        assert!(!read_tabs[0].is_hibernated, "Active tab must not be hibernated");
        assert!(read_tabs[1].is_hibernated, "Inactive background tab should be hibernated");
    }
}
