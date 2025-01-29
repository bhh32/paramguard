//! Error types for configuration management.
//!
//! This module provides detailed error types for all configuration-related
//! operations in ParamGuard.

use thiserror::Error;

/// Errors that can occur during configuration operations.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// Returned when there is an error reading from or writing to a file.
    ///
    /// # Examples
    /// - File doesn't exist
    /// - Permission denied
    /// - Directory not found
    #[error("Failed to read config file: {0}")]
    ReadError(#[from] std::io::Error),

    /// Returned when a configuration file's format is invalid or unsupported.
    ///
    /// # Examples
    /// - Unsupported file extension
    /// - Missing file extension
    /// - File type mismatch
    #[error("Invalid config format: {0}")]
    InvalidFormat(String),

    /// Returned when parsing a configuratioin file fails.
    ///
    /// # Examples
    /// - Invalid JSON syntax
    /// - Malformed YAML
    /// - Invalid TOML structure
    /// - Invalid INI format
    /// - Invalid environment variable format
    #[error("Failed to parse config: {0}")]
    ParseError(String),

    /// Returned when configuration validation fails.
    ///
    /// # Examples
    /// - Schema validation failure
    /// - Required fields missing
    /// - Invlaid value types
    #[error("Config validation failed: {0}")]
    ValidationError(String),

    /// Returned when attempting to create a configuration file that already exists.
    ///
    /// This error helps prevent accidental overwrites of existing configurations.
    #[error("Config '{0}' already exists")]
    ConfigExists(String),

    /// Returned when attempting to access a configuration that doesn't exist.
    ///
    /// This can occur during update, delete, or retrieval operations.
    #[error("Config '{0}' not found")]
    ConfigNotFound(String),

    /// Returned when there are permission issues accessing a configuration.
    ///
    /// # Examples
    /// - No write permission
    /// - No read permission
    /// - No execute permission on parent directory
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
}

impl ConfigError {
    /// Returns true if the error is related to file access permissions.
    ///
    /// # Examples
    /// ```
    /// use paramguard_core::config::error::ConfigError;
    ///
    /// let error = ConfigError::PermissionDenied("No write access".to_string());
    /// assert!(error.is_permission_error());
    /// ```
    pub fn is_permission_error(&self) -> bool {
        matches!(self, ConfigError::PermissionDenied(_))
            || matches!(self, ConfigError::ReadError(e) if e.kind() == std::io::ErrorKind::PermissionDenied)
    }

    /// Returns true if the error is related to a missing configuration.
    ///
    /// # Examples
    /// ```
    /// use paramguard_core::config::error::ConfigError;
    ///
    /// let error = ConfigError::ConfigNotFound("config.json".to_string());
    /// assert!(error.is_not_found_error());
    /// ```
    pub fn is_not_found_error(&self) -> bool {
        matches!(self, ConfigError::ConfigNotFound(_))
            || matches!(self, ConfigError::ReadError(e) if e.kind() == std::io::ErrorKind::NotFound)
    }

    /// Returns true if the error is related to invalid format or parsing.
    ///
    /// # Examples
    /// ```
    /// use paramguard_core::config::error::ConfigError;
    ///
    /// let error = ConfigError::ParseError("Invalid JSON".to_string());
    /// assert!(error.is_format_error());
    /// ```
    pub fn is_format_error(&self) -> bool {
        matches!(
            self,
            ConfigError::InvalidFormat(_) | ConfigError::ParseError(_)
        )
    }

    /// Returns a user-friendly error message for display purposes.
    ///
    /// # Examples
    /// ```
    /// use paramguard_core::config::error::ConfigError;
    ///
    /// let error = ConfigError::ConfigNotFound("settings.json".to_string());
    /// assert_eq!(
    ///     error.user_friendly_message(),
    ///     "Configuration file 'settings.json' could not be found"
    /// );
    /// ```
    pub fn user_friendly_message(&self) -> String {
        match self {
            ConfigError::ReadError(e) => format!("Unable to access configuration file: {}", e),
            ConfigError::InvalidFormat(msg) => {
                format!("Configuration format is not valid: {}", msg)
            }
            ConfigError::ParseError(msg) => format!("Configuration content is not valid: {}", msg),
            ConfigError::ValidationError(msg) => {
                format!("Configuration validation failed: {}", msg)
            }
            ConfigError::ConfigExists(name) => format!("Configuration '{}' already exists", name),
            ConfigError::ConfigNotFound(name) => {
                format!("Configuration file '{}' could not be found", name)
            }
            ConfigError::PermissionDenied(msg) => format!("Permission denied: {}", msg),
        }
    }
}
