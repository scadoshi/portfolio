//! Remembers the picked theme in `localStorage`. On the server build these
//! are no-ops, so SSR renders [`ThemeConfig::default`] and the client adopts
//! the stored theme after hydration.

use zwipe_components::ThemeConfig;

#[cfg(target_arch = "wasm32")]
mod imp {
    use super::ThemeConfig;
    use gloo_storage::{LocalStorage, Storage};

    /// `localStorage` key for the JSON-serialized [`ThemeConfig`].
    const KEY: &str = "zwipe.theme";

    pub fn load() -> Option<ThemeConfig> {
        LocalStorage::get(KEY).ok()
    }

    pub fn save(cfg: &ThemeConfig) {
        let _ = LocalStorage::set(KEY, cfg);
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod imp {
    use super::ThemeConfig;

    pub fn load() -> Option<ThemeConfig> {
        None
    }

    pub fn save(_cfg: &ThemeConfig) {}
}

pub use imp::{load, save};
