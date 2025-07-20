//! File based prompt templates that use the `tera` template engine. the module
//! searches for prompt templates in the `./prompts` directory relative to top level.
//! directory of the project or a directory provided by the user through an environment
//! variable. provides tools
//!     - load prompts from a directory
//!     - substitute other prompts, variables, conditionals, loops, etc
//!     - incrementally set variables a prompts with the builder pattern
//!     - logging and telemetery 
//!
//! # Examples
//!
//! ```rust
//! use ai_sdk_rs::prompt::Prompt;
//!
//! Prompt::new("system/base")  // loads from ./prompts/system/base or {PROMPT_DIR}/system/base
//!    .with("variable1", "value1")
//!    .with("variable2", "value2")
//!    .with("variable3", "value3")
//!    .generate()
//! ```

pub mod prompt {
    use std::collections::HashMap;
    use std::path::Path;

    type PromptVariables = HashMap<String, String>;

    pub trait Promptable {
        // generates the prompt
        fn generate(&self) -> String;

        // adds a variable to the prompt. does not replace a variable if already set
        fn with(&self, variable: &str, value: &str) -> Self;

        // adds a variable to the prompt. replaces a variable if already set
        fn with_overwrite(&self, variable: &str, value: &str) -> Self;

        fn file_path(&self) -> Path;

        // remove the file extention and everything until but not including ./prompt
        fn name(&self) -> String;

        // current state of variables
        fn variables(&self) -> PromptVariables;
    }

    pub struct Prompt {
        _name: String,  // name provided by the user
        _variables: PromptVariables  // current state of variables
    }
}
