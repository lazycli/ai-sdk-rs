

pub mod prompt;

#[cfg(test)]
mod tests {
    use crate::prompt::prompt::{Prompt, Promptable};

    #[test]
    fn it_works() {
        let prompt = Prompt::new("test");
        assert_eq!(prompt.name(), "test");
    }
}
