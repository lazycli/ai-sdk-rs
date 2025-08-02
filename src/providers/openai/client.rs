//! This module provides a client for interacting with the OpenAI API.
//! It handles the construction of requests, sending them to the API,
//! and parsing the responses.

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

const API_BASE_URL: &str = "https://api.openai.com/v1";

/// A client for the OpenAI API.
#[derive(Debug, Clone)]
pub struct OpenAIClient {
    client: Client,
    api_key: String,
}

impl OpenAIClient {
    /// Creates a new `OpenAIClient` with the given API key.
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    /// Sends a request to the OpenAI API to generate text.
    pub async fn generate_text(&self, request: &ChatCompletionRequest) -> Result<ChatCompletionResponse> {
        let url = format!("{}/chat/completions", API_BASE_URL);
        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::ApiError(response.text().await?));
        }

        Ok(response.json().await?)
    }
}

// TODO: improve the request and response types to fully match the OpenAI API

/// A request to the OpenAI API to generate a chat completion.
#[derive(Debug, Serialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub max_tokens: u32,
}

/// A message in a chat completion request.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// A response from the OpenAI API for a chat completion request.
#[derive(Debug, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChatChoice>,
}

/// A choice in a chat completion response.
#[derive(Debug, Deserialize)]
pub struct ChatChoice {
    pub index: u32,
    pub message: Message,
    pub finish_reason: String,
}
