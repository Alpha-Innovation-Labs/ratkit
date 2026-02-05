//! Load theme from JSON string.

use std::collections::HashMap;
use crate::services::theme::AppTheme;
use crate::services::theme::ThemeVariant;

use ratatui::style::Color;

use crate::services::theme::diff_colors::DiffColors;
use crate::services::theme::loader::resolve_defs::resolve_color_value;
use crate::services::theme::loader::theme_json::{ColorValue, ThemeJson};
use crate::services::theme::markdown_colors::MarkdownColors;
use crate::services::theme::syntax_colors::SyntaxColors;

/// Loads an [`AppTheme`] from a JSON string in opencode format.
///
/// Parses the JSON, resolves all color references, and constructs
/// a complete `AppTheme` with all color categories populated.
///
/// # Arguments
///
/// * `json` - The JSON string to parse
/// * `variant` - Which theme variant (dark/light) to use
///
/// # Returns
///
/// `Ok(AppTheme)` if parsing and resolution succeeds,
/// `Err` with a description if parsing fails.
///
/// # Errors
///
/// Returns an error if:
/// - The JSON is malformed
/// - Required color keys are missing
/// - Color values cannot be resolved
///
/// # Example
///
/// ```rust,ignore
/// use ratatui_toolkit::services::theme::{loader, ThemeVariant};
///
/// let json = r#"{
///   "defs": { "bg": "#282828", "fg": "#ebdbb2" },
///   "theme": {
///     "background": "bg",
///     "text": "fg"
///   }
/// }"#;
///
/// let theme = loader::load_theme_str(json, ThemeVariant::Dark);
/// assert!(theme.is_ok());
/// ```
pub fn load_theme_str(json: &str, variant: ThemeVariant) -> Result<AppTheme, String> {
    let theme_json: ThemeJson =
        serde_json::from_str(json).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    build_theme_from_json(&theme_json, variant)
}

/// Builds an AppTheme from parsed ThemeJson.
pub(crate) fn build_theme_from_json(
    theme_json: &ThemeJson,
    variant: ThemeVariant,
) -> Result<AppTheme, String> {
    let defs = &theme_json.defs;
    let theme = &theme_json.theme;

    // Helper to resolve a color with a default
    let resolve = |key: &str, default: Color| -> Color {
        theme
            .get(key)
            .and_then(|v| resolve_color_value(v, defs, variant))
            .unwrap_or(default)
    };

    // Get default theme for fallback values
    let default = AppTheme::default();

    // Build UI colors
    let primary = resolve("primary", default.primary);
    let secondary = resolve("secondary", default.secondary);
    let accent = resolve("accent", default.accent);
    let error = resolve("error", default.error);
    let warning = resolve("warning", default.warning);
    let success = resolve("success", default.success);
    let info = resolve("info", default.info);

    // Build text colors
    let text = resolve("text", default.text);
    let text_muted = resolve("textMuted", default.text_muted);
    // Selected text falls back to text if not specified
    let selected_text = theme
        .get("selectedText")
        .and_then(|v| resolve_color_value(v, defs, variant))
        .unwrap_or(text);

    // Build background colors
    let background = resolve("background", default.background);
    let background_panel = resolve("backgroundPanel", default.background_panel);
    let background_element = resolve("backgroundElement", default.background_element);
    // Menu background falls back to panel if not specified
    let background_menu = theme
        .get("backgroundMenu")
        .and_then(|v| resolve_color_value(v, defs, variant))
        .unwrap_or(background_panel);

    // Build border colors
    let border = resolve("border", default.border);
    let border_active = resolve("borderActive", default.border_active);
    let border_subtle = resolve("borderSubtle", default.border_subtle);

    // Build diff colors
    let diff = build_diff_colors(theme, defs, variant);

    // Build markdown colors
    let markdown = build_markdown_colors(theme, defs, variant);

    // Build syntax colors
    let syntax = build_syntax_colors(theme, defs, variant);

    Ok(AppTheme {
        primary,
        secondary,
        accent,
        error,
        warning,
        success,
        info,
        text,
        text_muted,
        selected_text,
        background,
        background_panel,
        background_element,
        background_menu,
        border,
        border_active,
        border_subtle,
        diff,
        markdown,
        syntax,
    })
}

/// Builds DiffColors from theme JSON.
fn build_diff_colors(
    theme: &HashMap<String, ColorValue>,
    defs: &HashMap<String, String>,
    variant: ThemeVariant,
) -> DiffColors {
    let default = DiffColors::default();

    let resolve = |key: &str, default: Color| -> Color {
        theme
            .get(key)
            .and_then(|v| resolve_color_value(v, defs, variant))
            .unwrap_or(default)
    };

    DiffColors {
        added: resolve("diffAdded", default.added),
        removed: resolve("diffRemoved", default.removed),
        context: resolve("diffContext", default.context),
        hunk_header: resolve("diffHunkHeader", default.hunk_header),
        highlight_added: resolve("diffHighlightAdded", default.highlight_added),
        highlight_removed: resolve("diffHighlightRemoved", default.highlight_removed),
        added_bg: resolve("diffAddedBg", default.added_bg),
        removed_bg: resolve("diffRemovedBg", default.removed_bg),
        context_bg: resolve("diffContextBg", default.context_bg),
        line_number: resolve("diffLineNumber", default.line_number),
        added_line_number_bg: resolve("diffAddedLineNumberBg", default.added_line_number_bg),
        removed_line_number_bg: resolve("diffRemovedLineNumberBg", default.removed_line_number_bg),
    }
}

/// Builds MarkdownColors from theme JSON.
fn build_markdown_colors(
    theme: &HashMap<String, ColorValue>,
    defs: &HashMap<String, String>,
    variant: ThemeVariant,
) -> MarkdownColors {
    let default = MarkdownColors::default();

    let resolve = |key: &str, default: Color| -> Color {
        theme
            .get(key)
            .and_then(|v| resolve_color_value(v, defs, variant))
            .unwrap_or(default)
    };

    MarkdownColors {
        text: resolve("markdownText", default.text),
        heading: resolve("markdownHeading", default.heading),
        link: resolve("markdownLink", default.link),
        link_text: resolve("markdownLinkText", default.link_text),
        code: resolve("markdownCode", default.code),
        block_quote: resolve("markdownBlockQuote", default.block_quote),
        emph: resolve("markdownEmph", default.emph),
        strong: resolve("markdownStrong", default.strong),
        horizontal_rule: resolve("markdownHorizontalRule", default.horizontal_rule),
        list_item: resolve("markdownListItem", default.list_item),
        list_enumeration: resolve("markdownListEnumeration", default.list_enumeration),
        image: resolve("markdownImage", default.image),
        image_text: resolve("markdownImageText", default.image_text),
        code_block: resolve("markdownCodeBlock", default.code_block),
    }
}

/// Builds SyntaxColors from theme JSON.
fn build_syntax_colors(
    theme: &HashMap<String, ColorValue>,
    defs: &HashMap<String, String>,
    variant: ThemeVariant,
) -> SyntaxColors {
    let default = SyntaxColors::default();

    let resolve = |key: &str, default: Color| -> Color {
        theme
            .get(key)
            .and_then(|v| resolve_color_value(v, defs, variant))
            .unwrap_or(default)
    };

    SyntaxColors {
        comment: resolve("syntaxComment", default.comment),
        keyword: resolve("syntaxKeyword", default.keyword),
        function: resolve("syntaxFunction", default.function),
        variable: resolve("syntaxVariable", default.variable),
        string: resolve("syntaxString", default.string),
        number: resolve("syntaxNumber", default.number),
        type_: resolve("syntaxType", default.type_),
        operator: resolve("syntaxOperator", default.operator),
        punctuation: resolve("syntaxPunctuation", default.punctuation),
    }
}
