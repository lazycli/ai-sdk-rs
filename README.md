# AI-SDK-RS

[![Build Status](https://github.com/lazyai/ai-sdk-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/lazyai/ai-sdk-rs/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Issues](https://img.shields.io/github/issues/lazyai/ai-sdk-rs)](https://github.com/lazyai/ai-sdk-rs/issues)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/lazyai/ai-sdk-rs/pulls)

An open-source Rust library for building AI-powered applications, inspired by the Vercel AI SDK. It provides a robust, type-safe, and easy-to-use interface for interacting with various Large Language Models (LLMs).

## Key Features

- **Multi-Model Support**: Seamlessly switch between different LLM providers (e.g., OpenAI, Anthropic, Cohere) with a unified API.
- **Filesystem-based Prompt Templating**: Organize and manage your prompts efficiently using simple text files.
- **Streaming Support**: Built-in support for streaming responses for real-time applications.
- **Type-Safe**: Leverage Rust's type system to ensure correctness and reliability.
- **Ergonomic API**: Designed to be intuitive and easy to integrate into any Rust project.

## Technologies Used

- **Rust**: Built entirely in Rust for performance, safety, and concurrency.
- **Tokio**: For asynchronous operations.
- **tera**: For Jinja style template rendering.

## Installation

Add `ai-sdk-rs` to your `Cargo.toml` dependencies:

```sh
cargo add ai-sdk-rs
```

## Usage

Here is a basic example of how to use `ai-sdk-rs` to interact with an LLM:

```rust
println!("Comming Soon ...")
```

- For more on prompts see [Prompt Templates](https://github.com/lazyai/ai-sdk-rs/tree/main/prompts/README.md)

## Contribution Guidelines

We welcome contributions from the community! Whether you're fixing a bug, adding a feature, or improving documentation, your help is appreciated.

Please read our [**Contributing Guidelines**](./.github/CONTRIBUTING.md) to get started.

### Commit Message Conventions

We use [**Conventional Commits**](https://www.conventionalcommits.org/en/v1.0.0/) for our commit messages. This helps us maintain a clear and organized commit history. Please see the [contribution guide](./.github/CONTRIBUTING.md#commit-messages) for details.

### Pull Request (PR) Standards

All PRs should be well-documented and include tests for any new code. Please fill out the [**pull request template**](./.github/pull_request_template.md) when submitting a PR.

## Code of Conduct

This project and everyone participating in it is governed by our [**Code of Conduct**](https://github.com/[your-username]/[your-repo]/blob/main/.github/CONTRIBUTING.md#code-of-conduct). By participating, you are expected to uphold this code.

## License

This project is licensed under the **MIT License**. See the [LICENSE](./LICENSE) file for details.
