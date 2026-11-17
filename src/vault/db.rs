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
    pub match_type: String, // "bookmark", "history", "search"
}

pub struct NexoraDatabase {
    conn: Arc<Mutex<Connection>>,
    pub db_path: PathBuf,
}

impl NexoraDatabase {
    /// Initialize local SQLite database in offline, secure, platform-native directory.
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

        // Performance & Reliability pragmas
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA foreign_keys = ON;
             PRAGMA temp_store = MEMORY;
             PRAGMA cache_size = -64000;",
        )?;

        // Create schema
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
            );",
        )?;

        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
            db_path,
        };

        db.seed_defaults()?;
        Ok(db)
    }

    fn seed_defaults(&self) -> SqlResult<()> {
        let conn = self.conn.lock();

        // Seed default settings
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

        // Seed default bookmarks if table empty
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

        Ok(())
    }

    // --- History Operations ---

    pub fn record_visit(&self, url: &str, title: &str, space_id: &str) -> SqlResult<()> {
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

    // --- Bookmarks Operations ---

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

    // --- Omnibox Autocomplete Suggestions ---

    pub fn get_omnibox_suggestions(&self, query: &str) -> SqlResult<Vec<OmniboxMatch>> {
        let clean = query.trim();
        if clean.is_empty() {
            return Ok(Vec::new());
        }

        let conn = self.conn.lock();
        let pattern = format!("%{}%", clean);
        let mut matches = Vec::new();

        // 1. Check matching bookmarks first
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

        // 2. Check matching history entries
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

    // --- Settings Operations ---

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

    // --- Shield Logging ---

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqlite_db_init_and_crud() {
        let temp_dir = std::env::temp_dir().join(format!("nexora_test_{}", Uuid::new_v4()));
        let db = NexoraDatabase::init(Some(temp_dir.clone())).expect("DB init failed");

        // 1. History test
        db.record_visit("https://github.com/Dardcor/Nexora", "Nexora Repo", "default").unwrap();
        db.record_visit("https://github.com/Dardcor/Nexora", "Nexora Browser", "default").unwrap();
        let history = db.query_history("Nexora", 10).unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].visit_count, 2);
        assert_eq!(history[0].title, "Nexora Browser");

        // 2. Bookmark test
        let b_id = db.add_bookmark("https://rust-lang.org", "Rust Language", "Bookmarks Bar").unwrap();
        assert!(!b_id.is_empty());
        assert!(db.is_bookmarked("https://rust-lang.org"));
        let bookmarks = db.get_bookmarks().unwrap();
        assert!(bookmarks.iter().any(|b| b.url == "https://rust-lang.org"));

        // 3. Omnibox suggestions
        let suggestions = db.get_omnibox_suggestions("rust").unwrap();
        assert!(!suggestions.is_empty());
        assert_eq!(suggestions[0].match_type, "bookmark");

        // 4. Settings test
        db.set_setting("search_engine", "duckduckgo").unwrap();
        let val = db.get_setting("search_engine", "google");
        assert_eq!(val, "duckduckgo");

        // 5. Shield stats
        db.record_shield_block("tracker.analytics.com").unwrap();
        assert!(db.get_shield_total_blocked() >= 1);

        let _ = fs::remove_dir_all(temp_dir);
    }
}

