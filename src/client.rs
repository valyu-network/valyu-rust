//! Client for interacting with the Valyu API

use crate::error::{Result, ValyuError};
use crate::types::{
    AnswerRequest, AnswerResponse, ContentsRequest, ContentsResponse, DeepSearchRequest,
    DeepSearchResponse,
    // DeepResearch API
    DeepResearchCreateRequest, DeepResearchCreateResponse, DeepResearchListResponse,
    DeepResearchOperationResponse, DeepResearchStatus, DeepResearchStatusResponse,
};

/// Base URL for the Valyu API
const API_BASE_URL: &str = "https://api.valyu.ai/v1";

/// Client for interacting with the Valyu API
///
/// # Example
///
/// ```no_run
/// use valyu::ValyuClient;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = ValyuClient::new("your-api-key");
///     let response = client.search("quantum computing").await?;
///     println!("Found {} results", response.results.as_ref().map(|r| r.len()).unwrap_or(0));
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct ValyuClient {
    api_key: String,
    client: reqwest::Client,
    base_url: String,
}

impl ValyuClient {
    /// Create a new Valyu client with an API key
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::ValyuClient;
    ///
    /// let client = ValyuClient::new("your-api-key");
    /// ```
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            client: reqwest::Client::new(),
            base_url: API_BASE_URL.to_string(),
        }
    }

    /// Create a new Valyu client with a custom base URL
    ///
    /// This is useful for testing or using alternative endpoints.
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::ValyuClient;
    ///
    /// let client = ValyuClient::with_base_url("your-api-key", "https://custom.api.url/v1");
    /// ```
    pub fn with_base_url(api_key: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            client: reqwest::Client::new(),
            base_url: base_url.into(),
        }
    }

    /// Create a new Valyu client with a custom reqwest client
    ///
    /// This allows you to configure custom timeout, proxy settings, etc.
    ///
    /// # Example
    ///
    /// ```
    /// use valyu::ValyuClient;
    /// use std::time::Duration;
    ///
    /// let http_client = reqwest::Client::builder()
    ///     .timeout(Duration::from_secs(30))
    ///     .build()
    ///     .unwrap();
    ///
    /// let client = ValyuClient::with_client("your-api-key", http_client);
    /// ```
    pub fn with_client(api_key: impl Into<String>, client: reqwest::Client) -> Self {
        Self {
            api_key: api_key.into(),
            client,
            base_url: API_BASE_URL.to_string(),
        }
    }

    /// Perform a deep search query with full control over request parameters
    ///
    /// # Example
    ///
    /// ```no_run
    /// use valyu::{ValyuClient, DeepSearchRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = ValyuClient::new("your-api-key");
    ///
    ///     let request = DeepSearchRequest::new("artificial intelligence")
    ///         .with_max_results(10)
    ///         .with_search_type("web")
    ///         .with_fast_mode(true);
    ///
    ///     let response = client.deep_search(&request).await?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The HTTP request fails
    /// - The API returns an error response
    /// - The response cannot be parsed
    pub async fn deep_search(&self, request: &DeepSearchRequest) -> Result<DeepSearchResponse> {
        let url = format!("{}/deepsearch", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("x-api-key", &self.api_key)
            .json(request)
            .send()
            .await?;

        let status = response.status();

        // Handle specific HTTP status codes
        match status.as_u16() {
            200 | 206 => {
                // Success - continue to parse response
            }
            401 | 403 => {
                return Err(ValyuError::InvalidApiKey);
            }
            429 => {
                return Err(ValyuError::RateLimitExceeded);
            }
            503 => {
                return Err(ValyuError::ServiceUnavailable);
            }
            _ => {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                return Err(ValyuError::ApiError(format!(
                    "HTTP {}: {}",
                    status, error_text
                )));
            }
        }

        let search_response: DeepSearchResponse = response.json().await.map_err(|e| {
            ValyuError::ParseError(format!("Failed to parse API response: {}", e))
        })?;

        // Check if the API returned an error in the response body
        if !search_response.success {
            if let Some(error) = &search_response.error {
                return Err(ValyuError::ApiError(error.clone()));
            }
            return Err(ValyuError::ApiError(
                "API request was not successful".to_string(),
            ));
        }

        Ok(search_response)
    }

    /// Convenience method to perform a simple query with default settings
    ///
    /// This is equivalent to creating a `DeepSearchRequest` with just a query
    /// and calling `deep_search`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use valyu::ValyuClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = ValyuClient::new("your-api-key");
    ///     let response = client.search("quantum computing").await?;
    ///
    ///     if let Some(results) = &response.results {
    ///         for result in results {
    ///             println!("{}", result.title.as_deref().unwrap_or("Untitled"));
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The HTTP request fails
    /// - The API returns an error response
    /// - The response cannot be parsed
    pub async fn search(&self, query: impl Into<String>) -> Result<DeepSearchResponse> {
        let request = DeepSearchRequest::new(query);
        self.deep_search(&request).await
    }

    /// Extract and process content from URLs
    ///
    /// Process up to 10 URLs and extract their content in a clean, structured format.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use valyu::{ValyuClient, ContentsRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = ValyuClient::new("your-api-key");
    ///
    ///     let request = ContentsRequest::new(vec![
    ///         "https://example.com/article1".to_string(),
    ///         "https://example.com/article2".to_string(),
    ///     ])
    ///     .with_response_length("medium")
    ///     .with_extract_effort("high");
    ///
    ///     let response = client.contents(&request).await?;
    ///
    ///     if let Some(results) = &response.results {
    ///         for result in results {
    ///             println!("Title: {}", result.title.as_deref().unwrap_or("Untitled"));
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The HTTP request fails
    /// - The API returns an error response
    /// - The response cannot be parsed
    /// - More than 10 URLs are provided
    pub async fn contents(&self, request: &ContentsRequest) -> Result<ContentsResponse> {
        let url = format!("{}/contents", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("x-api-key", &self.api_key)
            .json(request)
            .send()
            .await?;

        let status = response.status();

        // Handle specific HTTP status codes
        match status.as_u16() {
            200 | 206 => {
                // Success - continue to parse response
            }
            400 => {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Invalid request parameters".to_string());
                return Err(ValyuError::InvalidRequest(error_text));
            }
            401 | 403 => {
                return Err(ValyuError::InvalidApiKey);
            }
            402 => {
                return Err(ValyuError::ApiError("Insufficient credits".to_string()));
            }
            422 => {
                return Err(ValyuError::ApiError("All URLs failed processing".to_string()));
            }
            429 => {
                return Err(ValyuError::RateLimitExceeded);
            }
            503 => {
                return Err(ValyuError::ServiceUnavailable);
            }
            _ => {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                return Err(ValyuError::ApiError(format!(
                    "HTTP {}: {}",
                    status, error_text
                )));
            }
        }

        let contents_response: ContentsResponse = response.json().await.map_err(|e| {
            ValyuError::ParseError(format!("Failed to parse API response: {}", e))
        })?;

        // Check if the API returned an error in the response body
        if !contents_response.success {
            if let Some(error) = &contents_response.error {
                return Err(ValyuError::ApiError(error.clone()));
            }
            return Err(ValyuError::ApiError(
                "API request was not successful".to_string(),
            ));
        }

        Ok(contents_response)
    }

    /// Get AI-powered answers with automatic source retrieval
    ///
    /// Ask questions and get comprehensive answers backed by relevant sources.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use valyu::{ValyuClient, AnswerRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = ValyuClient::new("your-api-key");
    ///
    ///     let request = AnswerRequest::new("What are the latest developments in quantum computing?")
    ///         .with_search_type("web")
    ///         .with_system_instructions("Focus on breakthroughs from 2024");
    ///
    ///     let response = client.answer(&request).await?;
    ///
    ///     if let Some(contents) = &response.contents {
    ///         println!("Answer: {}", contents);
    ///     }
    ///
    ///     if let Some(sources) = &response.search_results {
    ///         println!("\nSources ({}):", sources.len());
    ///         for source in sources {
    ///             println!("  - {}", source.title.as_deref().unwrap_or("Untitled"));
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The HTTP request fails
    /// - The API returns an error response
    /// - The response cannot be parsed
    pub async fn answer(&self, request: &AnswerRequest) -> Result<AnswerResponse> {
        let url = format!("{}/answer", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("x-api-key", &self.api_key)
            .json(request)
            .send()
            .await?;

        let status = response.status();

        // Handle specific HTTP status codes
        match status.as_u16() {
            200 => {
                // Success - continue to parse response
            }
            400 => {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Invalid request parameters".to_string());
                return Err(ValyuError::InvalidRequest(error_text));
            }
            401 | 403 => {
                return Err(ValyuError::InvalidApiKey);
            }
            402 => {
                return Err(ValyuError::ApiError("Insufficient credits".to_string()));
            }
            429 => {
                return Err(ValyuError::RateLimitExceeded);
            }
            503 => {
                return Err(ValyuError::ServiceUnavailable);
            }
            _ => {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                return Err(ValyuError::ApiError(format!(
                    "HTTP {}: {}",
                    status, error_text
                )));
            }
        }

        let answer_response: AnswerResponse = response.json().await.map_err(|e| {
            ValyuError::ParseError(format!("Failed to parse API response: {}", e))
        })?;

        // Check if the API returned an error in the response body
        if !answer_response.success {
            if let Some(error) = &answer_response.error {
                return Err(ValyuError::ApiError(error.clone()));
            }
            return Err(ValyuError::ApiError(
                "API request was not successful".to_string(),
            ));
        }

        Ok(answer_response)
    }

    /// Convenience method to get an answer with default settings
    ///
    /// This is equivalent to creating an `AnswerRequest` with just a query
    /// and calling `answer`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use valyu::ValyuClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = ValyuClient::new("your-api-key");
    ///     let response = client.ask("What is quantum computing?").await?;
    ///
    ///     if let Some(contents) = &response.contents {
    ///         println!("Answer: {}", contents);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The HTTP request fails
    /// - The API returns an error response
    /// - The response cannot be parsed
    pub async fn ask(&self, query: impl Into<String>) -> Result<AnswerResponse> {
        let request = AnswerRequest::new(query);
        self.answer(&request).await
    }

    // ========== DeepResearch API Methods ==========

    /// Create a new DeepResearch task
    ///
    /// Starts an asynchronous research task that performs comprehensive research
    /// across multiple sources and generates detailed reports.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use valyu::{ValyuClient, DeepResearchCreateRequest, DeepResearchMode};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = ValyuClient::new("your-api-key");
    ///
    ///     let request = DeepResearchCreateRequest::new("What are the key differences between RAG and fine-tuning?")
    ///         .with_mode(DeepResearchMode::Lite)
    ///         .with_output_formats(vec!["markdown".to_string()]);
    ///
    ///     let response = client.deepresearch_create(&request).await?;
    ///     println!("Task created: {:?}", response.deepresearch_id);
    ///     Ok(())
    /// }
    /// ```
    pub async fn deepresearch_create(
        &self,
        request: &DeepResearchCreateRequest,
    ) -> Result<DeepResearchCreateResponse> {
        let url = format!("{}/deepresearch/tasks", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("x-api-key", &self.api_key)
            .json(request)
            .send()
            .await?;

        let status = response.status();

        match status.as_u16() {
            200 | 201 | 202 => {
                // Success - continue to parse response
            }
            400 => {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Invalid request parameters".to_string());
                return Err(ValyuError::InvalidRequest(error_text));
            }
            401 | 403 => {
                return Err(ValyuError::InvalidApiKey);
            }
            402 => {
                return Err(ValyuError::ApiError("Insufficient credits".to_string()));
            }
            429 => {
                return Err(ValyuError::RateLimitExceeded);
            }
            503 => {
                return Err(ValyuError::ServiceUnavailable);
            }
            _ => {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                return Err(ValyuError::ApiError(format!(
                    "HTTP {}: {}",
                    status, error_text
                )));
            }
        }

        let create_response: DeepResearchCreateResponse = response.json().await.map_err(|e| {
            ValyuError::ParseError(format!("Failed to parse API response: {}", e))
        })?;

        if !create_response.success {
            if let Some(error) = &create_response.error {
                return Err(ValyuError::ApiError(error.clone()));
            }
            return Err(ValyuError::ApiError(
                "API request was not successful".to_string(),
            ));
        }

        Ok(create_response)
    }

    /// Get the status of a DeepResearch task
    ///
    /// # Example
    ///
    /// ```no_run
    /// use valyu::ValyuClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = ValyuClient::new("your-api-key");
    ///     let status = client.deepresearch_status("task-id-here").await?;
    ///     println!("Status: {:?}", status.status);
    ///     Ok(())
    /// }
    /// ```
    pub async fn deepresearch_status(
        &self,
        task_id: impl AsRef<str>,
    ) -> Result<DeepResearchStatusResponse> {
        let url = format!(
            "{}/deepresearch/tasks/{}/status",
            self.base_url,
            task_id.as_ref()
        );

        let response = self
            .client
            .get(&url)
            .header("x-api-key", &self.api_key)
            .send()
            .await?;

        let status = response.status();

        match status.as_u16() {
            200 => {
                // Success
            }
            401 | 403 => {
                return Err(ValyuError::InvalidApiKey);
            }
            404 => {
                return Err(ValyuError::ApiError("Task not found".to_string()));
            }
            429 => {
                return Err(ValyuError::RateLimitExceeded);
            }
            503 => {
                return Err(ValyuError::ServiceUnavailable);
            }
            _ => {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                return Err(ValyuError::ApiError(format!(
                    "HTTP {}: {}",
                    status, error_text
                )));
            }
        }

        let status_response: DeepResearchStatusResponse = response.json().await.map_err(|e| {
            ValyuError::ParseError(format!("Failed to parse API response: {}", e))
        })?;

        Ok(status_response)
    }

    /// Wait for a DeepResearch task to complete
    ///
    /// Polls the task status until it completes, fails, or is cancelled.
    ///
    /// # Arguments
    ///
    /// * `task_id` - The task identifier
    /// * `poll_interval_secs` - Seconds between status checks (default: 5)
    /// * `max_wait_secs` - Maximum seconds to wait (default: 900 for lite, 5400 for heavy)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use valyu::ValyuClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = ValyuClient::new("your-api-key");
    ///     let result = client.deepresearch_wait("task-id", 5, 900).await?;
    ///
    ///     if let Some(output) = &result.output {
    ///         println!("Research output: {}", output);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn deepresearch_wait(
        &self,
        task_id: impl AsRef<str>,
        poll_interval_secs: u64,
        max_wait_secs: u64,
    ) -> Result<DeepResearchStatusResponse> {
        let task_id = task_id.as_ref();
        let start = std::time::Instant::now();
        let max_duration = std::time::Duration::from_secs(max_wait_secs);
        let poll_duration = std::time::Duration::from_secs(poll_interval_secs);

        loop {
            let status = self.deepresearch_status(task_id).await?;

            match &status.status {
                Some(DeepResearchStatus::Completed) => return Ok(status),
                Some(DeepResearchStatus::Failed) => {
                    let error_msg = status
                        .error
                        .clone()
                        .unwrap_or_else(|| "Task failed".to_string());
                    return Err(ValyuError::ApiError(error_msg));
                }
                Some(DeepResearchStatus::Cancelled) => {
                    return Err(ValyuError::ApiError("Task was cancelled".to_string()));
                }
                _ => {
                    // Still queued or running
                    if start.elapsed() > max_duration {
                        return Err(ValyuError::ApiError(format!(
                            "Maximum wait time of {} seconds exceeded",
                            max_wait_secs
                        )));
                    }
                    tokio::time::sleep(poll_duration).await;
                }
            }
        }
    }

    /// List DeepResearch tasks
    ///
    /// # Example
    ///
    /// ```no_run
    /// use valyu::ValyuClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = ValyuClient::new("your-api-key");
    ///     let tasks = client.deepresearch_list("api-key-id", Some(50)).await?;
    ///
    ///     if let Some(data) = &tasks.data {
    ///         for task in data {
    ///             println!("{}: {:?}", task.query, task.status);
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn deepresearch_list(
        &self,
        api_key_id: impl AsRef<str>,
        limit: Option<u32>,
    ) -> Result<DeepResearchListResponse> {
        let mut url = format!(
            "{}/deepresearch/list?api_key_id={}",
            self.base_url,
            api_key_id.as_ref()
        );

        if let Some(l) = limit {
            url.push_str(&format!("&limit={}", l));
        }

        let response = self
            .client
            .get(&url)
            .header("x-api-key", &self.api_key)
            .send()
            .await?;

        let status = response.status();

        match status.as_u16() {
            200 => {
                // Success
            }
            401 | 403 => {
                return Err(ValyuError::InvalidApiKey);
            }
            429 => {
                return Err(ValyuError::RateLimitExceeded);
            }
            _ => {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                return Err(ValyuError::ApiError(format!(
                    "HTTP {}: {}",
                    status, error_text
                )));
            }
        }

        let list_response: DeepResearchListResponse = response.json().await.map_err(|e| {
            ValyuError::ParseError(format!("Failed to parse API response: {}", e))
        })?;

        Ok(list_response)
    }

    /// Add follow-up instructions to a running task
    ///
    /// # Example
    ///
    /// ```no_run
    /// use valyu::ValyuClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = ValyuClient::new("your-api-key");
    ///     client.deepresearch_update("task-id", "Focus more on peer-reviewed sources").await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn deepresearch_update(
        &self,
        task_id: impl AsRef<str>,
        instruction: impl Into<String>,
    ) -> Result<DeepResearchOperationResponse> {
        let url = format!(
            "{}/deepresearch/tasks/{}/update",
            self.base_url,
            task_id.as_ref()
        );

        let body = serde_json::json!({
            "instruction": instruction.into()
        });

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("x-api-key", &self.api_key)
            .json(&body)
            .send()
            .await?;

        let status = response.status();

        match status.as_u16() {
            200 => {
                // Success
            }
            401 | 403 => {
                return Err(ValyuError::InvalidApiKey);
            }
            404 => {
                return Err(ValyuError::ApiError("Task not found".to_string()));
            }
            429 => {
                return Err(ValyuError::RateLimitExceeded);
            }
            _ => {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                return Err(ValyuError::ApiError(format!(
                    "HTTP {}: {}",
                    status, error_text
                )));
            }
        }

        let update_response: DeepResearchOperationResponse =
            response.json().await.map_err(|e| {
                ValyuError::ParseError(format!("Failed to parse API response: {}", e))
            })?;

        Ok(update_response)
    }

    /// Cancel a running task
    ///
    /// # Example
    ///
    /// ```no_run
    /// use valyu::ValyuClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = ValyuClient::new("your-api-key");
    ///     client.deepresearch_cancel("task-id").await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn deepresearch_cancel(
        &self,
        task_id: impl AsRef<str>,
    ) -> Result<DeepResearchOperationResponse> {
        let url = format!(
            "{}/deepresearch/tasks/{}/cancel",
            self.base_url,
            task_id.as_ref()
        );

        let response = self
            .client
            .post(&url)
            .header("x-api-key", &self.api_key)
            .send()
            .await?;

        let status = response.status();

        match status.as_u16() {
            200 => {
                // Success
            }
            401 | 403 => {
                return Err(ValyuError::InvalidApiKey);
            }
            404 => {
                return Err(ValyuError::ApiError("Task not found".to_string()));
            }
            429 => {
                return Err(ValyuError::RateLimitExceeded);
            }
            _ => {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                return Err(ValyuError::ApiError(format!(
                    "HTTP {}: {}",
                    status, error_text
                )));
            }
        }

        let cancel_response: DeepResearchOperationResponse =
            response.json().await.map_err(|e| {
                ValyuError::ParseError(format!("Failed to parse API response: {}", e))
            })?;

        Ok(cancel_response)
    }

    /// Delete a task
    ///
    /// # Example
    ///
    /// ```no_run
    /// use valyu::ValyuClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = ValyuClient::new("your-api-key");
    ///     client.deepresearch_delete("task-id").await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn deepresearch_delete(
        &self,
        task_id: impl AsRef<str>,
    ) -> Result<DeepResearchOperationResponse> {
        let url = format!(
            "{}/deepresearch/tasks/{}/delete",
            self.base_url,
            task_id.as_ref()
        );

        let response = self
            .client
            .delete(&url)
            .header("x-api-key", &self.api_key)
            .send()
            .await?;

        let status = response.status();

        match status.as_u16() {
            200 => {
                // Success
            }
            401 | 403 => {
                return Err(ValyuError::InvalidApiKey);
            }
            404 => {
                return Err(ValyuError::ApiError("Task not found".to_string()));
            }
            429 => {
                return Err(ValyuError::RateLimitExceeded);
            }
            _ => {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                return Err(ValyuError::ApiError(format!(
                    "HTTP {}: {}",
                    status, error_text
                )));
            }
        }

        let delete_response: DeepResearchOperationResponse =
            response.json().await.map_err(|e| {
                ValyuError::ParseError(format!("Failed to parse API response: {}", e))
            })?;

        Ok(delete_response)
    }

    /// Convenience method to create a research task with default settings
    ///
    /// # Example
    ///
    /// ```no_run
    /// use valyu::ValyuClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = ValyuClient::new("your-api-key");
    ///     let task = client.research("What is quantum computing?").await?;
    ///     println!("Task ID: {:?}", task.deepresearch_id);
    ///     Ok(())
    /// }
    /// ```
    pub async fn research(
        &self,
        query: impl Into<String>,
    ) -> Result<DeepResearchCreateResponse> {
        let request = DeepResearchCreateRequest::new(query);
        self.deepresearch_create(&request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = ValyuClient::new("test-key");
        assert_eq!(client.api_key, "test-key");
        assert_eq!(client.base_url, API_BASE_URL);
    }

    #[test]
    fn test_client_with_custom_url() {
        let client = ValyuClient::with_base_url("test-key", "https://custom.url");
        assert_eq!(client.api_key, "test-key");
        assert_eq!(client.base_url, "https://custom.url");
    }
}
