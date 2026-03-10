#[derive(Clone, PartialEq)]
pub struct ThemeConfig {
    pub name: String,
    pub is_dark: bool,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            name: "gruvbox".to_string(),
            is_dark: true,
        }
    }
}

impl ThemeConfig {
    pub fn css_class(&self) -> String {
        if self.name == "vantablack" {
            "theme-wrapper vantablack".to_string()
        } else {
            let mode = if self.is_dark { "dark" } else { "light" };
            format!("theme-wrapper {}-{}", self.name, mode)
        }
    }

    pub fn has_light_mode(&self) -> bool {
        self.name != "vantablack"
    }
}

pub const THEMES: &[(&str, &str)] = &[
    ("gruvbox", "Gruvbox"),
    ("everforest", "Everforest"),
    ("catppuccin", "Catppuccin"),
    ("tokyo-night", "Tokyo Night"),
    ("nord", "Nord"),
    ("vantablack", "Vantablack"),
];
