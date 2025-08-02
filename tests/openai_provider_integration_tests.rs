//! Integration tests for the OpenAI provider.

use ai_sdk_rs::{
    core::{GenerateTextCallOptions, generate_text},
    providers::openai::{OpenAI, OpenAIProviderSettings},
};

#[tokio::test]
async fn test_generate_text_with_openai() {
    // This test requires a valid OpenAI API key to be set in the environment.
    if std::env::var("OPENAI_API_KEY").is_err() {
        println!("Skipping test: OPENAI_API_KEY not set");
        return;
    }

    let settings = OpenAIProviderSettings::builder()
        .api_key(std::env::var("OPENAI_API_KEY").unwrap())
        .model_name("gpt-4o".to_string())
        .build()
        .expect("Failed to build OpenAIProviderSettings");

    let openai = OpenAI::new(settings);

    let options = GenerateTextCallOptions::<OpenAI>::builder()
        .model(openai)
        .prompt(
            "Respond with exactly the word 'hello' in all lowercase.\n 
                Do not include any punctuation, prefixes, or suffixes.",
        )
        .build()
        .expect("Failed to build GenerateTextCallOptions");

    let result = generate_text(options).await;
    assert!(result.is_ok());

    let text = result.as_ref().expect("Failed to get result").text.trim();
    assert!(text.contains("hello"));
}
