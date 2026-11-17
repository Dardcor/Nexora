use chrono::Utc;
use directories::ProjectDirs;
use parking_lot::Mutex;
use rusqlite::{params, Connection, Result as SqlResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryRecord {
    pub id: i64,
    pub url: String,
    pub title: String,
    pub visit_time: i64,
    pub visit_count: u32,
    pub space_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkRecord {
    pub id: String,
    pub url: String,
    pub title: String,
    pub folder: String,
    pub created_at: i64,
    pub favicon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadRecord {
    pub id: String,
    pub filename: String,
    pub url: String,
    pub target_path: String,
    pub file_size: i64,
    pub status: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OmniboxMatch {
    pub url: String,
    pub title: String,
    pub match_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileRecord {
    pub id: String,
    pub name: String,
    pub email: String,
    pub avatar_url: String,
    pub badge: String,
    pub color: String,
    pub is_active: bool,
    pub created_at: i64,
}

pub struct NexoraDatabase {
    conn: Arc<Mutex<Connection>>,
    pub db_path: PathBuf,
}

impl NexoraDatabase {
    pub fn init(custom_dir: Option<PathBuf>) -> Result<Self, Box<dyn std::error::Error>> {
        let db_dir = match custom_dir {
            Some(dir) => dir,
            None => {
                let proj_dirs = ProjectDirs::from("com", "Dardcor", "Nexora")
                    .ok_or("Cannot determine user application directory")?;
                proj_dirs.data_local_dir().to_path_buf()
            }
        };

        fs::create_dir_all(&db_dir)?;
        let db_path = db_dir.join("nexora.db");

        let conn = Connection::open(&db_path)?;
        Self::setup_connection(conn, db_path)
    }

    pub fn init_in_memory() -> Result<Self, Box<dyn std::error::Error>> {
        let conn = Connection::open_in_memory()?;
        Self::setup_connection(conn, PathBuf::from(":memory:"))
    }

    fn setup_connection(conn: Connection, db_path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        conn.execute_batch(
            "PRAGMA busy_timeout = 5000;
             PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA foreign_keys = ON;
             PRAGMA temp_store = MEMORY;
             PRAGMA secure_delete = FAST;
             PRAGMA cache_size = -64000;",
        )?;

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                url TEXT NOT NULL,
                title TEXT NOT NULL,
                visit_time INTEGER NOT NULL,
                visit_count INTEGER NOT NULL DEFAULT 1,
                space_id TEXT NOT NULL DEFAULT 'default'
            );
            CREATE INDEX IF NOT EXISTS idx_history_url ON history(url);
            CREATE INDEX IF NOT EXISTS idx_history_time ON history(visit_time DESC);

            CREATE TABLE IF NOT EXISTS bookmarks (
                id TEXT PRIMARY KEY,
                url TEXT NOT NULL UNIQUE,
                title TEXT NOT NULL,
                folder TEXT NOT NULL DEFAULT 'Bookmarks Bar',
                created_at INTEGER NOT NULL,
                favicon TEXT
            );
            CREATE INDEX IF NOT EXISTS idx_bookmarks_folder ON bookmarks(folder);

            CREATE TABLE IF NOT EXISTS downloads (
                id TEXT PRIMARY KEY,
                filename TEXT NOT NULL,
                url TEXT NOT NULL,
                target_path TEXT NOT NULL,
                file_size INTEGER NOT NULL DEFAULT 0,
                status TEXT NOT NULL,
                created_at INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS shield_stats (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                blocked_domain TEXT NOT NULL,
                timestamp INTEGER NOT NULL,
                action TEXT NOT NULL DEFAULT 'blocked'
            );

            CREATE TABLE IF NOT EXISTS profiles (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                email TEXT NOT NULL,
                avatar_url TEXT NOT NULL DEFAULT '',
                badge TEXT NOT NULL DEFAULT 'Pro',
                color TEXT NOT NULL DEFAULT '#4285f4',
                is_active INTEGER NOT NULL DEFAULT 0,
                created_at INTEGER NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_profiles_active ON profiles(is_active);",
        )?;

        let _ = conn.execute(
            "DELETE FROM history WHERE url LIKE 'data:%' OR url LIKE 'about:%' OR url LIKE 'nexora:%';",
            [],
        );

        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
            db_path,
        };

        db.seed_defaults()?;
        Ok(db)
    }

    fn seed_defaults(&self) -> SqlResult<()> {
        let conn = self.conn.lock();

        let defaults = [
            ("search_engine", "google"),
            ("shield_enabled", "true"),
            ("show_bookmarks_bar", "true"),
            ("https_only", "true"),
            ("default_zoom", "100"),
        ];

        for (k, v) in defaults {
            conn.execute(
                "INSERT OR IGNORE INTO settings (key, value) VALUES (?1, ?2)",
                params![k, v],
            )?;
        }

        let count: i64 = conn.query_row("SELECT COUNT(*) FROM bookmarks", [], |r| r.get(0))?;
        if count == 0 {
            let initial_bookmarks = [
                ("GitHub", "https://github.com/Dardcor/Nexora", "Bookmarks Bar"),
                ("Google", "https://www.google.com", "Bookmarks Bar"),
                ("Rust Lang", "https://www.rust-lang.org", "Bookmarks Bar"),
                ("Hacker News", "https://news.ycombinator.com", "Bookmarks Bar"),
                ("Wikipedia", "https://www.wikipedia.org", "Bookmarks Bar"),
            ];

            let now = Utc::now().timestamp_millis();
            for (title, url, folder) in initial_bookmarks {
                let id = Uuid::new_v4().to_string();
                let _ = conn.execute(
                    "INSERT INTO bookmarks (id, url, title, folder, created_at, favicon) VALUES (?1, ?2, ?3, ?4, ?5, NULL)",
                    params![id, url, title, folder, now],
                );
            }
        }

        let _ = conn.execute("DELETE FROM profiles WHERE id IN ('prof-acip', 'prof-afdaniko', 'prof-akehu', 'prof-ardi', 'prof-berlaut', 'prof-de')", []);
        let _ = conn.execute("UPDATE profiles SET name = 'Person 1', email = '', badge = '' WHERE email = 'dardcorxyz@gmail.com'", []);

        let prof_count: i64 = conn.query_row("SELECT COUNT(*) FROM profiles", [], |r| r.get(0))?;
        if prof_count == 0 {
            let now = Utc::now().timestamp_millis();
            let _ = conn.execute(
                "INSERT INTO profiles (id, name, email, avatar_url, badge, color, is_active, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params!["prof-default", "Person 1", "", "", "", "#5865f2", 1, now],
            );
        }

        Ok(())
    }

    pub fn record_visit(&self, url: &str, title: &str, space_id: &str) -> SqlResult<()> {
        if url.is_empty() 
            || url.starts_with("data:") 
            || url.starts_with("about:") 
            || url.starts_with("nexora://") 
        {
            return Ok(());
        }

        let conn = self.conn.lock();
        let now = Utc::now().timestamp_millis();

        let updated = conn.execute(
            "UPDATE history SET title = ?1, visit_time = ?2, visit_count = visit_count + 1 WHERE url = ?3",
            params![title, now, url],
        )?;

        if updated == 0 {
            conn.execute(
                "INSERT INTO history (url, title, visit_time, visit_count, space_id) VALUES (?1, ?2, ?3, 1, ?4)",
                params![url, title, now, space_id],
            )?;
        }

        Ok(())
    }

    pub fn query_history(&self, search_term: &str, limit: usize) -> SqlResult<Vec<HistoryRecord>> {
        let conn = self.conn.lock();
        let mut results = Vec::new();

        if search_term.trim().is_empty() {
            let mut stmt = conn.prepare(
                "SELECT id, url, title, visit_time, visit_count, space_id FROM history ORDER BY visit_time DESC LIMIT ?1",
            )?;
            let rows = stmt.query_map(params![limit as i64], |row| {
                Ok(HistoryRecord {
                    id: row.get(0)?,
                    url: row.get(1)?,
                    title: row.get(2)?,
                    visit_time: row.get(3)?,
                    visit_count: row.get(4)?,
                    space_id: row.get(5)?,
                })
            })?;
            for r in rows {
                results.push(r?);
            }
        } else {
            let pattern = format!("%{}%", search_term.trim());
            let mut stmt = conn.prepare(
                "SELECT id, url, title, visit_time, visit_count, space_id FROM history WHERE url LIKE ?1 OR title LIKE ?1 ORDER BY visit_time DESC LIMIT ?2",
            )?;
            let rows = stmt.query_map(params![pattern, limit as i64], |row| {
                Ok(HistoryRecord {
                    id: row.get(0)?,
                    url: row.get(1)?,
                    title: row.get(2)?,
                    visit_time: row.get(3)?,
                    visit_count: row.get(4)?,
                    space_id: row.get(5)?,
                })
            })?;
            for r in rows {
                results.push(r?);
            }
        }

        Ok(results)
    }

    pub fn delete_history_item(&self, id: i64) -> SqlResult<()> {
        let conn = self.conn.lock();
        conn.execute("DELETE FROM history WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn clear_history(&self) -> SqlResult<()> {
        let conn = self.conn.lock();
        conn.execute("DELETE FROM history", [])?;
        Ok(())
    }

    pub fn add_bookmark(&self, url: &str, title: &str, folder: &str) -> SqlResult<String> {
        let conn = self.conn.lock();
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().timestamp_millis();

        conn.execute(
            "INSERT INTO bookmarks (id, url, title, folder, created_at, favicon) 
             VALUES (?1, ?2, ?3, ?4, ?5, NULL)
             ON CONFLICT(url) DO UPDATE SET title = excluded.title, folder = excluded.folder",
            params![id, url, title, folder, now],
        )?;

        Ok(id)
    }

    pub fn remove_bookmark(&self, url: &str) -> SqlResult<bool> {
        let conn = self.conn.lock();
        let rows = conn.execute("DELETE FROM bookmarks WHERE url = ?1", params![url])?;
        Ok(rows > 0)
    }

    pub fn is_bookmarked(&self, url: &str) -> bool {
        let conn = self.conn.lock();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM bookmarks WHERE url = ?1", params![url], |r| r.get(0))
            .unwrap_or(0);
        count > 0
    }

    pub fn get_bookmarks(&self) -> SqlResult<Vec<BookmarkRecord>> {
        let conn = self.conn.lock();
        let mut stmt = conn.prepare(
            "SELECT id, url, title, folder, created_at, favicon FROM bookmarks ORDER BY created_at ASC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(BookmarkRecord {
                id: row.get(0)?,
                url: row.get(1)?,
                title: row.get(2)?,
                folder: row.get(3)?,
                created_at: row.get(4)?,
                favicon: row.get(5)?,
            })
        })?;

        let mut results = Vec::new();
        for r in rows {
            results.push(r?);
        }
        Ok(results)
    }

    pub fn update_bookmark_favicon(&self, url: &str, favicon: &str) -> SqlResult<()> {
        let conn = self.conn.lock();
        conn.execute(
            "UPDATE bookmarks SET favicon = ?1 WHERE url = ?2",
            params![favicon, url],
        )?;
        Ok(())
    }

    pub fn record_download(
        &self,
        id: &str,
        filename: &str,
        url: &str,
        target_path: &str,
        status: &str,
    ) -> SqlResult<()> {
        let conn = self.conn.lock();
        let now = Utc::now().timestamp_millis();
        conn.execute(
            "INSERT OR REPLACE INTO downloads (id, filename, url, target_path, file_size, status, created_at)
             VALUES (?1, ?2, ?3, ?4, 0, ?5, ?6)",
            params![id, filename, url, target_path, status, now],
        )?;
        Ok(())
    }

    pub fn update_download_status(&self, url: &str, status: &str, file_size: i64) -> SqlResult<()> {
        let conn = self.conn.lock();
        conn.execute(
            "UPDATE downloads SET status = ?1, file_size = ?2 WHERE url = ?3",
            params![status, file_size, url],
        )?;
        Ok(())
    }

    pub fn get_downloads(&self) -> SqlResult<Vec<DownloadRecord>> {
        let conn = self.conn.lock();
        let mut stmt = conn.prepare(
            "SELECT id, filename, url, target_path, file_size, status, created_at FROM downloads ORDER BY created_at DESC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(DownloadRecord {
                id: row.get(0)?,
                filename: row.get(1)?,
                url: row.get(2)?,
                target_path: row.get(3)?,
                file_size: row.get(4)?,
                status: row.get(5)?,
                created_at: row.get(6)?,
            })
        })?;

        let mut results = Vec::new();
        for r in rows {
            results.push(r?);
        }
        Ok(results)
    }

    pub fn clear_downloads(&self) -> SqlResult<()> {
        let conn = self.conn.lock();
        conn.execute("DELETE FROM downloads", [])?;
        Ok(())
    }

    pub fn get_omnibox_suggestions(&self, query: &str) -> SqlResult<Vec<OmniboxMatch>> {
        let clean = query.trim();
        if clean.is_empty() {
            return Ok(Vec::new());
        }

        let conn = self.conn.lock();
        let pattern = format!("%{}%", clean);
        let mut matches = Vec::new();

        {
            let mut b_stmt = conn.prepare(
                "SELECT url, title FROM bookmarks WHERE url LIKE ?1 OR title LIKE ?1 LIMIT 3",
            )?;
            let b_rows = b_stmt.query_map(params![pattern], |row| {
                Ok(OmniboxMatch {
                    url: row.get(0)?,
                    title: row.get(1)?,
                    match_type: "bookmark".to_string(),
                })
            })?;
            for r in b_rows {
                if let Ok(m) = r {
                    matches.push(m);
                }
            }
        }

        {
            let mut h_stmt = conn.prepare(
                "SELECT url, title FROM history WHERE url LIKE ?1 OR title LIKE ?1 ORDER BY visit_count DESC, visit_time DESC LIMIT 4",
            )?;
            let h_rows = h_stmt.query_map(params![pattern], |row| {
                Ok(OmniboxMatch {
                    url: row.get(0)?,
                    title: row.get(1)?,
                    match_type: "history".to_string(),
                })
            })?;
            for r in h_rows {
                if let Ok(m) = r {
                    if !matches.iter().any(|existing| existing.url == m.url) {
                        matches.push(m);
                    }
                }
            }
        }

        Ok(matches)
    }

    pub fn get_setting(&self, key: &str, default_val: &str) -> String {
        let conn = self.conn.lock();
        conn.query_row(
            "SELECT value FROM settings WHERE key = ?1",
            params![key],
            |r| r.get(0),
        )
        .unwrap_or_else(|_| default_val.to_string())
    }

    pub fn set_setting(&self, key: &str, value: &str) -> SqlResult<()> {
        let conn = self.conn.lock();
        conn.execute(
            "INSERT INTO settings (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![key, value],
        )?;
        Ok(())
    }

    pub fn record_shield_block(&self, domain: &str) -> SqlResult<()> {
        let conn = self.conn.lock();
        let now = Utc::now().timestamp_millis();
        conn.execute(
            "INSERT INTO shield_stats (blocked_domain, timestamp, action) VALUES (?1, ?2, 'blocked')",
            params![domain, now],
        )?;
        Ok(())
    }

    pub fn get_shield_total_blocked(&self) -> i64 {
        let conn = self.conn.lock();
        conn.query_row("SELECT COUNT(*) FROM shield_stats", [], |r| r.get(0))
            .unwrap_or(0)
    }

    pub fn get_profiles(&self) -> SqlResult<Vec<ProfileRecord>> {
        let conn = self.conn.lock();
        let mut stmt = conn.prepare(
            "SELECT id, name, email, avatar_url, badge, color, is_active, created_at FROM profiles ORDER BY is_active DESC, created_at ASC",
        )?;
        let rows = stmt.query_map([], |row| {
            let active_int: i32 = row.get(6)?;
            Ok(ProfileRecord {
                id: row.get(0)?,
                name: row.get(1)?,
                email: row.get(2)?,
                avatar_url: row.get(3)?,
                badge: row.get(4)?,
                color: row.get(5)?,
                is_active: active_int != 0,
                created_at: row.get(7)?,
            })
        })?;

        let mut results = Vec::new();
        for r in rows {
            results.push(r?);
        }
        Ok(results)
    }

    pub fn get_active_profile(&self) -> SqlResult<Option<ProfileRecord>> {
        let conn = self.conn.lock();
        let mut stmt = conn.prepare(
            "SELECT id, name, email, avatar_url, badge, color, is_active, created_at FROM profiles WHERE is_active = 1 LIMIT 1",
        )?;
        let mut rows = stmt.query_map([], |row| {
            let active_int: i32 = row.get(6)?;
            Ok(ProfileRecord {
                id: row.get(0)?,
                name: row.get(1)?,
                email: row.get(2)?,
                avatar_url: row.get(3)?,
                badge: row.get(4)?,
                color: row.get(5)?,
                is_active: active_int != 0,
                created_at: row.get(7)?,
            })
        })?;

        if let Some(r) = rows.next() {
            Ok(Some(r?))
        } else {
            Ok(None)
        }
    }

    pub fn switch_profile(&self, id: &str) -> SqlResult<()> {
        let conn = self.conn.lock();
        conn.execute("UPDATE profiles SET is_active = 0", [])?;
        conn.execute("UPDATE profiles SET is_active = 1 WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn add_profile(&self, name: &str, email: &str, badge: &str, color: &str) -> SqlResult<ProfileRecord> {
        let conn = self.conn.lock();
        let id = format!("prof-{}", Uuid::new_v4());
        let now = Utc::now().timestamp_millis();
        conn.execute(
            "INSERT INTO profiles (id, name, email, avatar_url, badge, color, is_active, created_at)
             VALUES (?1, ?2, ?3, '', ?4, ?5, 0, ?6)",
            params![id, name, email, badge, color, now],
        )?;

        Ok(ProfileRecord {
            id,
            name: name.to_string(),
            email: email.to_string(),
            avatar_url: String::new(),
            badge: badge.to_string(),
            color: color.to_string(),
            is_active: false,
            created_at: now,
        })
    }

    pub fn update_profile(&self, id: &str, name: &str, email: &str, badge: &str, color: &str) -> SqlResult<()> {
        let conn = self.conn.lock();
        conn.execute(
            "UPDATE profiles SET name = ?1, email = ?2, badge = ?3, color = ?4 WHERE id = ?5",
            params![name, email, badge, color, id],
        )?;
        Ok(())
    }

    pub fn sign_out(&self) -> SqlResult<()> {
        let conn = self.conn.lock();
        conn.execute(
            "UPDATE profiles SET email = '', badge = '', avatar_url = '' WHERE is_active = 1",
            [],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqlite_db_init_and_crud() {
        let temp_dir = std::env::temp_dir().join(format!("nexora_test_{}", Uuid::new_v4()));
        let db = NexoraDatabase::init(Some(temp_dir.clone())).expect("DB init failed");

        db.record_visit("https://github.com/Dardcor/Nexora", "Nexora Repo", "default").unwrap();
        db.record_visit("https://github.com/Dardcor/Nexora", "Nexora Browser", "default").unwrap();
        let history = db.query_history("Nexora", 10).unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].visit_count, 2);
        assert_eq!(history[0].title, "Nexora Browser");

        let b_id = db.add_bookmark("https://rust-lang.org", "Rust Language", "Bookmarks Bar").unwrap();
        assert!(!b_id.is_empty());
        assert!(db.is_bookmarked("https://rust-lang.org"));
        let bookmarks = db.get_bookmarks().unwrap();
        assert!(bookmarks.iter().any(|b| b.url == "https://rust-lang.org"));

        let suggestions = db.get_omnibox_suggestions("rust").unwrap();
        assert!(!suggestions.is_empty());
        assert_eq!(suggestions[0].match_type, "bookmark");

        db.set_setting("search_engine", "duckduckgo").unwrap();
        let val = db.get_setting("search_engine", "google");
        assert_eq!(val, "duckduckgo");

        db.record_shield_block("tracker.analytics.com").unwrap();
        assert!(db.get_shield_total_blocked() >= 1);

        let profiles = db.get_profiles().unwrap();
        assert!(!profiles.is_empty());
        let active = db.get_active_profile().unwrap();
        assert!(active.is_some());
        assert_eq!(active.unwrap().name, "Person 1");

        db.sign_out().unwrap();
        let after_signout = db.get_active_profile().unwrap().unwrap();
        assert_eq!(after_signout.email, "");

        let _ = fs::remove_dir_all(temp_dir);
    }
}
