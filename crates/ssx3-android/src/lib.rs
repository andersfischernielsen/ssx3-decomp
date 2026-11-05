//! Android application entry point; empty on other platforms, like ssx3-web.
#![cfg(target_os = "android")]

#[path = "../../ssx3/src/main.rs"]
mod frontend;
