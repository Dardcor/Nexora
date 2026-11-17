use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use url::Url;

pub struct NexoraShield {
    blocked_domains: HashSet<String>,
    blocked_patterns: Vec<String>,
    pub blocked_count: AtomicUsize,
    pub data_saved_est_bytes: AtomicUsize,
    pub is_enabled: AtomicBool,
}

impl NexoraShield {
    pub fn new() -> Self {
        let mut shield = Self {
            blocked_domains: HashSet::with_capacity(512),
            blocked_patterns: Vec::with_capacity(64),
            blocked_count: AtomicUsize::new(0),
            data_saved_est_bytes: AtomicUsize::new(0),
            is_enabled: AtomicBool::new(true),
        };

        shield.load_default_filter_rules();
        shield
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.is_enabled.store(enabled, Ordering::Relaxed);
    }

    pub fn is_enabled(&self) -> bool {
        self.is_enabled.load(Ordering::Relaxed)
    }

    fn load_default_filter_rules(&mut self) {
        let default_blocked = [
            "google-analytics.com",
            "googletagmanager.com",
            "googlesyndication.com",
            "doubleclick.net",
            "googleadservices.com",
            "pagead2.googlesyndication.com",
            "adservice.google.com",
            "telemetry.google.com",
            "connect.facebook.net",
            "pixel.facebook.com",
            "analytics.facebook.com",
            "graph.instagram.com",
            "ads.twitter.com",
            "analytics.twitter.com",
            "ads.tiktok.com",
            "analytics.tiktok.com",
            "ads.linkedin.com",
            "px.ads.linkedin.com",
            "adnxs.com",
            "criteo.com",
            "criteo.net",
            "outbrain.com",
            "taboola.com",
            "scorecardresearch.com",
            "hotjar.com",
            "clarity.ms",
            "mixpanel.com",
            "segment.io",
            "amplitude.com",
            "sentry.io",
            "newrelic.com",
            "quantserve.com",
            "chartbeat.com",
            "smartlook.com",
            "fullstory.com",
            "coinhive.com",
            "coin-hive.com",
            "crypto-loot.com",
            "fingerprintjs.com",
            "fpjs.sh",
        ];

        for domain in default_blocked {
            self.blocked_domains.insert(domain.to_string());
        }

        let patterns = [
            "/ads.js",
            "/pagead/",
            "/gtm.js",
            "/analytics.js",
            "/ga.js",
            "/fbevents.js",
            "/adserver/",
            "/tracking/",
            "/telemetry/",
            "beacon.min.js",
        ];

        for pattern in patterns {
            self.blocked_patterns.push(pattern.to_string());
        }
    }

    pub fn should_block(&self, url_str: &str) -> bool {
        if !self.is_enabled.load(Ordering::Relaxed) {
            return false;
        }

        if let Ok(url) = Url::parse(url_str) {
            if let Some(host) = url.host_str() {
                let host_lower = host.to_lowercase();

                for blocked in &self.blocked_domains {
                    if host_lower == *blocked || host_lower.ends_with(&format!(".{}", blocked)) {
                        self.blocked_count.fetch_add(1, Ordering::Relaxed);
                        self.data_saved_est_bytes.fetch_add(46_080, Ordering::Relaxed);
                        return true;
                    }
                }
            }

            let path_and_query = url.path();
            for pattern in &self.blocked_patterns {
                if path_and_query.contains(pattern) {
                    self.blocked_count.fetch_add(1, Ordering::Relaxed);
                    self.data_saved_est_bytes.fetch_add(25_600, Ordering::Relaxed);
                    return true;
                }
            }
        }

        false
    }

    pub fn add_blocked_domain(&mut self, domain: &str) {
        self.blocked_domains.insert(domain.to_lowercase());
    }

    pub fn get_statistics(&self) -> (usize, usize) {
        (
            self.blocked_count.load(Ordering::Relaxed),
            self.data_saved_est_bytes.load(Ordering::Relaxed),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shield_blocking_ads_and_trackers() {
        let shield = NexoraShield::new();

        assert!(shield.should_block("https://www.google-analytics.com/analytics.js"));
        assert!(shield.should_block("https://sub.doubleclick.net/ad/display"));
        assert!(shield.should_block("https://connect.facebook.net/en_US/fbevents.js"));
        assert!(shield.should_block("https://example.com/assets/ads.js"));

        assert!(!shield.should_block("https://github.com/Dardcor/Nexora"));
        assert!(!shield.should_block("https://rust-lang.org/learn"));
        assert!(!shield.should_block("https://duckduckgo.com/?q=rust"));
    }
}
