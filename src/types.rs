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

// ========== DeepResearch API Types ==========

/// Research mode for DeepResearch API
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DeepResearchMode {
    /// Fast mode - quick lookups, simple questions (1-2 min)
    Fast,
    /// Standard mode - moderate research depth (5-10 min)
    Standard,
    /// Lite mode - moderate research depth (5-10 min) [Deprecated: use Standard instead]
    Lite,
    /// Heavy mode - comprehensive analysis (15-90 min)
    Heavy,
}

impl Default for DeepResearchMode {
    fn default() -> Self {
        DeepResearchMode::Standard
    }
}

/// Task status for DeepResearch
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DeepResearchStatus {
    /// Task is waiting to start
    Queued,
    /// Task is actively researching
    Running,
    /// Task completed successfully
    Completed,
    /// Task failed
    Failed,
    /// Task was cancelled
    Cancelled,
}

/// File attachment for DeepResearch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepResearchFileAttachment {
    /// Data URL (base64 encoded, e.g., "data:application/pdf;base64,...")
    pub data: String,
    /// Original filename
    pub filename: String,
    /// MIME type (e.g., "application/pdf", "image/png")
    #[serde(rename = "mediaType")]
    pub media_type: String,
    /// Optional context about this file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
}

/// Deliverable file type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DeliverableType {
    /// CSV file
    Csv,
    /// Excel spreadsheet (XLSX)
    Xlsx,
    /// PowerPoint presentation (PPTX)
    Pptx,
    /// Word document (DOCX)
    Docx,
    /// PDF document
    Pdf,
}

/// Deliverable configuration for DeepResearch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deliverable {
    /// File type to generate
    #[serde(rename = "type")]
    pub deliverable_type: DeliverableType,
    /// Description of what data to extract or content to generate (max 500 characters)
    pub description: String,
    /// Suggested column names (for CSV/XLSX only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<String>>,
    /// Include column headers (for CSV/XLSX, default: true)
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeHeaders")]
    pub include_headers: Option<bool>,
    /// Sheet name (for XLSX only)
    #[serde(skip_serializing_if = "Option::is_none", rename = "sheetName")]
    pub sheet_name: Option<String>,
    /// Number of slides to generate (for PPTX only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slides: Option<i32>,
    /// Template name to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}

/// Deliverable generation status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DeliverableStatus {
    /// Successfully generated
    Completed,
    /// Generation failed
    Failed,
}

/// Result of deliverable generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliverableResult {
    /// Unique deliverable ID
    pub id: String,
    /// Original request description
    pub request: String,
    /// Deliverable file type
    #[serde(rename = "type")]
    pub deliverable_type: String,
    /// Generation status
    pub status: DeliverableStatus,
    /// Generated filename/title
    pub title: String,
    /// Deliverable content description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Token-signed authenticated URL to download the file
    pub url: String,
    /// S3 storage key
    pub s3_key: String,
    /// Number of rows (for CSV/XLSX)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_count: Option<i32>,
    /// Number of columns (for CSV/XLSX)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_count: Option<i32>,
    /// Error message if status is failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Unix timestamp of creation
    pub created_at: i64,
}

/// MCP server configuration for DeepResearch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepResearchMCPServerConfig {
    /// MCP server URL
    pub url: String,
    /// Server name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Custom tool prefix
    #[serde(skip_serializing_if = "Option::is_none", rename = "toolPrefix")]
    pub tool_prefix: Option<String>,
    /// Authentication configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<serde_json::Value>,
    /// Allowed tools
    #[serde(skip_serializing_if = "Option::is_none", rename = "allowedTools")]
    pub allowed_tools: Option<Vec<String>>,
}

/// Search configuration for DeepResearch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepResearchSearchConfig {
    /// Type of search: "all", "web", or "proprietary"
    #[serde(skip_serializing_if = "Option::is_none", rename = "searchType")]
    pub search_type: Option<String>,
    /// Sources to include in search
    #[serde(skip_serializing_if = "Option::is_none", rename = "includedSources")]
    pub included_sources: Option<Vec<String>>,
}

/// Request parameters for creating a DeepResearch task
///
/// # Example
///
/// ```
/// use valyu::{DeepResearchCreateRequest, DeepResearchMode};
///
/// let request = DeepResearchCreateRequest::new("What are the key differences between RAG and fine-tuning?")
///     .with_mode(DeepResearchMode::Standard)
///     .with_output_formats(vec!["markdown".to_string()]);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct DeepResearchCreateRequest {
    /// Research query or task description (required)
    pub input: String,

    /// Research mode: "fast", "lite", or "heavy"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<DeepResearchMode>,

    /// Output formats: ["markdown"], ["markdown", "pdf"], or JSON schema
    #[serde(skip_serializing_if = "Option::is_none", rename = "outputFormats")]
    pub output_formats: Option<Vec<serde_json::Value>>,

    /// Natural language strategy instructions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,

    /// Search configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<DeepResearchSearchConfig>,

    /// URLs to extract content from (max 10)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,

    /// File attachments (max 10)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<DeepResearchFileAttachment>>,

    /// Additional file outputs to generate (CSV, Excel, PowerPoint, Word, PDF). Max 10.
    /// Can be simple strings or Deliverable objects with detailed configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliverables: Option<Vec<serde_json::Value>>,

    /// MCP server configurations (max 5)
    #[serde(skip_serializing_if = "Option::is_none", rename = "mcpServers")]
    pub mcp_servers: Option<Vec<DeepResearchMCPServerConfig>>,

    /// Enable/disable code execution (default: true)
    #[serde(skip_serializing_if = "Option::is_none", rename = "codeExecution")]
    pub code_execution: Option<bool>,

    /// Previous DeepResearch IDs to use as context (max 3)
    #[serde(skip_serializing_if = "Option::is_none", rename = "previousReports")]
    pub previous_reports: Option<Vec<String>>,

    /// HTTPS URL for completion notification
    #[serde(skip_serializing_if = "Option::is_none", rename = "webhookUrl")]
    pub webhook_url: Option<String>,

    /// Custom metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl DeepResearchCreateRequest {
    /// Create a new DeepResearch request with a query
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::DeepResearchCreateRequest;
    ///
    /// let request = DeepResearchCreateRequest::new("Research market trends in AI");
    /// ```
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            input: input.into(),
            model: None,
            output_formats: None,
            strategy: None,
            search: None,
            urls: None,
            files: None,
            deliverables: None,
            mcp_servers: None,
            code_execution: None,
            previous_reports: None,
            webhook_url: None,
            metadata: None,
        }
    }

    /// Set the research mode
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::{DeepResearchCreateRequest, DeepResearchMode};
    ///
    /// let request = DeepResearchCreateRequest::new("AI research")
    ///     .with_mode(DeepResearchMode::Heavy);
    /// ```
    pub fn with_mode(mut self, mode: DeepResearchMode) -> Self {
        self.model = Some(mode);
        self
    }

    /// Set output formats (e.g., ["markdown"], ["markdown", "pdf"])
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::DeepResearchCreateRequest;
    ///
    /// let request = DeepResearchCreateRequest::new("AI research")
    ///     .with_output_formats(vec!["markdown".to_string(), "pdf".to_string()]);
    /// ```
    pub fn with_output_formats(mut self, formats: Vec<String>) -> Self {
        self.output_formats = Some(formats.into_iter().map(serde_json::Value::String).collect());
        self
    }

    /// Set a JSON schema for structured output
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::DeepResearchCreateRequest;
    /// use serde_json::json;
    ///
    /// let schema = json!({
    ///     "type": "object",
    ///     "properties": {
    ///         "summary": {"type": "string"},
    ///         "key_points": {"type": "array", "items": {"type": "string"}}
    ///     }
    /// });
    ///
    /// let request = DeepResearchCreateRequest::new("AI research")
    ///     .with_structured_output(schema);
    /// ```
    pub fn with_structured_output(mut self, schema: serde_json::Value) -> Self {
        self.output_formats = Some(vec![schema]);
        self
    }

    /// Set natural language strategy instructions
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::DeepResearchCreateRequest;
    ///
    /// let request = DeepResearchCreateRequest::new("AI research")
    ///     .with_strategy("Focus on peer-reviewed sources and recent publications");
    /// ```
    pub fn with_strategy(mut self, strategy: impl Into<String>) -> Self {
        self.strategy = Some(strategy.into());
        self
    }

    /// Set search configuration
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::{DeepResearchCreateRequest, DeepResearchSearchConfig};
    ///
    /// let request = DeepResearchCreateRequest::new("AI research")
    ///     .with_search(DeepResearchSearchConfig {
    ///         search_type: Some("web".to_string()),
    ///         included_sources: Some(vec!["arxiv.org".to_string()]),
    ///     });
    /// ```
    pub fn with_search(mut self, search: DeepResearchSearchConfig) -> Self {
        self.search = Some(search);
        self
    }

    /// Set URLs to extract content from
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::DeepResearchCreateRequest;
    ///
    /// let request = DeepResearchCreateRequest::new("Compare these articles")
    ///     .with_urls(vec!["https://example.com/article1".to_string()]);
    /// ```
    pub fn with_urls(mut self, urls: Vec<String>) -> Self {
        self.urls = Some(urls);
        self
    }

    /// Set file attachments
    pub fn with_files(mut self, files: Vec<DeepResearchFileAttachment>) -> Self {
        self.files = Some(files);
        self
    }

    /// Set deliverables (additional file outputs to generate)
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::{DeepResearchCreateRequest, Deliverable, DeliverableType};
    /// use serde_json::json;
    ///
    /// let request = DeepResearchCreateRequest::new("Market research")
    ///     .with_deliverables(vec![
    ///         json!("Excel file with company data"),
    ///         json!(Deliverable {
    ///             deliverable_type: DeliverableType::Pptx,
    ///             description: "10-slide presentation summarizing findings".to_string(),
    ///             columns: None,
    ///             include_headers: None,
    ///             sheet_name: None,
    ///             slides: Some(10),
    ///             template: None,
    ///         })
    ///     ]);
    /// ```
    pub fn with_deliverables(mut self, deliverables: Vec<serde_json::Value>) -> Self {
        self.deliverables = Some(deliverables);
        self
    }

    /// Set MCP server configurations
    pub fn with_mcp_servers(mut self, servers: Vec<DeepResearchMCPServerConfig>) -> Self {
        self.mcp_servers = Some(servers);
        self
    }

    /// Enable or disable code execution
    pub fn with_code_execution(mut self, enabled: bool) -> Self {
        self.code_execution = Some(enabled);
        self
    }

    /// Set previous report IDs for context
    pub fn with_previous_reports(mut self, report_ids: Vec<String>) -> Self {
        self.previous_reports = Some(report_ids);
        self
    }

    /// Set webhook URL for completion notification
    pub fn with_webhook_url(mut self, url: impl Into<String>) -> Self {
        self.webhook_url = Some(url.into());
        self
    }

    /// Set custom metadata
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// Response from creating a DeepResearch task
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeepResearchCreateResponse {
    /// Whether the request was successful
    pub success: bool,

    /// Unique task identifier
    pub deepresearch_id: Option<String>,

    /// Current task status
    pub status: Option<DeepResearchStatus>,

    /// Research mode used
    pub model: Option<DeepResearchMode>,

    /// Task creation timestamp
    pub created_at: Option<String>,

    /// Custom metadata
    pub metadata: Option<serde_json::Value>,

    /// Whether the task is publicly accessible
    pub public: Option<bool>,

    /// Webhook secret for signature verification (only returned once)
    pub webhook_secret: Option<String>,

    /// Additional status message
    pub message: Option<String>,

    /// Error message if failed
    pub error: Option<String>,
}

/// Progress information for a running task
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeepResearchProgress {
    /// Current step number
    pub current_step: i32,
    /// Total number of steps
    pub total_steps: i32,
}

/// Source information from research
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeepResearchSource {
    /// Source title
    pub title: String,
    /// Source URL
    pub url: String,
    /// Content snippet
    pub snippet: Option<String>,
    /// Description
    pub description: Option<String>,
    /// Source type (web, pubmed, etc.)
    pub source: Option<String>,
    /// Organization ID
    pub org_id: Option<String>,
    /// Price/cost
    pub price: Option<f64>,
    /// Document ID
    pub id: Option<String>,
    /// DOI for academic papers
    pub doi: Option<String>,
    /// Document category
    pub category: Option<String>,
    /// Word count
    pub word_count: Option<i32>,
}

/// Image metadata from research
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeepResearchImage {
    /// Unique image identifier
    pub image_id: String,
    /// Image type: "chart" or "ai_generated"
    pub image_type: String,
    /// Associated DeepResearch ID
    pub deepresearch_id: String,
    /// Image title
    pub title: String,
    /// Image description
    pub description: Option<String>,
    /// Image URL
    pub image_url: String,
    /// S3 key
    pub s3_key: String,
    /// Creation timestamp
    pub created_at: i64,
    /// Chart type (if applicable)
    pub chart_type: Option<String>,
}

/// Usage and cost breakdown
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeepResearchUsage {
    /// Search cost in dollars
    pub search_cost: f64,
    /// Contents extraction cost in dollars
    pub contents_cost: f64,
    /// AI processing cost in dollars
    pub ai_cost: f64,
    /// Compute cost in dollars
    pub compute_cost: f64,
    /// Total cost in dollars
    pub total_cost: f64,
}

/// Response from getting task status
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeepResearchStatusResponse {
    /// Whether the request was successful
    pub success: bool,

    /// Unique task identifier
    pub deepresearch_id: Option<String>,

    /// Current task status
    pub status: Option<DeepResearchStatus>,

    /// Original research query
    pub query: Option<String>,

    /// Research mode used
    pub mode: Option<DeepResearchMode>,

    /// Requested output formats
    pub output_formats: Option<Vec<serde_json::Value>>,

    /// Task creation timestamp (Unix)
    pub created_at: Option<i64>,

    /// Whether the task is publicly accessible
    pub public: Option<bool>,

    /// Progress information (when running)
    pub progress: Option<DeepResearchProgress>,

    /// Conversation messages
    pub messages: Option<Vec<serde_json::Value>>,

    /// Completion timestamp (Unix)
    pub completed_at: Option<i64>,

    /// Research output (markdown string or JSON object)
    pub output: Option<serde_json::Value>,

    /// Output type: "markdown" or "json"
    pub output_type: Option<String>,

    /// PDF download URL (if requested)
    pub pdf_url: Option<String>,

    /// Generated images
    pub images: Option<Vec<DeepResearchImage>>,

    /// Generated deliverable files
    pub deliverables: Option<Vec<DeliverableResult>>,

    /// Sources used in research
    pub sources: Option<Vec<DeepResearchSource>>,

    /// Usage and cost breakdown
    pub usage: Option<DeepResearchUsage>,

    /// Error message if failed
    pub error: Option<String>,
}

/// Response from listing tasks
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeepResearchListResponse {
    /// Whether the request was successful
    pub success: bool,

    /// List of tasks
    pub data: Option<Vec<DeepResearchTaskListItem>>,

    /// Error message if failed
    pub error: Option<String>,
}

/// Minimal task info for list view
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeepResearchTaskListItem {
    /// Unique task identifier
    pub deepresearch_id: String,
    /// Research query
    pub query: String,
    /// Current status
    pub status: DeepResearchStatus,
    /// Creation timestamp (Unix)
    pub created_at: i64,
    /// Whether publicly accessible
    pub public: Option<bool>,
}

/// Response from update/cancel/delete operations
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeepResearchOperationResponse {
    /// Whether the request was successful
    pub success: bool,

    /// Status message
    pub message: Option<String>,

    /// Task identifier
    pub deepresearch_id: Option<String>,

    /// Error message if failed
    pub error: Option<String>,
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
