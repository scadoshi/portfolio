#[derive(Clone, PartialEq)]
pub struct ThemeConfig {
    pub name: String,
    pub is_dark: bool,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            name: "rustbox".to_string(),
            is_dark: true,
        }
    }
}

impl ThemeConfig {
    pub fn css_class(&self) -> String {
        if self.name == "vantablack" || self.name == "zwipe" {
            format!("theme-wrapper {}", self.name)
        } else {
            let mode = if self.is_dark { "dark" } else { "light" };
            format!("theme-wrapper {}-{}", self.name, mode)
        }
    }

    pub fn has_light_mode(&self) -> bool {
        self.name != "vantablack" && self.name != "zwipe"
    }
}

pub const THEMES: &[(&str, &str)] = &[
    ("rustbox", "Rustbox"),
    ("gruvbox", "Gruvbox"),
    ("dracula", "Dracula"),
    ("everforest", "Everforest"),
    ("catppuccin", "Catppuccin"),
    ("tokyo-night", "Tokyo Night"),
    ("nord", "Nord"),
    ("zwipe", "Zwipe"),
    ("vantablack", "Vantablack"),
];
