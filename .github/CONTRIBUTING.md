# Contributing to AI-SDK-RS

First off, thank you for considering contributing to `ai-sdk-rs`! Your help is essential for keeping it great.

This document provides guidelines for contributing to the project. Please read it carefully to ensure a smooth and effective contribution process.

## How to Contribute

### Fork, Clone, and Create a Branch

1.  **Fork the repository** on GitHub.
2.  **Clone your forked repository** to your local machine:
    ```sh
    git clone https://github.com/[your-username]/ai-sdk-rs.git
    ```
3.  **Create a new branch** for your changes. Choose a descriptive name:
    ```sh
    git checkout -b feat/your-feature-name
    ```

### Commit Messages

We use the [**Conventional Commits**](https://www.conventionalcommits.org/en/v1.0.0/) specification for our commit messages. This helps us automate changelogs and maintain a clear commit history.

A commit message should be structured as follows:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

**Types**:

*   `feat`: A new feature.
*   `fix`: A bug fix.
*   `docs`: Documentation only changes.
*   `style`: Changes that do not affect the meaning of the code (white-space, formatting, etc.).
*   `refactor`: A code change that neither fixes a bug nor adds a feature.
*   `perf`: A code change that improves performance.
*   `test`: Adding missing tests or correcting existing tests.
*   `chore`: Changes to the build process or auxiliary tools.

**Example**:

```
feat(api): add support for Cohere models

This commit introduces a new client for interacting with Cohere's API, expanding the library's multi-model capabilities.

Closes #42
```

### Pull Request (PR) Checklist

Before submitting a pull request, please ensure the following:

*   [ ] **Tests**: Your changes are covered by tests.
*   [ ] **Linting**: The code follows the project's style guidelines. Run `cargo fmt` and `cargo clippy`.
*   [ ] **Description**: The PR description is clear and explains the changes.
*   [ ] **Conventional Commits**: Your commit messages follow the Conventional Commits standard.

## Code of Conduct

We are committed to providing a friendly, safe, and welcoming environment for all, regardless of gender, sexual orientation, disability, ethnicity, or religion. By participating in this project, you agree to abide by our **Code of Conduct**. Please read it [here](./CODE_OF_CONDUCT.md).