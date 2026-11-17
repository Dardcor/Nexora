use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use directories::ProjectDirs;
use parking_lot::RwLock;
use std::sync::Arc;

use super::crypto::VaultCrypto;
use super::models::{BookmarkItem, BrowserSpace, HistoryItem, TabSession};

pub struct LocalVault {
    vault_dir: PathBuf,
    crypto: Arc<VaultCrypto>,
    history: RwLock<Vec<HistoryItem>>,
    bookmarks: RwLock<Vec<BookmarkItem>>,
    spaces: RwLock<Vec<BrowserSpace>>,
    sessions: RwLock<Vec<TabSession>>,
}

impl LocalVault {
    /// Initialize the Local Vault with a master passphrase.
    /// The vault files are stored in the platform-standard local app data directory.
    pub fn init(passphrase: &str, custom_dir: Option<PathBuf>) -> Result<Self, Box<dyn std::error::Error>> {
        let vault_dir = match custom_dir {
            Some(dir) => dir,
            None => {
                let proj_dirs = ProjectDirs::from("com", "Dardcor", "Nexora")
                    .ok_or("Cannot determine user application directory")?;
                proj_dirs.data_local_dir().join("vault")
            }
        };

        fs::create_dir_all(&vault_dir)?;

        let salt_file = vault_dir.join("vault.salt");
        let (crypto, _salt) = if salt_file.exists() {
            let mut file = File::open(&salt_file)?;
            let mut existing_salt = Vec::new();
            file.read_to_end(&mut existing_salt)?;
            VaultCrypto::new_from_passphrase(passphrase, Some(&existing_salt))?
        } else {
            let (c, new_salt) = VaultCrypto::new_from_passphrase(passphrase, None)?;
            let mut file = File::create(&salt_file)?;
            file.write_all(&new_salt)?;
            (c, new_salt)
        };

        let crypto = Arc::new(crypto);

        let mut vault = Self {
            vault_dir,
            crypto,
            history: RwLock::new(Vec::new()),
            bookmarks: RwLock::new(Vec::new()),
            spaces: RwLock::new(Vec::new()),
            sessions: RwLock::new(Vec::new()),
        };

        // Load or initialize default data
        vault.load_all()?;

        if vault.spaces.read().is_empty() {
            vault.spaces.write().push(BrowserSpace::default());
            let _ = vault.save_spaces();
        }

        Ok(vault)
    }

    // --- Loading & Decrypting ---

    fn load_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let _ = self.load_history();
        let _ = self.load_bookmarks();
        let _ = self.load_spaces();
        let _ = self.load_sessions();
        Ok(())
    }

    fn read_and_decrypt<T: serde::de::DeserializeOwned>(&self, file_name: &str) -> Result<Vec<T>, Box<dyn std::error::Error>> {
        let file_path = self.vault_dir.join(file_name);
        if !file_path.exists() {
            return Ok(Vec::new());
        }

        let mut file = File::open(&file_path)?;
        let mut encrypted_data = Vec::new();
        file.read_to_end(&mut encrypted_data)?;

        let decrypted_bytes = self.crypto.decrypt(&encrypted_data)?;
        let items: Vec<T> = serde_json::from_slice(&decrypted_bytes)?;
        Ok(items)
    }

    fn encrypt_and_write<T: serde::Serialize>(&self, file_name: &str, items: &[T]) -> Result<(), Box<dyn std::error::Error>> {
        let serialized = serde_json::to_vec(items)?;
        let encrypted_bytes = self.crypto.encrypt(&serialized)?;

        let temp_path = self.vault_dir.join(format!("{}.{}.tmp", file_name, std::process::id()));
        let final_path = self.vault_dir.join(file_name);

        let mut file = File::create(&temp_path)?;
        file.write_all(&encrypted_bytes)?;
        file.sync_all()?;

        fs::rename(temp_path, final_path)?;
        Ok(())
    }

    // --- History Management ---

    pub fn load_history(&self) -> Result<(), Box<dyn std::error::Error>> {
        let items: Vec<HistoryItem> = self.read_and_decrypt("history.enc")?;
        *self.history.write() = items;
        Ok(())
    }

    pub fn save_history(&self) -> Result<(), Box<dyn std::error::Error>> {
        let history = self.history.read();
        self.encrypt_and_write("history.enc", &*history)
    }

    pub fn record_visit(&self, url: String, title: String, space_id: String) {
        {
            let mut hist = self.history.write();
            if let Some(existing) = hist.iter_mut().find(|h| h.url == url && h.space_id == space_id) {
                existing.visit_count += 1;
                existing.title = title;
                existing.visit_time = chrono::Utc::now();
            } else {
                hist.insert(0, HistoryItem::new(url, title, space_id));
            }
            if hist.len() > 10_000 {
                hist.truncate(10_000); // Cap history to keep it ultra lightweight
            }
        }
        let _ = self.save_history();
    }

    pub fn search_history(&self, query: &str) -> Vec<HistoryItem> {
        let query_lower = query.to_lowercase();
        let hist = self.history.read();
        hist.iter()
            .filter(|h| h.url.to_lowercase().contains(&query_lower) || h.title.to_lowercase().contains(&query_lower))
            .take(20)
            .cloned()
            .collect()
    }

    pub fn clear_history(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.history.write().clear();
        self.save_history()
    }

    // --- Bookmarks Management ---

    pub fn load_bookmarks(&self) -> Result<(), Box<dyn std::error::Error>> {
        let items: Vec<BookmarkItem> = self.read_and_decrypt("bookmarks.enc")?;
        *self.bookmarks.write() = items;
        Ok(())
    }

    pub fn save_bookmarks(&self) -> Result<(), Box<dyn std::error::Error>> {
        let bookmarks = self.bookmarks.read();
        self.encrypt_and_write("bookmarks.enc", &*bookmarks)
    }

    pub fn add_bookmark(&self, url: String, title: String, folder: String, tags: Vec<String>) {
        {
            let mut bm = self.bookmarks.write();
            if !bm.iter().any(|b| b.url == url) {
                bm.push(BookmarkItem::new(url, title, folder, tags));
            }
        }
        let _ = self.save_bookmarks();
    }

    pub fn get_bookmarks(&self) -> Vec<BookmarkItem> {
        self.bookmarks.read().clone()
    }

    // --- Spaces Management ---

    pub fn load_spaces(&self) -> Result<(), Box<dyn std::error::Error>> {
        let items: Vec<BrowserSpace> = self.read_and_decrypt("spaces.enc")?;
        *self.spaces.write() = items;
        Ok(())
    }

    pub fn save_spaces(&self) -> Result<(), Box<dyn std::error::Error>> {
        let spaces = self.spaces.read();
        self.encrypt_and_write("spaces.enc", &*spaces)
    }

    pub fn get_spaces(&self) -> Vec<BrowserSpace> {
        self.spaces.read().clone()
    }

    // --- Sessions Management ---

    pub fn load_sessions(&self) -> Result<(), Box<dyn std::error::Error>> {
        let items: Vec<TabSession> = self.read_and_decrypt("sessions.enc")?;
        *self.sessions.write() = items;
        Ok(())
    }

    pub fn save_sessions(&self, sessions: Vec<TabSession>) -> Result<(), Box<dyn std::error::Error>> {
        *self.sessions.write() = sessions;
        let s = self.sessions.read();
        self.encrypt_and_write("sessions.enc", &*s)
    }

    pub fn get_sessions(&self) -> Vec<TabSession> {
        self.sessions.read().clone()
    }
}
