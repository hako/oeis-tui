#![allow(dead_code)]

use super::models::{OEISResponse, Sequence};
use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};
use rusqlite::{params, Connection};
use std::path::PathBuf;

use crate::utils::paths;

/// SQLite-based cache for OEIS API responses
pub struct Cache {
    conn: Connection,
}

impl Cache {
    /// Create or open the cache database
    pub fn new() -> Result<Self> {
        let cache_dir = Self::cache_dir()?;
        std::fs::create_dir_all(&cache_dir)?;

        let db_path = cache_dir.join("oeis_cache.db");

        let conn = Connection::open(db_path).context("Failed to open cache database")?;

        let cache = Self { conn };
        cache.init_tables()?;

        Ok(cache)
    }

    /// Initialize database tables
    fn init_tables(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS sequence_cache (
                query TEXT PRIMARY KEY,
                response TEXT NOT NULL,
                cached_at TEXT NOT NULL
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS sequence_detail (
                number INTEGER PRIMARY KEY,
                data TEXT NOT NULL,
                cached_at TEXT NOT NULL
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS search_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                query TEXT NOT NULL,
                searched_at TEXT NOT NULL
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS viewed_sequences (
                number INTEGER PRIMARY KEY,
                viewed_at TEXT NOT NULL,
                view_count INTEGER DEFAULT 1
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS bookmarks (
                number INTEGER PRIMARY KEY,
                bookmarked_at TEXT NOT NULL,
                notes TEXT
            )",
            [],
        )?;

        Ok(())
    }

    /// Get the cache directory path
    fn cache_dir() -> Result<PathBuf> {
        paths::ensure_config_dir()
    }

    /// Cache a search response
    pub fn cache_search(&self, query: &str, response: &OEISResponse) -> Result<()> {
        let response_json = serde_json::to_string(response)?;
        let now = Utc::now().to_rfc3339();

        self.conn.execute(
            "INSERT OR REPLACE INTO sequence_cache (query, response, cached_at) VALUES (?1, ?2, ?3)",
            params![query, response_json, now],
        )?;

        Ok(())
    }

    /// Get a cached search response
    /// Returns None if not cached or cache is too old
    pub fn get_cached_search(
        &self,
        query: &str,
        max_age_days: i64,
    ) -> Result<Option<OEISResponse>> {
        let mut stmt = self
            .conn
            .prepare("SELECT response, cached_at FROM sequence_cache WHERE query = ?1")?;

        let result = stmt.query_row(params![query], |row| {
            let response_json: String = row.get(0)?;
            let cached_at: String = row.get(1)?;
            Ok((response_json, cached_at))
        });

        match result {
            Ok((response_json, cached_at)) => {
                // Check if cache is still valid
                let cached_time = DateTime::parse_from_rfc3339(&cached_at)
                    .context("Invalid cached_at timestamp")?
                    .with_timezone(&Utc);

                let age = Utc::now() - cached_time;

                if age > Duration::days(max_age_days) {
                    return Ok(None);
                }

                let response: OEISResponse = serde_json::from_str(&response_json)
                    .context("Failed to deserialize cached response")?;

                Ok(Some(response))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Cache a single sequence
    pub fn cache_sequence(&self, sequence: &Sequence) -> Result<()> {
        let data_json = serde_json::to_string(sequence)?;
        let now = Utc::now().to_rfc3339();

        self.conn.execute(
            "INSERT OR REPLACE INTO sequence_detail (number, data, cached_at) VALUES (?1, ?2, ?3)",
            params![sequence.number, data_json, now],
        )?;

        Ok(())
    }

    /// Get a cached sequence by number
    pub fn get_cached_sequence(&self, number: i32, max_age_days: i64) -> Result<Option<Sequence>> {
        let mut stmt = self
            .conn
            .prepare("SELECT data, cached_at FROM sequence_detail WHERE number = ?1")?;

        let result = stmt.query_row(params![number], |row| {
            let data_json: String = row.get(0)?;
            let cached_at: String = row.get(1)?;
            Ok((data_json, cached_at))
        });

        match result {
            Ok((data_json, cached_at)) => {
                // Check if cache is still valid
                let cached_time = DateTime::parse_from_rfc3339(&cached_at)
                    .context("Invalid cached_at timestamp")?
                    .with_timezone(&Utc);

                let age = Utc::now() - cached_time;

                if age > Duration::days(max_age_days) {
                    return Ok(None);
                }

                let sequence: Sequence = serde_json::from_str(&data_json)
                    .context("Failed to deserialize cached sequence")?;

                Ok(Some(sequence))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Add a search to history
    pub fn add_search_history(&self, query: &str) -> Result<()> {
        let now = Utc::now().to_rfc3339();

        self.conn.execute(
            "INSERT INTO search_history (query, searched_at) VALUES (?1, ?2)",
            params![query, now],
        )?;

        Ok(())
    }

    /// Get recent search history
    pub fn get_search_history(&self, limit: usize) -> Result<Vec<(String, DateTime<Utc>)>> {
        let mut stmt = self.conn.prepare(
            "SELECT DISTINCT query, searched_at FROM search_history ORDER BY searched_at DESC LIMIT ?1"
        )?;

        let history = stmt.query_map(params![limit], |row| {
            let query: String = row.get(0)?;
            let searched_at: String = row.get(1)?;
            Ok((query, searched_at))
        })?;

        let mut results = Vec::new();
        for item in history {
            let (query, searched_at_str) = item?;
            let searched_at = DateTime::parse_from_rfc3339(&searched_at_str)
                .context("Invalid searched_at timestamp")?
                .with_timezone(&Utc);
            results.push((query, searched_at));
        }

        Ok(results)
    }

    /// Record that a sequence was viewed
    pub fn record_view(&self, number: i32) -> Result<()> {
        let now = Utc::now().to_rfc3339();

        self.conn.execute(
            "INSERT INTO viewed_sequences (number, viewed_at, view_count) VALUES (?1, ?2, 1)
             ON CONFLICT(number) DO UPDATE SET viewed_at = ?2, view_count = view_count + 1",
            params![number, now],
        )?;

        Ok(())
    }

    /// Get recently viewed sequences
    pub fn get_recently_viewed(&self, limit: usize) -> Result<Vec<i32>> {
        let mut stmt = self
            .conn
            .prepare("SELECT number FROM viewed_sequences ORDER BY viewed_at DESC LIMIT ?1")?;

        let numbers = stmt.query_map(params![limit], |row| row.get(0))?;

        let mut results = Vec::new();
        for number in numbers {
            results.push(number?);
        }

        Ok(results)
    }

    /// Get recently viewed sequences with details (number, name, view_count, viewed_at)
    pub fn get_recently_viewed_with_details(
        &self,
        limit: usize,
    ) -> Result<Vec<(i32, String, i32, String)>> {
        let mut stmt = self.conn.prepare(
            "SELECT v.number, COALESCE(s.data, ''), v.view_count, v.viewed_at
             FROM viewed_sequences v
             LEFT JOIN sequence_detail s ON v.number = s.number
             ORDER BY v.viewed_at DESC
             LIMIT ?1",
        )?;

        let rows = stmt.query_map(params![limit], |row| {
            Ok((
                row.get::<_, i32>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i32>(2)?,
                row.get::<_, String>(3)?,
            ))
        })?;

        let mut results = Vec::new();
        for row in rows {
            let (number, data_json, view_count, viewed_at) = row?;

            // Parse sequence data to extract name
            let name = if !data_json.is_empty() {
                if let Ok(seq) = serde_json::from_str::<Sequence>(&data_json) {
                    seq.name.clone()
                } else {
                    // JSON parsing failed - show unknown
                    "(Name unavailable)".to_string()
                }
            } else {
                // No cached data - skip this entry by continuing to next iteration
                continue;
            };

            results.push((number, name, view_count, viewed_at));
        }

        Ok(results)
    }

    /// Add a bookmark
    pub fn add_bookmark(&self, number: i32, notes: Option<&str>) -> Result<()> {
        let now = Utc::now().to_rfc3339();

        self.conn.execute(
            "INSERT OR REPLACE INTO bookmarks (number, bookmarked_at, notes) VALUES (?1, ?2, ?3)",
            params![number, now, notes],
        )?;

        Ok(())
    }

    /// Remove a bookmark
    pub fn remove_bookmark(&self, number: i32) -> Result<()> {
        self.conn
            .execute("DELETE FROM bookmarks WHERE number = ?1", params![number])?;

        Ok(())
    }

    /// Get all bookmarks
    pub fn get_bookmarks(&self) -> Result<Vec<(i32, Option<String>)>> {
        let mut stmt = self
            .conn
            .prepare("SELECT number, notes FROM bookmarks ORDER BY bookmarked_at DESC")?;

        let bookmarks = stmt.query_map([], |row| {
            let number: i32 = row.get(0)?;
            let notes: Option<String> = row.get(1)?;
            Ok((number, notes))
        })?;

        let mut results = Vec::new();
        for bookmark in bookmarks {
            results.push(bookmark?);
        }

        Ok(results)
    }

    /// Check if a sequence is bookmarked
    pub fn is_bookmarked(&self, number: i32) -> Result<bool> {
        let mut stmt = self
            .conn
            .prepare("SELECT COUNT(*) FROM bookmarks WHERE number = ?1")?;

        let count: i32 = stmt.query_row(params![number], |row| row.get(0))?;

        Ok(count > 0)
    }

    /// Clear all cache (but keep history and bookmarks)
    pub fn clear_cache(&self) -> Result<()> {
        self.conn.execute("DELETE FROM sequence_cache", [])?;
        self.conn.execute("DELETE FROM sequence_detail", [])?;
        Ok(())
    }

    /// Get cache statistics
    pub fn get_stats(&self) -> Result<CacheStats> {
        let cached_searches: i32 =
            self.conn
                .query_row("SELECT COUNT(*) FROM sequence_cache", [], |row| row.get(0))?;

        let cached_sequences: i32 =
            self.conn
                .query_row("SELECT COUNT(*) FROM sequence_detail", [], |row| row.get(0))?;

        let total_searches: i32 =
            self.conn
                .query_row("SELECT COUNT(*) FROM search_history", [], |row| row.get(0))?;

        let viewed_sequences: i32 =
            self.conn
                .query_row("SELECT COUNT(*) FROM viewed_sequences", [], |row| {
                    row.get(0)
                })?;

        let bookmarked_sequences: i32 =
            self.conn
                .query_row("SELECT COUNT(*) FROM bookmarks", [], |row| row.get(0))?;

        Ok(CacheStats {
            cached_searches,
            cached_sequences,
            total_searches,
            viewed_sequences,
            bookmarked_sequences,
        })
    }
}

/// Cache statistics
#[derive(Debug)]
pub struct CacheStats {
    pub cached_searches: i32,
    pub cached_sequences: i32,
    pub total_searches: i32,
    pub viewed_sequences: i32,
    pub bookmarked_sequences: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::models::Sequence;

    fn create_test_sequence(number: i32) -> Sequence {
        Sequence {
            number,
            id: String::new(),
            data: "1,2,3,4,5".to_string(),
            name: "Test sequence".to_string(),
            offset: "0,1".to_string(),
            comment: vec![],
            reference: vec![],
            link: vec![],
            formula: vec![],
            example: vec![],
            maple: vec![],
            mathematica: vec![],
            program: vec![],
            xref: vec![],
            keyword: "test".to_string(),
            author: String::new(),
            created: String::new(),
            time: String::new(),
            references: 0,
            revision: 0,
        }
    }

    #[test]
    fn test_cache_creation() {
        let cache = Cache::new();
        assert!(cache.is_ok());
    }

    #[test]
    fn test_cache_sequence() {
        let cache = Cache::new().unwrap();
        let seq = create_test_sequence(123);

        assert!(cache.cache_sequence(&seq).is_ok());

        let cached = cache.get_cached_sequence(123, 365).unwrap();
        assert!(cached.is_some());

        let cached_seq = cached.unwrap();
        assert_eq!(cached_seq.number, 123);
        assert_eq!(cached_seq.name, "Test sequence");
    }

    #[test]
    fn test_search_history() {
        let cache = Cache::new().unwrap();

        assert!(cache.add_search_history("fibonacci").is_ok());
        assert!(cache.add_search_history("prime numbers").is_ok());

        let history = cache.get_search_history(10).unwrap();
        assert!(history.len() >= 2);
    }

    #[test]
    fn test_bookmarks() {
        let cache = Cache::new().unwrap();

        assert!(cache.add_bookmark(45, Some("Fibonacci sequence")).is_ok());
        assert!(cache.is_bookmarked(45).unwrap());

        let bookmarks = cache.get_bookmarks().unwrap();
        assert!(!bookmarks.is_empty());

        assert!(cache.remove_bookmark(45).is_ok());
        assert!(!cache.is_bookmarked(45).unwrap());
    }
}
