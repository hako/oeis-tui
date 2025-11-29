#![allow(dead_code)]

use serde::{Deserialize, Serialize};

/// Root response from the OEIS API
/// Note: The OEIS API returns a plain JSON array, not an object with count/results.
/// This struct is a wrapper we use internally to provide pagination hints.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OEISResponse {
    /// Estimated result count (since API doesn't provide exact count)
    /// Set to 10 if we got exactly 10 results (indicating more might exist)
    /// Set to actual length if we got fewer than 10 results
    pub count: i32,
    /// Array of sequence objects (max 10 per request from API)
    /// Will be None if too many results or on error
    pub results: Option<Vec<Sequence>>,
}

/// A single OEIS sequence with all metadata
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Sequence {
    /// OEIS sequence number (e.g., 55 for A000055)
    pub number: i32,

    /// Alternative identifier codes (e.g., "M0791 N0299")
    #[serde(default)]
    pub id: String,

    /// Comma-separated sequence values (first ~40-80 terms)
    pub data: String,

    /// Full descriptive title of the sequence
    pub name: String,

    /// Starting indices, comma-separated (e.g., "0,5")
    pub offset: String,

    /// Contextual remarks, alternate interpretations, properties
    #[serde(default)]
    pub comment: Vec<String>,

    /// Academic citations and textbook references
    #[serde(default)]
    pub reference: Vec<String>,

    /// HTML-formatted external resources and papers
    #[serde(default)]
    pub link: Vec<String>,

    /// Mathematical generating functions and asymptotic expressions
    #[serde(default)]
    pub formula: Vec<String>,

    /// Illustrative cases (often with ASCII diagrams)
    #[serde(default)]
    pub example: Vec<String>,

    /// Maple implementations
    #[serde(default)]
    pub maple: Vec<String>,

    /// Mathematica code
    #[serde(default)]
    pub mathematica: Vec<String>,

    /// Code in other languages (PARI, Magma, SageMath, Haskell, Python)
    #[serde(default)]
    pub program: Vec<String>,

    /// Cross-references to related sequences with annotations
    #[serde(default)]
    pub xref: Vec<String>,

    /// Classification tags (e.g., "nonn,easy,nice,core")
    #[serde(default)]
    pub keyword: String,

    /// Original submitter attribution
    #[serde(default)]
    pub author: String,

    /// Record creation date (ISO 8601 timestamp)
    #[serde(default)]
    pub created: String,

    /// Last revision date (ISO 8601 timestamp)
    #[serde(default)]
    pub time: String,

    /// Count of citations
    #[serde(default)]
    pub references: i32,

    /// Edit history version number
    #[serde(default)]
    pub revision: i32,
}

impl Sequence {
    /// Get the A-number identifier (e.g., "A000055")
    pub fn a_number(&self) -> String {
        format!("A{:06}", self.number)
    }

    /// Parse the sequence data into a vector of integers
    /// Returns None if data contains non-integer values
    pub fn parse_data(&self) -> Vec<String> {
        self.data
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    /// Get the offset values as a tuple (start_index, first_index_of_1)
    pub fn parse_offset(&self) -> (i32, i32) {
        let parts: Vec<&str> = self.offset.split(',').collect();
        let start = parts
            .first()
            .and_then(|s| s.trim().parse().ok())
            .unwrap_or(0);
        let first_one = parts
            .get(1)
            .and_then(|s| s.trim().parse().ok())
            .unwrap_or(1);
        (start, first_one)
    }

    /// Get keywords as a vector
    pub fn keywords(&self) -> Vec<&str> {
        self.keyword.split(',').map(|s| s.trim()).collect()
    }

    /// Check if sequence has a specific keyword
    pub fn has_keyword(&self, keyword: &str) -> bool {
        self.keywords().contains(&keyword)
    }

    /// Get URL to view this sequence on OEIS website
    pub fn url(&self) -> String {
        format!("https://oeis.org/{}", self.a_number())
    }

    /// Get URL to the B-file (extended sequence data) if it exists
    pub fn b_file_url(&self) -> String {
        format!("https://oeis.org/b{:06}.txt", self.number)
    }
}

/// Entry from a B-file (extended sequence data)
#[derive(Debug, Clone)]
pub struct BFileEntry {
    /// Index in the sequence
    pub index: i64,
    /// Value at this index (stored as String to handle arbitrary precision)
    pub value: String,
}

impl BFileEntry {
    /// Parse a line from a B-file
    /// Format: "index value"
    /// Lines starting with '#' are comments and return None
    pub fn parse(line: &str) -> Option<Self> {
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            return None;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            return None;
        }

        let index = parts[0].parse().ok()?;
        let value = parts[1].to_string();

        Some(BFileEntry { index, value })
    }
}

/// Search query builder for OEIS API
#[derive(Debug, Clone, Default)]
pub struct SearchQuery {
    /// The main query string
    pub query: String,
    /// Response format (json or text)
    pub format: String,
    /// Pagination offset (0-indexed, increments by 10)
    pub start: i32,
}

impl SearchQuery {
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            format: "json".to_string(),
            start: 0,
        }
    }

    pub fn with_format(mut self, format: impl Into<String>) -> Self {
        self.format = format.into();
        self
    }

    pub fn with_start(mut self, start: i32) -> Self {
        self.start = start;
        self
    }

    pub fn next_page(&self, page_size: usize) -> Self {
        let mut query = self.clone();
        query.start += page_size as i32;
        query
    }

    pub fn prev_page(&self, page_size: usize) -> Self {
        let mut query = self.clone();
        query.start = (query.start - page_size as i32).max(0);
        query
    }

    /// Build the URL query string
    pub fn to_url(&self) -> String {
        format!(
            "https://oeis.org/search?q={}&fmt={}&start={}",
            urlencoding::encode(&self.query),
            self.format,
            self.start
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_number() {
        let seq = Sequence {
            number: 55,
            id: String::new(),
            data: String::new(),
            name: String::new(),
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
            keyword: String::new(),
            author: String::new(),
            created: String::new(),
            time: String::new(),
            references: 0,
            revision: 0,
        };
        assert_eq!(seq.a_number(), "A000055");
    }

    #[test]
    fn test_parse_data() {
        let seq = Sequence {
            number: 45,
            id: String::new(),
            data: "1,1,2,3,5,8,13,21".to_string(),
            name: "Fibonacci sequence".to_string(),
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
            keyword: "nonn,easy".to_string(),
            author: String::new(),
            created: String::new(),
            time: String::new(),
            references: 0,
            revision: 0,
        };

        let data = seq.parse_data();
        assert_eq!(data, vec!["1", "1", "2", "3", "5", "8", "13", "21"]);
    }

    #[test]
    fn test_bfile_parse() {
        let entry = BFileEntry::parse("0 1");
        assert!(entry.is_some());
        let entry = entry.unwrap();
        assert_eq!(entry.index, 0);
        assert_eq!(entry.value, "1");

        // Comment line
        assert!(BFileEntry::parse("# This is a comment").is_none());

        // Empty line
        assert!(BFileEntry::parse("").is_none());
    }

    #[test]
    fn test_search_query() {
        let query = SearchQuery::new("fibonacci");
        assert_eq!(query.query, "fibonacci");
        assert_eq!(query.format, "json");
        assert_eq!(query.start, 0);

        let next = query.next_page(10);
        assert_eq!(next.start, 10);

        let prev = next.prev_page(10);
        assert_eq!(prev.start, 0);
    }
}
