//! Settings lookups for every crate: the `SSX3_*` options (`docs/options.md`),
//! paths and debug switches alike.
//!
//! The process environment wins. A key the environment does not set comes
//! from the front end's settings ([`set_provider`]: a settings file, an
//! app's preferences, the browser page's values), then from the process's
//! defaults ([`set_default`]: e.g. the engine's short memory card messages).
//! Empty values from the provider or the defaults count as unset; an empty
//! environment variable is returned as it is (as reading the environment
//! directly did).
//!
//! Lookups are not cached here: a site taken often caches its value
//! (`OnceLock`) as it would an environment read.

use std::ffi::OsString;
use std::sync::{OnceLock, RwLock};

/// A front end's settings: the value of a key, `None` (or empty) for unset.
pub type Provider = Box<dyn Fn(&str) -> Option<String> + Send + Sync>;

static PROVIDER: OnceLock<Provider> = OnceLock::new();
static DEFAULTS: RwLock<Vec<(String, String)>> = RwLock::new(Vec::new());

/// Install the front end's settings, once per process (`ssx3_engine::Engine::boot`
/// does it with its `Settings`). A second call returns the rejected provider.
pub fn set_provider(p: Provider) -> Result<(), Provider> {
    PROVIDER.set(p)
}

/// Whether a provider is installed.
pub fn has_provider() -> bool {
    PROVIDER.get().is_some()
}

/// A default for `key` below the environment and the provider (replaces an
/// earlier default for the same key).
pub fn set_default(key: &str, value: impl Into<String>) {
    let value = value.into();
    let mut d = DEFAULTS.write().unwrap_or_else(|e| e.into_inner());
    match d.iter_mut().find(|(k, _)| k == key) {
        Some(e) => e.1 = value,
        None => d.push((key.to_string(), value)),
    }
}

fn fallback(key: &str) -> Option<String> {
    if let Some(v) = PROVIDER.get().and_then(|p| p(key)) {
        if !v.is_empty() {
            return Some(v);
        }
    }
    let d = DEFAULTS.read().unwrap_or_else(|e| e.into_inner());
    d.iter()
        .find(|(k, v)| k == key && !v.is_empty())
        .map(|(_, v)| v.clone())
}

/// `key` as UTF-8 (`std::env::var(key).ok()` with the front end's settings
/// under it). A non-UTF-8 environment value is `None`, as it was.
pub fn var(key: &str) -> Option<String> {
    match std::env::var(key) {
        Ok(v) => Some(v),
        Err(std::env::VarError::NotPresent) => fallback(key),
        Err(std::env::VarError::NotUnicode(_)) => None,
    }
}

/// `key` as an OS string (`std::env::var_os(key)` with the front end's
/// settings under it).
pub fn var_os(key: &str) -> Option<OsString> {
    std::env::var_os(key).or_else(|| fallback(key).map(OsString::from))
}

/// Whether `key` is set at all (`std::env::var_os(key).is_some()`; an empty
/// environment variable counts).
pub fn is_set(key: &str) -> bool {
    var_os(key).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn environment_then_provider_then_defaults() {
        // Keys of this test only; the provider is process-wide.
        let _ = set_provider(Box::new(|k| match k {
            "SSX3_CONFIG_TEST_P" => Some("provider".into()),
            "SSX3_CONFIG_TEST_EMPTY" => Some(String::new()),
            "SSX3_CONFIG_TEST_ENV" => Some("provider".into()),
            _ => None,
        }));
        if !has_provider() {
            return;
        }
        set_default("SSX3_CONFIG_TEST_P", "default");
        set_default("SSX3_CONFIG_TEST_D", "default");
        set_default("SSX3_CONFIG_TEST_EMPTY", "default");
        assert_eq!(var("SSX3_CONFIG_TEST_P").as_deref(), Some("provider"));
        assert_eq!(var("SSX3_CONFIG_TEST_D").as_deref(), Some("default"));
        // Empty from the provider: unset there, the default applies.
        assert_eq!(var("SSX3_CONFIG_TEST_EMPTY").as_deref(), Some("default"));
        assert_eq!(var("SSX3_CONFIG_TEST_NONE"), None);
        assert!(!is_set("SSX3_CONFIG_TEST_NONE"));
        std::env::set_var("SSX3_CONFIG_TEST_ENV", "");
        // An empty environment variable is set and wins.
        assert_eq!(var("SSX3_CONFIG_TEST_ENV").as_deref(), Some(""));
        assert!(is_set("SSX3_CONFIG_TEST_ENV"));
        std::env::set_var("SSX3_CONFIG_TEST_ENV", "env");
        assert_eq!(var_os("SSX3_CONFIG_TEST_ENV"), Some("env".into()));
        std::env::remove_var("SSX3_CONFIG_TEST_ENV");
        assert_eq!(var("SSX3_CONFIG_TEST_ENV").as_deref(), Some("provider"));
    }
}
