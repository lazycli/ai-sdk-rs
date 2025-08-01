//! Defines the `Provider` trait, a core abstraction for AI model providers.
//!
//! This module contains the `Provider` trait, which unifies the behavior of
//! different AI providers like OpenAI, Anthropic, or Google.

use crate::core::language_model::LanguageModel;

/// A marker trait representing a fully configured AI provider.
///
/// The `Provider` trait aggregates all necessary capabilities for a given AI provider,
/// such as `LanguageModel`, and in the future, potentially `ImageModel` or `EmbeddingModel`.
///
/// By implementing `Provider`, a type signals that it is a complete and ready-to-use
/// client for interacting with a specific AI service.
pub trait Provider: Send + Sync + LanguageModel {}
