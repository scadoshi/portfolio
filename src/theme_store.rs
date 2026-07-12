//! Local theme persistence.
//!
//! Remembers the visitor's picked theme across reloads via the browser's
//! `localStorage`, so the site opens in their last-used theme instead of the
//! default. Client-only: on the server build (SSR) these are no-ops, so the
//! signal falls back to [`ThemeConfig::default`] and the client adopts the
//! stored theme on hydration.

use zwipe_components::ThemeConfig;

#[cfg(target_arch = "wasm32")]
mod imp {
    use super::ThemeConfig;
    use gloo_storage::{LocalStorage, Storage};

    /// `localStorage` key holding the JSON-serialized [`ThemeConfig`].
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
