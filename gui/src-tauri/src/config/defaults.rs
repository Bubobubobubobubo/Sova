use super::types::{AppearanceConfig, Config, EditorConfig, EditorMode};

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            mode: EditorMode::Normal,
            font_size: 14.0,
            show_line_numbers: true,
            line_wrapping: false,
            highlight_active_line: true,
            cursor_blink_rate: 1200,
            tab_size: 4,
            indent_unit: "  ".to_string(),
            use_tabs: false,
            close_brackets: true,
            bracket_matching: true,
            autocomplete: true,
            rectangular_selection: true,
            fold_gutter: true,
            match_highlighting: true,
        }
    }
}

impl Default for AppearanceConfig {
    fn default() -> Self {
        Self {
            theme: "monokai".to_string(),
            transparency: 100,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            editor: EditorConfig::default(),
            appearance: AppearanceConfig::default(),
        }
    }
}
