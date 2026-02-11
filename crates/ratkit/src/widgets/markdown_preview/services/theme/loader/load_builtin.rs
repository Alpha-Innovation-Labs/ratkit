//! Load builtin themes embedded in the crate.

use crate::widgets::markdown_preview::services::theme::loader::load_theme_str::load_theme_str;
use crate::widgets::markdown_preview::services::theme::AppTheme;
use crate::widgets::markdown_preview::services::theme::ThemeVariant;

/// Available builtin theme names.
///
/// These themes are embedded in the crate and can be loaded without
/// external files.
pub const BUILTIN_THEMES: &[&str] = &[
    "aura",
    "ayu",
    "carbonfox",
    "catppuccin",
    "catppuccin-frappe",
    "catppuccin-macchiato",
    "cobalt2",
    "cursor",
    "dracula",
    "everforest",
    "flexoki",
    "github",
    "gruvbox",
    "kanagawa",
    "lucent-orng",
    "material",
    "matrix",
    "mercury",
    "monokai",
    "nightowl",
    "nord",
    "one-dark",
    "orng",
    "osaka-jade",
    "palenight",
    "rosepine",
    "solarized",
    "synthwave84",
    "tokyonight",
    "vercel",
    "vesper",
    "zenburn",
];

/// Loads a builtin theme by name.
///
/// Builtin themes are embedded in the crate and don't require external files.
/// Use [`BUILTIN_THEMES`] to see available theme names.
///
/// # Arguments
///
/// * `name` - The name of the builtin theme (e.g., "gruvbox", "dracula")
/// * `variant` - Which theme variant (dark/light) to use
///
/// # Returns
///
/// `Ok(AppTheme)` if the theme exists and can be loaded,
/// `Err` with a description if the theme doesn't exist.
///
/// # Example
///
/// ```rust,ignore
/// use ratatui_toolkit::services::theme::{loader, ThemeVariant};
///
/// let theme = loader::load_builtin_theme("gruvbox", ThemeVariant::Dark)
///     .expect("Failed to load gruvbox theme");
/// ```
pub fn load_builtin_theme(name: &str, variant: ThemeVariant) -> Result<AppTheme, String> {
    let json = get_builtin_theme_json(name)?;
    load_theme_str(json, variant)
}

/// Gets the raw JSON for a builtin theme.
fn get_builtin_theme_json(name: &str) -> Result<&'static str, String> {
    match name {
        "aura" => Ok(include_str!("../themes/aura.json")),
        "ayu" => Ok(include_str!("../themes/ayu.json")),
        "carbonfox" => Ok(include_str!("../themes/carbonfox.json")),
        "catppuccin" => Ok(include_str!("../themes/catppuccin.json")),
        "catppuccin-frappe" => Ok(include_str!("../themes/catppuccin-frappe.json")),
        "catppuccin-macchiato" => Ok(include_str!("../themes/catppuccin-macchiato.json")),
        "cobalt2" => Ok(include_str!("../themes/cobalt2.json")),
        "cursor" => Ok(include_str!("../themes/cursor.json")),
        "dracula" => Ok(include_str!("../themes/dracula.json")),
        "everforest" => Ok(include_str!("../themes/everforest.json")),
        "flexoki" => Ok(include_str!("../themes/flexoki.json")),
        "github" => Ok(include_str!("../themes/github.json")),
        "gruvbox" => Ok(include_str!("../themes/gruvbox.json")),
        "kanagawa" => Ok(include_str!("../themes/kanagawa.json")),
        "lucent-orng" => Ok(include_str!("../themes/lucent-orng.json")),
        "material" => Ok(include_str!("../themes/material.json")),
        "matrix" => Ok(include_str!("../themes/matrix.json")),
        "mercury" => Ok(include_str!("../themes/mercury.json")),
        "monokai" => Ok(include_str!("../themes/monokai.json")),
        "nightowl" => Ok(include_str!("../themes/nightowl.json")),
        "nord" => Ok(include_str!("../themes/nord.json")),
        "one-dark" => Ok(include_str!("../themes/one-dark.json")),
        "orng" => Ok(include_str!("../themes/orng.json")),
        "osaka-jade" => Ok(include_str!("../themes/osaka-jade.json")),
        "palenight" => Ok(include_str!("../themes/palenight.json")),
        "rosepine" => Ok(include_str!("../themes/rosepine.json")),
        "solarized" => Ok(include_str!("../themes/solarized.json")),
        "synthwave84" => Ok(include_str!("../themes/synthwave84.json")),
        "tokyonight" => Ok(include_str!("../themes/tokyonight.json")),
        "vercel" => Ok(include_str!("../themes/vercel.json")),
        "vesper" => Ok(include_str!("../themes/vesper.json")),
        "zenburn" => Ok(include_str!("../themes/zenburn.json")),
        _ => Err(format!(
            "Unknown builtin theme: '{}'. Available themes: {:?}",
            name, BUILTIN_THEMES
        )),
    }
}
