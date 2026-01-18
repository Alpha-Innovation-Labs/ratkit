//! Find syntax method for SyntaxHighlighter.

use crate::markdown_renderer::syntax_highlighter::SyntaxHighlighter;

#[cfg(feature = "markdown")]
impl SyntaxHighlighter {
    /// Find a syntax definition for given language identifier.
    pub(crate) fn find_syntax(&self, language: &str) -> Option<syntect::parsing::SyntaxReference> {
        if language.is_empty() {
            return None;
        }

        self.syntax_set
            .find_syntax_by_token(language)
            .or_else(|| self.syntax_set.find_syntax_by_name(language))
            .or_else(|| self.syntax_set.find_syntax_by_extension(language))
            .cloned()
    }
}
