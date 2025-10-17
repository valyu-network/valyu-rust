//! Type definitions for Valyu API requests and responses

use serde::{Deserialize, Serialize};

/// Request parameters for the Valyu DeepSearch API
///
/// # Example
///
/// ```
/// use valyu::DeepSearchRequest;
///
/// let request = DeepSearchRequest::new("quantum computing")
///     .with_max_results(10)
///     .with_search_type("web")
///     .with_fast_mode(true);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct DeepSearchRequest {
    /// The search query text (required)
    pub query: String,

    /// Maximum number of results to return (1-20, default: 5)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_num_results: Option<u8>,

    /// Type of search: "all", "web", or "proprietary" (default: "all")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_type: Option<String>,

    /// Enable fast mode for reduced latency but shorter results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fast_mode: Option<bool>,

    /// Maximum cost per thousand retrievals in dollars (default: 20)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_price: Option<f64>,

    /// Relevance threshold (0.0-1.0, default: 0.5)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relevance_threshold: Option<f64>,

    /// Specific domains/URLs/datasets to search
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_sources: Option<Vec<String>>,

    /// Sources to exclude from search
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_sources: Option<Vec<String>>,

    /// Natural language guide phrase for categorization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    /// Response length: "short", "medium", "large", or "max"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_length: Option<String>,

    /// 2-letter ISO country code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,

    /// Whether this is a tool call (default: true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_tool_call: Option<bool>,

    /// Start date for filtering results (YYYY-MM-DD)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,

    /// End date for filtering results (YYYY-MM-DD)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}

impl DeepSearchRequest {
    /// Create a new DeepSearch request with just a query
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::DeepSearchRequest;
    ///
    /// let request = DeepSearchRequest::new("quantum computing");
    /// ```
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            max_num_results: None,
            search_type: None,
            fast_mode: None,
            max_price: None,
            relevance_threshold: None,
            included_sources: None,
            excluded_sources: None,
            category: None,
            response_length: None,
            country_code: None,
            is_tool_call: None,
            start_date: None,
            end_date: None,
        }
    }

    /// Set the maximum number of results (1-20)
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::DeepSearchRequest;
    ///
    /// let request = DeepSearchRequest::new("AI").with_max_results(10);
    /// ```
    pub fn with_max_results(mut self, max: u8) -> Self {
        self.max_num_results = Some(max);
        self
    }

    /// Set the search type ("all", "web", or "proprietary")
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::DeepSearchRequest;
    ///
    /// let request = DeepSearchRequest::new("AI").with_search_type("web");
    /// ```
    pub fn with_search_type(mut self, search_type: impl Into<String>) -> Self {
        self.search_type = Some(search_type.into());
        self
    }

    /// Enable or disable fast mode
    ///
    /// Fast mode provides reduced latency but may return shorter results.
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::DeepSearchRequest;
    ///
    /// let request = DeepSearchRequest::new("AI").with_fast_mode(true);
    /// ```
    pub fn with_fast_mode(mut self, enabled: bool) -> Self {
        self.fast_mode = Some(enabled);
        self
    }

    /// Set the maximum price per thousand retrievals in dollars
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::DeepSearchRequest;
    ///
    /// let request = DeepSearchRequest::new("AI").with_max_price(10.0);
    /// ```
    pub fn with_max_price(mut self, price: f64) -> Self {
        self.max_price = Some(price);
        self
    }

    /// Set the relevance threshold (0.0-1.0)
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::DeepSearchRequest;
    ///
    /// let request = DeepSearchRequest::new("AI").with_relevance_threshold(0.7);
    /// ```
    pub fn with_relevance_threshold(mut self, threshold: f64) -> Self {
        self.relevance_threshold = Some(threshold);
        self
    }

    /// Set the response length ("short", "medium", "large", or "max")
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::DeepSearchRequest;
    ///
    /// let request = DeepSearchRequest::new("AI").with_response_length("medium");
    /// ```
    pub fn with_response_length(mut self, length: impl Into<String>) -> Self {
        self.response_length = Some(length.into());
        self
    }

    /// Set specific sources to include in the search
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::DeepSearchRequest;
    ///
    /// let request = DeepSearchRequest::new("AI")
    ///     .with_included_sources(vec!["arxiv.org".to_string(), "nature.com".to_string()]);
    /// ```
    pub fn with_included_sources(mut self, sources: Vec<String>) -> Self {
        self.included_sources = Some(sources);
        self
    }

    /// Set specific sources to exclude from the search
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::DeepSearchRequest;
    ///
    /// let request = DeepSearchRequest::new("AI")
    ///     .with_excluded_sources(vec!["example.com".to_string()]);
    /// ```
    pub fn with_excluded_sources(mut self, sources: Vec<String>) -> Self {
        self.excluded_sources = Some(sources);
        self
    }

    /// Set a natural language category guide phrase
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::DeepSearchRequest;
    ///
    /// let request = DeepSearchRequest::new("AI")
    ///     .with_category("academic research");
    /// ```
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// Set the country code (2-letter ISO code)
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::DeepSearchRequest;
    ///
    /// let request = DeepSearchRequest::new("AI").with_country_code("US");
    /// ```
    pub fn with_country_code(mut self, code: impl Into<String>) -> Self {
        self.country_code = Some(code.into());
        self
    }

    /// Set whether this is a tool call
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::DeepSearchRequest;
    ///
    /// let request = DeepSearchRequest::new("AI").with_is_tool_call(false);
    /// ```
    pub fn with_is_tool_call(mut self, is_tool_call: bool) -> Self {
        self.is_tool_call = Some(is_tool_call);
        self
    }

    /// Set a date range for filtering results (YYYY-MM-DD format)
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::DeepSearchRequest;
    ///
    /// let request = DeepSearchRequest::new("AI")
    ///     .with_date_range("2024-01-01", "2024-12-31");
    /// ```
    pub fn with_date_range(mut self, start: impl Into<String>, end: impl Into<String>) -> Self {
        self.start_date = Some(start.into());
        self.end_date = Some(end.into());
        self
    }
}

/// Response from the Valyu DeepSearch API
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeepSearchResponse {
    /// Whether the request was successful
    pub success: bool,

    /// Error message if the request failed
    pub error: Option<String>,

    /// Transaction ID for tracking
    pub tx_id: Option<String>,

    /// The original query
    pub query: Option<String>,

    /// Array of search results
    pub results: Option<Vec<SearchResult>>,

    /// Breakdown of results by source type
    pub results_by_source: Option<ResultsBySource>,

    /// Total deduction per thousand retrievals
    pub total_deduction_pcm: Option<f64>,

    /// Total deduction in dollars
    pub total_deduction_dollars: Option<f64>,

    /// Total number of characters in results
    pub total_characters: Option<i32>,
}

/// Individual search result from the Valyu API
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchResult {
    /// Unique identifier for the result
    pub id: Option<String>,

    /// Title of the result
    pub title: Option<String>,

    /// URL of the source
    pub url: Option<String>,

    /// Content snippet or full text
    pub content: Option<String>,

    /// Description/summary of the result
    pub description: Option<String>,

    /// Source type (e.g., "web", "proprietary")
    pub source: Option<String>,

    /// Source type detail (e.g., "website")
    pub source_type: Option<String>,

    /// Data type ("structured" or "unstructured")
    pub data_type: Option<String>,

    /// Length of the content in characters
    pub length: Option<i32>,

    /// Price/cost of this result
    pub price: Option<f64>,

    /// URLs of associated images (can be object with numeric keys or simple map)
    pub image_url: Option<serde_json::Value>,

    /// Publication date (if available)
    pub publication_date: Option<String>,

    /// DOI (Digital Object Identifier) for academic papers
    pub doi: Option<String>,

    /// Citation information
    pub citation: Option<String>,

    /// Number of times this source has been cited
    pub citation_count: Option<i32>,

    /// List of authors
    pub authors: Option<Vec<String>>,

    /// Relevance score (0.0-1.0)
    pub relevance_score: Option<f64>,
}

/// Breakdown of results by source type
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResultsBySource {
    /// Number of web results
    pub web: Option<i32>,

    /// Number of proprietary/database results
    pub proprietary: Option<i32>,
}

// ========== Contents API Types ==========

/// Request parameters for the Valyu Contents API
///
/// Extract and process content from up to 10 URLs.
///
/// # Example
///
/// ```
/// use valyu::ContentsRequest;
///
/// let request = ContentsRequest::new(vec![
///     "https://example.com/article1".to_string(),
///     "https://example.com/article2".to_string(),
/// ])
/// .with_response_length("medium")
/// .with_extract_effort("high");
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct ContentsRequest {
    /// Array of 1-10 URLs to process (must use http/https protocol)
    pub urls: Vec<String>,

    /// Controls output size: "short" (25K), "medium" (50K), "large" (100K), "max", or custom integer (1K-1M)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_length: Option<ResponseLength>,

    /// Extraction quality: "normal" (fastest), "high" (better quality), or "auto" (automatic)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract_effort: Option<String>,

    /// Summary instructions: boolean, custom string, or JSON schema for structured extraction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<SummaryOption>,

    /// Maximum cost in dollars (defaults to 2x estimated cost)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_price_dollars: Option<f64>,
}

/// Response length configuration for Contents API
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseLength {
    /// Predefined size: "short", "medium", "large", or "max"
    Preset(String),
    /// Custom size in characters (1K-1M)
    Custom(i32),
}

/// Summary configuration for Contents API
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SummaryOption {
    /// Boolean flag for default summarization
    Boolean(bool),
    /// Custom summarization instructions
    Instructions(String),
    /// JSON schema for structured extraction
    Schema(serde_json::Value),
}

impl ContentsRequest {
    /// Create a new Contents request with URLs
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::ContentsRequest;
    ///
    /// let request = ContentsRequest::new(vec![
    ///     "https://example.com/article".to_string(),
    /// ]);
    /// ```
    pub fn new(urls: Vec<String>) -> Self {
        Self {
            urls,
            response_length: None,
            extract_effort: None,
            summary: None,
            max_price_dollars: None,
        }
    }

    /// Set the response length
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::ContentsRequest;
    ///
    /// let request = ContentsRequest::new(vec!["https://example.com".to_string()])
    ///     .with_response_length("medium");
    /// ```
    pub fn with_response_length(mut self, length: impl Into<String>) -> Self {
        self.response_length = Some(ResponseLength::Preset(length.into()));
        self
    }

    /// Set a custom response length in characters
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::ContentsRequest;
    ///
    /// let request = ContentsRequest::new(vec!["https://example.com".to_string()])
    ///     .with_custom_response_length(30000);
    /// ```
    pub fn with_custom_response_length(mut self, chars: i32) -> Self {
        self.response_length = Some(ResponseLength::Custom(chars));
        self
    }

    /// Set the extraction effort level
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::ContentsRequest;
    ///
    /// let request = ContentsRequest::new(vec!["https://example.com".to_string()])
    ///     .with_extract_effort("high");
    /// ```
    pub fn with_extract_effort(mut self, effort: impl Into<String>) -> Self {
        self.extract_effort = Some(effort.into());
        self
    }

    /// Enable or disable default summarization
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::ContentsRequest;
    ///
    /// let request = ContentsRequest::new(vec!["https://example.com".to_string()])
    ///     .with_summary(true);
    /// ```
    pub fn with_summary(mut self, enabled: bool) -> Self {
        self.summary = Some(SummaryOption::Boolean(enabled));
        self
    }

    /// Set custom summarization instructions
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::ContentsRequest;
    ///
    /// let request = ContentsRequest::new(vec!["https://example.com".to_string()])
    ///     .with_summary_instructions("Extract key findings and methodology");
    /// ```
    pub fn with_summary_instructions(mut self, instructions: impl Into<String>) -> Self {
        self.summary = Some(SummaryOption::Instructions(instructions.into()));
        self
    }

    /// Set a JSON schema for structured extraction
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::ContentsRequest;
    /// use serde_json::json;
    ///
    /// let schema = json!({
    ///     "type": "object",
    ///     "properties": {
    ///         "title": {"type": "string"},
    ///         "summary": {"type": "string"}
    ///     }
    /// });
    ///
    /// let request = ContentsRequest::new(vec!["https://example.com".to_string()])
    ///     .with_summary_schema(schema);
    /// ```
    pub fn with_summary_schema(mut self, schema: serde_json::Value) -> Self {
        self.summary = Some(SummaryOption::Schema(schema));
        self
    }

    /// Set the maximum price in dollars
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::ContentsRequest;
    ///
    /// let request = ContentsRequest::new(vec!["https://example.com".to_string()])
    ///     .with_max_price_dollars(5.0);
    /// ```
    pub fn with_max_price_dollars(mut self, max_price: f64) -> Self {
        self.max_price_dollars = Some(max_price);
        self
    }
}

/// Response from the Valyu Contents API
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContentsResponse {
    /// Whether the request was successful
    pub success: bool,

    /// Error message if the request failed
    pub error: Option<String>,

    /// Transaction ID for tracking
    pub tx_id: Option<String>,

    /// Array of processed content results
    pub results: Option<Vec<ContentResult>>,

    /// Number of URLs requested
    pub urls_requested: Option<i32>,

    /// Number of URLs successfully processed
    pub urls_processed: Option<i32>,

    /// Number of URLs that failed processing
    pub urls_failed: Option<i32>,

    /// Total cost in dollars
    pub total_cost_dollars: Option<f64>,

    /// Total number of characters in results
    pub total_characters: Option<i32>,
}

/// Individual content result from the Contents API
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContentResult {
    /// Title of the content
    pub title: Option<String>,

    /// Source URL
    pub url: Option<String>,

    /// Extracted content (markdown or structured JSON)
    pub content: Option<serde_json::Value>,

    /// Description/summary
    pub description: Option<String>,

    /// Publication date
    pub publication_date: Option<String>,

    /// Extracted images
    pub images: Option<Vec<String>>,

    /// Cost for this URL
    pub cost_dollars: Option<f64>,

    /// Number of characters
    pub characters: Option<i32>,
}

// ========== Answer API Types ==========

/// Request parameters for the Valyu Answer API
///
/// Get AI-powered answers with automatic source retrieval.
///
/// # Example
///
/// ```
/// use valyu::AnswerRequest;
///
/// let request = AnswerRequest::new("What are the latest developments in quantum computing?")
///     .with_search_type("web")
///     .with_system_instructions("Focus on breakthroughs from 2024");
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct AnswerRequest {
    /// The search query to process (required)
    pub query: String,

    /// Custom AI processing directives (max 2000 chars)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_instructions: Option<String>,

    /// JSON schema for structured responses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_output: Option<serde_json::Value>,

    /// Type of search: "all", "web", or "proprietary" (default: "all")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_type: Option<String>,

    /// Enable fast mode for reduced latency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fast_mode: Option<bool>,

    /// Maximum CPM for search data (default: $30)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_max_price: Option<f64>,

    /// Specific domains/URLs/datasets to search
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_sources: Option<Vec<String>>,

    /// Sources to exclude from search
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_sources: Option<Vec<String>>,

    /// Start date for filtering results (YYYY-MM-DD)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,

    /// End date for filtering results (YYYY-MM-DD)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,

    /// 2-letter ISO country code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
}

impl AnswerRequest {
    /// Create a new Answer request with a query
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::AnswerRequest;
    ///
    /// let request = AnswerRequest::new("What is quantum computing?");
    /// ```
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            system_instructions: None,
            structured_output: None,
            search_type: None,
            fast_mode: None,
            data_max_price: None,
            included_sources: None,
            excluded_sources: None,
            start_date: None,
            end_date: None,
            country_code: None,
        }
    }

    /// Set custom system instructions for the AI
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::AnswerRequest;
    ///
    /// let request = AnswerRequest::new("quantum computing")
    ///     .with_system_instructions("Focus on practical applications");
    /// ```
    pub fn with_system_instructions(mut self, instructions: impl Into<String>) -> Self {
        self.system_instructions = Some(instructions.into());
        self
    }

    /// Set a JSON schema for structured output
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::AnswerRequest;
    /// use serde_json::json;
    ///
    /// let schema = json!({
    ///     "type": "object",
    ///     "properties": {
    ///         "summary": {"type": "string"},
    ///         "key_points": {"type": "array"}
    ///     }
    /// });
    ///
    /// let request = AnswerRequest::new("quantum computing")
    ///     .with_structured_output(schema);
    /// ```
    pub fn with_structured_output(mut self, schema: serde_json::Value) -> Self {
        self.structured_output = Some(schema);
        self
    }

    /// Set the search type
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::AnswerRequest;
    ///
    /// let request = AnswerRequest::new("quantum computing")
    ///     .with_search_type("web");
    /// ```
    pub fn with_search_type(mut self, search_type: impl Into<String>) -> Self {
        self.search_type = Some(search_type.into());
        self
    }

    /// Enable fast mode
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::AnswerRequest;
    ///
    /// let request = AnswerRequest::new("quantum computing")
    ///     .with_fast_mode(true);
    /// ```
    pub fn with_fast_mode(mut self, enabled: bool) -> Self {
        self.fast_mode = Some(enabled);
        self
    }

    /// Set the maximum data price
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::AnswerRequest;
    ///
    /// let request = AnswerRequest::new("quantum computing")
    ///     .with_data_max_price(20.0);
    /// ```
    pub fn with_data_max_price(mut self, price: f64) -> Self {
        self.data_max_price = Some(price);
        self
    }

    /// Set included sources
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::AnswerRequest;
    ///
    /// let request = AnswerRequest::new("quantum computing")
    ///     .with_included_sources(vec!["arxiv.org".to_string()]);
    /// ```
    pub fn with_included_sources(mut self, sources: Vec<String>) -> Self {
        self.included_sources = Some(sources);
        self
    }

    /// Set excluded sources
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::AnswerRequest;
    ///
    /// let request = AnswerRequest::new("quantum computing")
    ///     .with_excluded_sources(vec!["example.com".to_string()]);
    /// ```
    pub fn with_excluded_sources(mut self, sources: Vec<String>) -> Self {
        self.excluded_sources = Some(sources);
        self
    }

    /// Set date range for filtering
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::AnswerRequest;
    ///
    /// let request = AnswerRequest::new("quantum computing")
    ///     .with_date_range("2024-01-01", "2024-12-31");
    /// ```
    pub fn with_date_range(mut self, start: impl Into<String>, end: impl Into<String>) -> Self {
        self.start_date = Some(start.into());
        self.end_date = Some(end.into());
        self
    }

    /// Set country code
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::AnswerRequest;
    ///
    /// let request = AnswerRequest::new("quantum computing")
    ///     .with_country_code("US");
    /// ```
    pub fn with_country_code(mut self, code: impl Into<String>) -> Self {
        self.country_code = Some(code.into());
        self
    }
}

/// Response from the Valyu Answer API
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnswerResponse {
    /// Whether the request was successful
    pub success: bool,

    /// Error message if the request failed
    pub error: Option<String>,

    /// AI transaction ID
    pub ai_tx_id: Option<String>,

    /// The original query
    pub original_query: Option<String>,

    /// AI-generated response (string or structured object)
    pub contents: Option<serde_json::Value>,

    /// Data type: "unstructured" or "structured"
    pub data_type: Option<String>,

    /// Sources used for the answer
    pub search_results: Option<Vec<AnswerSearchResult>>,

    /// Search metadata
    pub search_metadata: Option<AnswerSearchMetadata>,

    /// AI usage statistics
    pub ai_usage: Option<AiUsage>,

    /// Cost breakdown
    pub cost: Option<AnswerCost>,
}

/// Search result included in Answer response
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnswerSearchResult {
    /// Result title
    pub title: Option<String>,

    /// Source URL
    pub url: Option<String>,

    /// Content snippet
    pub snippet: Option<String>,

    /// Publication date
    pub date: Option<String>,

    /// Content length
    pub length: Option<i32>,
}

/// Search metadata for Answer API
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnswerSearchMetadata {
    /// Search transaction ID
    pub search_tx_id: Option<String>,

    /// Number of results found
    pub result_count: Option<i32>,

    /// Total characters retrieved
    pub total_characters: Option<i32>,
}

/// AI usage statistics
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AiUsage {
    /// Input tokens used
    pub input_tokens: Option<i32>,

    /// Output tokens generated
    pub output_tokens: Option<i32>,
}

/// Cost breakdown for Answer API
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnswerCost {
    /// Total cost in dollars
    pub total_dollars: Option<f64>,

    /// Search cost in dollars
    pub search_dollars: Option<f64>,

    /// AI processing cost in dollars
    pub ai_dollars: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_builder() {
        let request = DeepSearchRequest::new("quantum computing")
            .with_max_results(10)
            .with_search_type("web")
            .with_fast_mode(true);

        assert_eq!(request.query, "quantum computing");
        assert_eq!(request.max_num_results, Some(10));
        assert_eq!(request.search_type, Some("web".to_string()));
        assert_eq!(request.fast_mode, Some(true));
    }

    #[test]
    fn test_request_serialization() {
        let request = DeepSearchRequest::new("test query")
            .with_max_results(5);

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("test query"));
        assert!(json.contains("max_num_results"));
    }
}
