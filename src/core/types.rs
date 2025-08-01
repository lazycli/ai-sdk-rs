//! Core types for AI SDK functions.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{core::language_model::LanguageModel, error::Error};

/// Options for a `generate_text` call.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(pattern = "owned", setter(into), build_fn(error = "Error"))]
pub struct GenerateTextCallOptions<LM>
where
    LM: LanguageModel + Serialize + Clone,
{
    /// The language model to use for text generation.
    pub model: LM,
    /// The prompt to generate text from.
    pub prompt: String,
}

impl<LM> GenerateTextCallOptions<LM>
where
    LM: LanguageModel + Serialize + Clone,
{
    /// Creates a new builder for `GenerateTextCallOptions`.
    pub fn builder() -> GenerateTextCallOptionsBuilder<LM> {
        GenerateTextCallOptionsBuilder::default()
    }
}

/// Response from a `generate_text` call.
#[derive(Debug)]
pub struct GenerateTextResponse {
    /// The generated text.
    pub text: String,
}

impl GenerateTextResponse {
    /// Creates a new response with the generated text.
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

/// Options for a language model request.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(pattern = "owned", setter(into), build_fn(error = "Error"))]
pub struct LanguageModelCallOptions {
    /// The prompt to generate text from.
    pub prompt: String,
}

impl LanguageModelCallOptions {
    /// Creates a new builder for `LanguageModelCallOptions`.
    pub fn builder() -> LanguageModelCallOptionsBuilder {
        LanguageModelCallOptionsBuilder::default()
    }
}

/// Response from a language model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageModelResponse {
    /// The generated text.
    pub text: String,

    /// The model that generated the response.
    pub model: Option<String>,
}

impl LanguageModelResponse {
    /// Creates a new response with the generated text.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            model: None,
        }
    }
}
