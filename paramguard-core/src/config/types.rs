use chrono::{DateTime, Utc};
use std::path::PathBuf;

/// Restricts the configuration file formats to the ones listed.
///
/// Supported formats include:
/// - JSON (.json)
/// - YAML (.yaml, .yml)
/// - TOML (.toml)
/// - INI (.ini)
/// - Environment Variables (.env)
/// - Generic Config (.cfg)
/// - Nix (.nix)
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigFormat {
    /// JSON format (.json)
    Json,
    /// YAML format (.yaml, .yml)
    Yaml,
    /// TOML format (.toml)
    Toml,
    /// INI format (.ini)
    Ini,
    /// Environment Variables format (.env)
    Env,
    /// Generic Config format (.cfg)
    Cfg,
    /// Nix format (.nix)
    Nix,
}

/// Represents a configuration file managed by ParamGuard.
///
/// Contains metadata about the configuration file including its location,
/// format, content, and last modification time.
#[derive(Debug, Clone)]
pub struct ConfigFile {
    pub name: String,
    pub path: PathBuf,
    pub format: ConfigFormat,
    pub content: String,
    pub last_modified: DateTime<Utc>,
}

impl ConfigFormat {
    /// Returns the file extension associated with this configuration format.
    ///
    /// # Example
    /// ```
    /// use paramguard_core::config::types::ConfigFormat;
    ///
    /// assert_eq!(ConfigFormat::Json.as_extension(), "json");
    /// assert_eq!(ConfigFormat::Yaml.as_extension(), "yaml");
    /// ```
    pub fn as_extension(&self) -> &'static str {
        match self {
            ConfigFormat::Json => "json",
            ConfigFormat::Yaml => "yaml",
            ConfigFormat::Toml => "toml",
            ConfigFormat::Ini => "ini",
            ConfigFormat::Env => "env",
            ConfigFormat::Cfg => "cfg",
            ConfigFormat::Nix => "nix",
        }
    }

    /// Attempts to create a ConfigFormat from a file extension.
    ///
    /// # Examples
    /// ```
    /// use paramguard_core::config::types::ConfigFormat;
    ///
    /// assert_eq!(ConfigFormat::from_extension("json"), Some(ConfigFormat::Json));
    /// assert_eq!(ConfigFormat::from_extension("yml"), Some(ConfigFormat::Yaml));
    /// assert_eq!(ConfigFormat::from_extension("invalid"), None);
    /// ```
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "json" => Some(Self::Json),
            "yaml" | "yml" => Some(Self::Yaml),
            "toml" => Some(Self::Toml),
            "ini" => Some(Self::Ini),
            "env" => Some(Self::Env),
            "cfg" => Some(Self::Cfg),
            "nix" => Some(Self::Nix),
            _ => None,
        }
    }

    /// Checks if the given extenstion is supported by ParamGuard.
    ///
    /// # Examples
    /// ```
    /// use paramguard_core::config::types::ConfigFormat;
    ///
    /// assert!(ConfigFormat::is_valid_extension("json"));
    /// assert!(!ConfigFormat::is_valid_extension("txt"));
    /// ```
    pub fn is_valid_extension(ext: &str) -> bool {
        Self::from_extension(ext).is_some()
    }

    /// Returns the default content for a new configuration file of this format.
    ///
    /// # Examples
    /// ```
    /// use paramguard_core::config::types::ConfigFormat;
    ///
    /// assert_eq!(ConfigFormat::Json.get_default_content(), "{}");
    /// assert_eq!(ConfigFormat::Yaml.get_default_content(), "---");
    /// ```
    pub fn get_default_content(&self) -> &'static str {
        match self {
            ConfigFormat::Json => "{}",
            ConfigFormat::Yaml => "---",
            ConfigFormat::Toml | ConfigFormat::Ini | ConfigFormat::Env => "",
            ConfigFormat::Cfg => "# Configuration File",
            ConfigFormat::Nix => "{ }",
        }
    }
    /// Returns a list of all supported file extensions.
    ///
    /// # Examples
    /// ```
    /// use paramguard_core::config::types::ConfigFormat;
    ///
    /// let extensions = ConfigFormat::supported_extensions();
    /// assert!(extensions.contains(&"json"));
    /// assert!(extensions.contains(&"yaml"));
    /// ```
    pub fn supported_extensions() -> Vec<&'static str> {
        vec!["json", "yaml", "yml", "toml", "ini", "env", "cfg", "nix"]
    }
}
