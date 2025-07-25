//! Define constants based on feature flags

pub const BIN_NAME_CLI: &str = "addonmanager-cli";
pub const REPO_OWNER: &str = "sogladev";
pub const REPO_NAME: &str = "rs-game-launcher";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(feature = "production")]
pub const IS_PRODUCTION: bool = true;
#[cfg(not(feature = "production"))]
pub const IS_PRODUCTION: bool = false;

pub fn default_figure_text() -> &'static str {
    "Addon Manager"
}

pub fn default_description() -> String {
    if IS_PRODUCTION {
        format!(
            "addon manager - Sogladev v{}\n\
            Bugs or issues: https://github.com/sogladev/rs-game-launcher\n\
            {VERSION}",
            "-".repeat(100)
        )
    } else {
        format!(
            "addon manager - For testing purposes only v{}-demo\n\
            Bugs or issues: https://github.com/sogladev/rs-game-launcher\n\
            {VERSION}",
            "-".repeat(100)
        )
    }
}
