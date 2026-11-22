//! A plugin's Rust code.

use crate::hooks::{HookError, Hooks};

/// What a code plugin implements. The plugin's folder (manifest, content) is
/// matched by [`Plugin::id`].
pub trait Plugin: Send + Sync {
    /// The same id as in the plugin's `plugin.toml`.
    fn id(&self) -> &str;

    /// Install hooks, after the game's own overrides. Hooks installed before
    /// an error stay in.
    fn install(&self, _hooks: &mut Hooks<'_>) -> Result<(), HookError> {
        Ok(())
    }
}
