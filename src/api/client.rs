#![allow(dead_code)]

use super::models::{BFileEntry, OEISResponse, SearchQuery, Sequence};
use super::OEISSearchResponse;
use anyhow::{Context, Result};
use rand::{rngs::StdRng, Rng, SeedableRng};
use reqwest::Client;
use serde_json;
use std::time::Duration;

/// OEIS API client with async support
#[derive(Clone)]
pub struct OEISClient {
    client: Client,
    base_url: String,
}

impl OEISClient {
    /// Create a new OEIS API client
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.6050.0 Safari/537.36")
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            base_url: "https://oeis.org".to_string(),
        })
    }

    /// Extract error message from OEIS text format response
    fn extract_error_message(text: &str) -> anyhow::Error {
        // OEIS returns HTML even for fmt=txt, so we need to check for HTML content
        let text_lower = text.to_lowercase();

        if text_lower.contains("too many to show")
            || text_lower.contains("please refine your search")
        {
            anyhow::anyhow!("Too many results. Please narrow your search.")
        } else if text_lower.contains("no results")
            || text_lower.contains("sorry, but the terms do not match")
        {
            anyhow::anyhow!("No results found.")
        } else {
            anyhow::anyhow!("Unable to parse OEIS response")
        }
    }

    /// Search for sequences matching the query
    pub async fn search(&self, query: &SearchQuery, page_size: usize) -> Result<OEISResponse> {
        let url = query.to_url();

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to send request to OEIS API")?;

        if !response.status().is_success() {
            anyhow::bail!("OEIS API returned error: {}", response.status());
        }

        // Read response as text first to check for "null"
        let body = response
            .text()
            .await
            .context("Failed to read response from OEIS API")?;

        let body_trimmed = body.trim();

        // Check if OEIS returned null (too many/no results)
        if body_trimmed == "null" {
            // Request text format to get error message
            let text_query = query.clone().with_format("txt");
            let text_url = text_query.to_url();

            let text_response = self
                .client
                .get(&text_url)
                .send()
                .await
                .context("Failed to send text format request to OEIS API")?;

            if !text_response.status().is_success() {
                anyhow::bail!("OEIS API returned error: {}", text_response.status());
            }

            let text = text_response
                .text()
                .await
                .context("Failed to read text response from OEIS API")?;

            // Extract and return error message
            return Err(Self::extract_error_message(&text));
        }

        // Parse the JSON response
        let parsed: OEISSearchResponse =
            serde_json::from_str(&body).context("Failed to parse JSON response from OEIS API")?;

        let sequences: Vec<Sequence> = parsed.into_sequences();

        // Since the API doesn't provide total count, we estimate:
        // - If we got exactly page_size results, there might be more
        //   Set count to 100 to indicate "many results" for pagination UI
        // - If we got fewer than page_size, we've reached the end and can show exact count
        let count = if sequences.len() == page_size {
            // Set to 100 to indicate "many more results exist"
            // This allows pagination UI to show multiple pages
            100
        } else {
            sequences.len() as i32
        };

        Ok(OEISResponse {
            count,
            results: Some(sequences),
        })
    }

    /// Get a single sequence by its A-number (e.g., "A000055" or "55")
    pub async fn get_sequence(&self, a_number: &str) -> Result<Option<Sequence>> {
        let number = a_number
            .trim_start_matches('A')
            .trim_start_matches('a')
            .parse::<i32>()
            .context("Invalid A-number format")?;

        let query = SearchQuery::new(format!("id:A{:06}", number));
        let response = self.search(&query, 10).await?;

        // Return the first result if any
        Ok(response.results.and_then(|mut results| {
            if results.is_empty() {
                None
            } else {
                Some(results.remove(0))
            }
        }))
    }

    /// Fetch B-file data (extended sequence values)
    pub async fn fetch_b_file(&self, sequence_number: i32) -> Result<Vec<BFileEntry>> {
        let url = format!("{}/b{:06}.txt", self.base_url, sequence_number);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch B-file")?;

        if !response.status().is_success() {
            anyhow::bail!("B-file not found or unavailable");
        }

        let text = response
            .text()
            .await
            .context("Failed to read B-file content")?;

        let entries: Vec<BFileEntry> = text.lines().filter_map(BFileEntry::parse).collect();

        Ok(entries)
    }

    /// Fetch the recent sequences list
    pub async fn fetch_recent(&self) -> Result<String> {
        let url = format!("{}/recent.txt", self.base_url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch recent sequences")?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch recent.txt");
        }

        response
            .text()
            .await
            .context("Failed to read recent.txt content")
    }

    /// Get a random sequence
    /// This is done by searching for a random keyword and picking a random result
    pub async fn random_sequence(&self) -> Result<Option<Sequence>> {
        let mut rng = StdRng::from_entropy();

        // Generate a random sequence number between 1 and 370000 (approximate OEIS size)
        let random_num = rng.gen_range(1..370000);

        let query = SearchQuery::new(format!("id:A{:06}", random_num));
        let response = self.search(&query, 10).await?;

        Ok(response.results.and_then(|mut results| {
            if results.is_empty() {
                None
            } else {
                Some(results.remove(0))
            }
        }))
    }

    /// Fetch sequences by category (for webcam feature)
    pub async fn fetch_by_category(&self, category: SequenceCategory) -> Result<OEISResponse> {
        let query = match category {
            SequenceCategory::Best => SearchQuery::new("keyword:nice"),
            SequenceCategory::NeedingTerms => SearchQuery::new("keyword:more"),
            SequenceCategory::Recent => {
                // For recent, we could parse recent.txt, but for now just search new
                SearchQuery::new("keyword:new")
            }
            SequenceCategory::Unedited => {
                // Search for sequences with low revision count
                SearchQuery::new("keyword:new")
            }
        };

        self.search(&query, 10).await
    }

    /// Fetch the OEIS greeting message
    pub async fn fetch_greeting(&self) -> Result<String> {
        // OEIS doesn't have a dedicated API for greetings,
        // so we'll provide a static welcome message
        Ok(
            "Welcome to the On-Line Encyclopedia of Integer Sequences!\n\
            Founded in 1964 by N. J. A. Sloane.\n\
            Currently containing over 370,000 sequences.\n\
            https://oeis.org"
                .to_string(),
        )
    }
}

impl Default for OEISClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default OEIS client")
    }
}

/// Categories of sequences for webcam feature
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SequenceCategory {
    Best,
    NeedingTerms,
    Recent,
    Unedited,
}

impl SequenceCategory {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Best => "Best Sequences",
            Self::NeedingTerms => "Sequences Needing More Terms",
            Self::Recent => "Recent Additions",
            Self::Unedited => "Unedited Sequences",
        }
    }

    pub fn all() -> Vec<Self> {
        vec![Self::Best, Self::NeedingTerms, Self::Recent, Self::Unedited]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search_fibonacci() {
        let client = OEISClient::new().unwrap();
        let query = SearchQuery::new("id:A000045");

        let result = client.search(&query, 10).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.count > 0);
        assert!(response.results.is_some());
    }

    #[tokio::test]
    async fn test_get_sequence() {
        let client = OEISClient::new().unwrap();

        let result = client.get_sequence("A000045").await;
        assert!(result.is_ok());

        let sequence = result.unwrap();
        assert!(sequence.is_some());

        let seq = sequence.unwrap();
        assert_eq!(seq.number, 45);
        assert!(seq.name.contains("Fibonacci"));
    }

    #[test]
    fn test_category_display() {
        assert_eq!(SequenceCategory::Best.as_str(), "Best Sequences");
        assert_eq!(
            SequenceCategory::NeedingTerms.as_str(),
            "Sequences Needing More Terms"
        );
    }
}
