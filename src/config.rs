#[cfg(debug_assertions)]
pub const APP_ID: &str = "rip.chroma.Hype.Devel";
#[cfg(not(debug_assertions))]
pub const APP_ID: &str = "rip.chroma.Hype";

#[cfg(debug_assertions)]
pub const PROFILE: &str = "Devel";
#[cfg(not(debug_assertions))]
pub const PROFILE: &str = "Release";

pub const GETTEXT_PACKAGE: &str = "hype";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
