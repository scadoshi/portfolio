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
        let mode = if self.is_dark { "dark" } else { "light" };
        format!("theme-wrapper theme-{}-{}", self.name, mode)
    }

    pub fn has_light_mode(&self) -> bool {
        true
    }
}

pub const THEMES: &[(&str, &str)] = &[
    ("catppuccin", "Catppuccin"),
    ("dracula", "Dracula"),
    ("everforest", "Everforest"),
    ("gruvbox", "Gruvbox"),
    ("monokai", "Monokai"),
    ("nord", "Nord"),
    ("one-dark", "One Dark"),
    ("rose-pine", "Rosé Pine"),
    ("rustbox", "Rustbox"),
    ("solarized", "Solarized"),
    ("tokyo-night", "Tokyo Night"),
    // Colorblind-accessible themes
    ("deuteranopia", "Deuteranopia"),
    ("protanopia", "Protanopia"),
    ("tritanopia", "Tritanopia"),
];
