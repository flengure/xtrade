pub use crate::bot::state::{
    BotInsertArgs, BotListArgs, BotListView, BotUpdateArgs, BotView, ListenerInsertArgs,
    ListenerListArgs, ListenerListView, ListenerUpdateArgs, ListenerView,
};
use crate::errors::ApiError;
use log::{error, info};
use reqwest::{Client, Method, Response};

#[derive(Debug, Clone)]
pub struct RestClient {
    base_url: String,
    client: Client,
}

impl RestClient {
    /// Create a new `RestClient` with the given base URL.
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            client: Client::new(),
        }
    }

    /// Helper to send an HTTP request with an optional JSON body.
    async fn send_request<T: serde::Serialize + std::fmt::Debug>(
        &self,
        method: Method,
        url: &str,
        body: Option<T>,
    ) -> Result<Response, ApiError> {
        info!("Sending {:?} request to URL: {}", method, url);

        let request = self.client.request(method, url);

        let request = if let Some(body) = body {
            info!("Request body: {:?}", &body);
            request.json(&body)
        } else {
            request
        };

        request.send().await.map_err(|e| {
            error!("Request to {} failed: {}", url, e);
            ApiError::ConnectionError(format!("Failed to send request: {}", e))
        })
    }

    /// Helper to process the HTTP response into the desired type.
    async fn process_response<T: serde::de::DeserializeOwned>(
        response: reqwest::Response,
    ) -> Result<T, ApiError> {
        if response.status().is_success() {
            info!("Received successful response: {:?}", response.status());
            response
                .json::<T>()
                .await
                .map_err(|e| ApiError::ParseError(format!("Failed to parse response: {}", e)))
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("HTTP error {}: {}", status.as_u16(), error_text);
            Err(ApiError::HttpError(status.as_u16(), error_text))
        }
    }

    /// Helper to process and log the HTTP response.
    async fn process_and_log_response<T: serde::de::DeserializeOwned + std::fmt::Debug>(
        response: reqwest::Response,
    ) -> Result<T, ApiError> {
        let result: T = Self::process_response(response).await?;
        info!("Parsed response: {:?}", result);
        Ok(result)
    }

    /// Add a new bot.
    pub async fn add_bot(&self, bot: BotInsertArgs) -> Result<BotView, ApiError> {
        let response = self
            .send_request(Method::POST, &format!("{}/bots", self.base_url), Some(&bot))
            .await?;
        Self::process_and_log_response(response).await
    }

    /// List bots with optional pagination and filtering.
    pub async fn get_bots(
        &self,
        page: Option<u32>,
        limit: Option<u32>,
        filter: Option<BotListArgs>,
    ) -> Result<BotListView, ApiError> {
        let mut url = reqwest::Url::parse(&format!("{}/bots", self.base_url))
            .map_err(|e| ApiError::ConnectionError(format!("Invalid URL: {}", e)))?;

        // Add query parameters for pagination if they are provided
        if let Some(page_val) = page {
            url.query_pairs_mut()
                .append_pair("page", &page_val.to_string());
        }
        if let Some(limit_val) = limit {
            url.query_pairs_mut()
                .append_pair("limit", &limit_val.to_string());
        }

        // Serialize the filter into JSON (if provided)
        let body = filter
            .map(|f| {
                serde_json::to_value(f).map_err(|e| ApiError::SerializationError(e.to_string()))
            })
            .transpose()?;

        // Send the GET request
        let response = self
            .send_request(Method::GET, url.as_str(), body.as_ref())
            .await?;
        Self::process_and_log_response(response).await
    }

    /// Retrieve a single bot by its ID.
    pub async fn get_bot(&self, bot_id: &str) -> Result<BotView, ApiError> {
        let response = self
            .send_request(
                Method::GET,
                &format!("{}/bots/{}", self.base_url, bot_id),
                None::<()>,
            )
            .await?;
        Self::process_and_log_response(response).await
    }

    /// Update a bot by ID.
    pub async fn update_bot(
        &self,
        bot_id: &str,
        update_data: BotUpdateArgs,
    ) -> Result<BotView, ApiError> {
        let response = self
            .send_request(
                Method::PUT,
                &format!("{}/bots/{}", self.base_url, bot_id),
                Some(&update_data),
            )
            .await?;
        Self::process_and_log_response(response).await
    }

    /// Delete a bot by ID.
    pub async fn delete_bot(&self, bot_id: &str) -> Result<String, ApiError> {
        let response = self
            .send_request(
                Method::DELETE,
                &format!("{}/bots/{}", self.base_url, bot_id),
                None::<()>,
            )
            .await?;
        Self::process_and_log_response(response).await
    }

    /// Add a listener to a bot.
    pub async fn add_listener(
        &self,
        bot_id: &str,
        args: ListenerInsertArgs,
    ) -> Result<ListenerView, ApiError> {
        let response = self
            .send_request(
                Method::POST,
                &format!("{}/bots/{}/listeners", self.base_url, bot_id),
                Some(&args),
            )
            .await?;
        Self::process_and_log_response(response).await
    }

    /// List listeners for a bot with optional pagination and filtering.
    pub async fn get_listeners(
        &self,
        bot_id: &str,
        page: Option<u32>,
        limit: Option<u32>,
        filter: Option<ListenerListArgs>,
    ) -> Result<ListenerListView, ApiError> {
        let mut url = reqwest::Url::parse(&format!("{}/bots/{}/listeners", self.base_url, bot_id))
            .map_err(|e| ApiError::ConnectionError(format!("Invalid URL: {}", e)))?;

        // Add query parameters for pagination if they are provided
        if let Some(page_val) = page {
            url.query_pairs_mut()
                .append_pair("page", &page_val.to_string());
        }
        if let Some(limit_val) = limit {
            url.query_pairs_mut()
                .append_pair("limit", &limit_val.to_string());
        }

        // Serialize the filter into JSON (if provided)
        let body = filter
            .map(|f| {
                serde_json::to_value(f).map_err(|e| ApiError::SerializationError(e.to_string()))
            })
            .transpose()?;

        // Send the GET request
        let response = self
            .send_request(Method::GET, url.as_str(), body.as_ref())
            .await?;
        Self::process_and_log_response(response).await
    }

    /// Retrieve a specific listener by bot ID and listener ID.
    pub async fn get_listener(
        &self,
        bot_id: &str,
        listener_id: &str,
    ) -> Result<ListenerView, ApiError> {
        let response = self
            .send_request(
                Method::GET,
                &format!(
                    "{}/bots/{}/listeners/{}",
                    self.base_url, bot_id, listener_id
                ),
                None::<()>,
            )
            .await?;
        Self::process_and_log_response(response).await
    }

    /// Update a specific listener by bot ID and listener ID.
    pub async fn update_listener(
        &self,
        bot_id: &str,
        listener_id: &str,
        update: ListenerUpdateArgs,
    ) -> Result<ListenerView, ApiError> {
        let response = self
            .send_request(
                Method::PUT,
                &format!(
                    "{}/bots/{}/listeners/{}",
                    self.base_url, bot_id, listener_id
                ),
                Some(&update),
            )
            .await?;
        Self::process_and_log_response(response).await
    }

    /// Delete a specific listener by bot ID and listener ID.
    pub async fn delete_listener(
        &self,
        bot_id: &str,
        listener_id: &str,
    ) -> Result<String, ApiError> {
        let response = self
            .send_request(
                Method::DELETE,
                &format!(
                    "{}/bots/{}/listeners/{}",
                    self.base_url, bot_id, listener_id
                ),
                None::<()>,
            )
            .await?;
        Self::process_and_log_response(response).await
    }

    /// Delete multiple listeners for a bot based on filters.
    pub async fn delete_listeners(
        &self,
        bot_id: &str,
        filter: Option<ListenerListArgs>,
    ) -> Result<ListenerListView, ApiError> {
        let response = if let Some(filter_payload) = filter {
            self.send_request(
                Method::DELETE,
                &format!("{}/bots/{}/listeners", self.base_url, bot_id),
                Some(&filter_payload),
            )
            .await?
        } else {
            self.send_request(
                Method::DELETE,
                &format!("{}/bots/{}/listeners", self.base_url, bot_id),
                None::<()>,
            )
            .await?
        };
        Self::process_and_log_response(response).await
    }
}
