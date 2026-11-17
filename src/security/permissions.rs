use std::collections::HashMap;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PermissionState {
    Prompt,
    Allow,
    Deny,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainPermission {
    pub origin: String,
    pub camera: PermissionState,
    pub microphone: PermissionState,
    pub geolocation: PermissionState,
    pub notifications: PermissionState,
    pub clipboard_read: PermissionState,
}

impl DomainPermission {
    pub fn new(origin: String) -> Self {
        Self {
            origin,
            camera: PermissionState::Prompt,
            microphone: PermissionState::Prompt,
            geolocation: PermissionState::Prompt,
            notifications: PermissionState::Prompt,
            clipboard_read: PermissionState::Prompt,
        }
    }
}

pub struct PermissionFirewall {
    permissions: RwLock<HashMap<String, DomainPermission>>,
}

impl PermissionFirewall {
    pub fn new() -> Self {
        Self {
            permissions: RwLock::new(HashMap::new()),
        }
    }

    pub fn get_or_create(&self, origin: &str) -> DomainPermission {
        let mut map = self.permissions.write();
        map.entry(origin.to_string())
            .or_insert_with(|| DomainPermission::new(origin.to_string()))
            .clone()
    }

    pub fn set_permission(&self, origin: &str, perm: DomainPermission) {
        let mut map = self.permissions.write();
        map.insert(origin.to_string(), perm);
    }
}
