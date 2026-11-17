pub mod fingerprint;
pub mod headers;
pub mod permissions;
pub mod shield;

pub use fingerprint::AntiFingerprint;
pub use headers::PrivacyHeaders;
pub use permissions::{DomainPermission, PermissionFirewall, PermissionState};
pub use shield::NexoraShield;

use std::sync::Arc;

pub struct NexoraSecurityManager {
    pub shield: Arc<NexoraShield>,
    pub permissions: Arc<PermissionFirewall>,
}

impl NexoraSecurityManager {
    pub fn new() -> Self {
        Self {
            shield: Arc::new(NexoraShield::new()),
            permissions: Arc::new(PermissionFirewall::new()),
        }
    }

    pub fn inspect_and_filter_url(&self, raw_url: &str) -> Option<String> {
        let cleaned = PrivacyHeaders::strip_tracking_params(raw_url);
        if self.shield.should_block(&cleaned) {
            None
        } else {
            Some(cleaned)
        }
    }
}
