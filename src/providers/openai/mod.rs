//! This module provides the OpenAI provider, which implements the `LanguageModel`
//! and `Provider` traits for interacting with the OpenAI API.

pub mod client;
pub mod settings;

pub use settings::OpenAIProviderSettings;

use self::client::{ChatCompletionRequest, Message, OpenAIClient};
use crate::{
    core::{
        language_model::LanguageModel,
        provider::Provider,
        types::{LanguageModelCallOptions, LanguageModelResponse},
    },
    error::Result,
};
use async_trait::async_trait;
use serde::Serialize;

/// The OpenAI provider.
#[derive(Debug, Clone, Serialize)]
pub struct OpenAI {
    #[serde(skip)]
    client: OpenAIClient,
    settings: OpenAIProviderSettings,
}

impl OpenAI {
    /// Creates a new `OpenAI` provider with the given settings.
    pub fn new(settings: OpenAIProviderSettings) -> Self {
        let client = OpenAIClient::new(settings.api_key.to_string());
        Self { client, settings }
    }
}

impl Provider for OpenAI {}

#[async_trait]
impl LanguageModel for OpenAI {
    fn provider_name(&self) -> &str {
        &self.settings.provider_name
    }

    fn model_name(&self) -> &str {
        &self.settings.model_name
    }

    async fn generate(&self, options: LanguageModelCallOptions) -> Result<LanguageModelResponse> {
        let request = ChatCompletionRequest {
            model: self.model_name().to_string(),
            // TODO: support multiple messages
            messages: vec![Message {
                role: "user".to_string(),
                content: options.prompt,
            }],
            max_tokens: self.settings.max_tokens,
        };

        let response = self.client.generate_text(&request).await?;

        Ok(LanguageModelResponse {
            text: response.choices[0].message.content.to_string(),
            model: Some(response.model),
        })
    }
}
