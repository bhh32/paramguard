//! Configuration management module for ParamGuard.
//!
//! This module provides the core functionality for managing configuration files,
//! including:
//! - File creation and deletion
//! - Content validation
//! - Format detection
//! - Content updates
//!
//! # Examples
//!
//! ```no_run
//! use paramguard_core::config::{manager::ConfigManager, types::ConfigFormat};
//! use std::path::Path;
//!
//! // Create a new configuration manager
//! let mut manager = ConfigManager::new();
//!
//! // Create a new JSON configuration
//! manager.create_config_file(
//!     "settings",
//!     Path::new("settings.json"),
//!     ConfigFormat::Json,
//!     Some(r#"{"setting": "value"}"#)
//! ).unwrap();
//!
//! // Add an existing configuration file
//! manager.add_config_file(Path::new("existing.yaml")).unwrap();
//!
//! // Update a configuration
//! manager.update_config("settings", r#"{"updated": true}"#).unwrap();
//!
//! // List all configurations
//! for config in manager.list_configs() {
//!     println!("Config: {}", config.name);
//! }
//! ```

use super::{error::*, types::*};
use chrono::Utc;
use std::{collections::HashMap, fs, path::Path};

/// Manages configuration files for ParamGuard.
///
/// The ConfigManager is responsible for:
/// - Adding existing configuration files to be managed
/// - Creating new configuration files
/// - Updating configuration content
/// - Deleting configurations
/// - Validating configuration formats
///
/// # Examples
/// ```no_run
/// use paramguard_core::config::{manager::ConfigManager, types::ConfigFormat};
/// use std::path::Path;
///
/// let mut manager = ConfigManager::new();
///
/// // Create a new JSON config
/// manager.create_config_file(
///     "settings",
///     Path::new("settings.json"),
///     ConfigFormat::Json,
///     Some(r#"{"setting": "value"}"#)
/// ).unwrap();
///
/// // Add an existing config file
/// manager.add_config_file(Path::new("existing-config.yaml")).unwrap();
/// ```
pub struct ConfigManager {
    configs: HashMap<String, ConfigFile>,
}

impl ConfigManager {
    /// Creates a new ConfigManager instance.
    ///
    /// # Examples
    /// ```
    /// use paramguard_core::config::manager::ConfigManager;
    ///
    /// let manager = ConfigManager::new();
    /// ```
    pub fn new() -> Self {
        Self {
            configs: HashMap::new(),
        }
    }

    /// Adds an existing configuration file to be managed by ParamGuard.
    ///
    /// This function will:
    /// - Verify the file exists
    /// - Detect and validate the file format
    /// - Read and validate the file content
    /// - Add the file to the managed configurations
    ///
    /// # Arguments
    /// * `path` - Path to the existing configuration file
    ///
    /// # Returns
    /// * `Ok(())` if the file was successfully added
    /// * `Err(ConfigError)` if the file doesn't exist or is invalid
    ///
    /// # Examples
    /// ```no_run
    /// use paramguard_core::config::manager::ConfigManager;
    /// use std::path::Path;
    ///
    /// let mut manager = ConfigManager::new();
    /// manager.add_config_file(Path::new("config.json")).unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns error if:
    /// - File doesn't exist
    /// - File format is not supported
    /// - File content is invalid for its format
    pub fn add_config_file(&mut self, path: &Path) -> Result<(), ConfigError> {
        // Validate the config file exists
        if !path.exists() {
            return Err(ConfigError::ConfigNotFound(
                path.to_string_lossy().to_string(),
            ));
        }

        let format = Self::detect_format(path)?;
        let content = fs::read_to_string(path).map_err(|e| {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                ConfigError::PermissionDenied(format!(
                    "Cannot read file: {}",
                    path.to_string_lossy()
                ))
            } else {
                ConfigError::ReadError(e)
            }
        })?;

        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| {
                ConfigError::InvalidFormat(format!("Invalid filename: {}", path.to_string_lossy()))
            })?
            .to_string();

        if self.exists(&name) {
            return Err(ConfigError::ConfigExists(name));
        }

        // Validate the file is a valid config file
        let config = ConfigFile {
            name: name.clone(),
            path: path.to_path_buf(),
            format,
            content: content.clone(),
            last_modified: Utc::now(),
        };

        self.validate_format(&config)?;
        self.configs.insert(name, config);

        Ok(())
    }

    /// Creates a new configuration file and adds it to be managed by ParamGuard.
    ///
    /// This function will:
    /// - Create the file with specified format
    /// - Initialize it with provided or default content
    /// - Validate the content
    /// - Add it to managed configurations
    ///
    /// # Arguments
    /// * `name` - Name for the new configuration
    /// * `path` - Path where the new file should be created
    /// * `format` - Format of the new configuration file
    /// * `init_content` - Optional initial content (uses format default if None)
    ///
    /// # Returns
    /// * `Ok(())` if the file was successfully created
    /// * `Err(ConfigError)` if creation failed or file already exists
    ///
    /// # Examples
    /// ```no_run
    /// use paramguard_core::config::{manager::ConfigManager, types::ConfigFormat};
    /// use std::path::Path;
    ///
    /// let mut manager = ConfigManager::new();
    ///
    /// // Create with custom content
    /// manager.create_config_file(
    ///     "config",
    ///     Path::new("config.json"),
    ///     ConfigFormat::Json,
    ///     Some(r#"{"key": "value"}"#)
    /// ).unwrap();
    ///
    /// // Create with default content
    /// manager.create_config_file(
    ///     "empty",
    ///     Path::new("empty.yaml"),
    ///     ConfigFormat::Yaml,
    ///     None
    /// ).unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns error if:
    /// - File already exists
    /// - Parent directory creation fails
    /// - Content validation fails
    /// - File creation fails
    pub fn create_config_file(
        &mut self,
        name: &str,
        path: &Path,
        format: ConfigFormat,
        init_content: Option<&str>,
    ) -> Result<(), ConfigError> {
        // Check if config already exists in manager
        if self.exists(name) {
            return Err(ConfigError::ConfigExists(name.to_string()));
        }

        // Check if the file already exists on disk
        if path.exists() {
            return Err(ConfigError::ConfigExists(format!(
                "File already exists at: {}",
                path.to_string_lossy()
            )));
        }

        // Create default content based on format if none was provided
        let content = match init_content {
            Some(content) => {
                if content.is_empty() {
                    format.get_default_content().to_string()
                } else {
                    content.to_string()
                }
            }
            None => format.get_default_content().to_string(),
        };

        let config = ConfigFile {
            name: name.to_string(),
            path: path.to_path_buf(),
            format,
            content: content.clone(),
            last_modified: Utc::now(),
        };

        // Validate content before creating file
        self.validate_format(&config)?;

        // Create parent directories if needed
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                if e.kind() == std::io::ErrorKind::PermissionDenied {
                    ConfigError::PermissionDenied(format!(
                        "Cannot create directory: {}",
                        parent.to_string_lossy()
                    ))
                } else {
                    ConfigError::ReadError(e)
                }
            })?;
        }

        // Write the file
        fs::write(path, &content).map_err(|e| {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                ConfigError::PermissionDenied(format!(
                    "Cannot write to file: {}",
                    path.to_string_lossy()
                ))
            } else {
                ConfigError::ReadError(e)
            }
        })?;

        // Add to managed configs
        self.configs.insert(name.to_string(), config);

        Ok(())
    }

    pub fn get_config(&self, name: &str) -> Option<&ConfigFile> {
        self.configs.get(name)
    }

    /// Updates the content of an existing configuration file.
    ///
    /// This function will:
    /// - Validate the new content
    /// - Update the file on disk
    /// - Update the in-memory representation
    ///
    /// # Arguments
    /// * `name` - Name of the configuration to update
    /// * `content` - New content for the configuration file
    ///
    /// # Returns
    /// * `Ok(())` if the update was successful
    /// * `Err(ConfigError)` if the update failed
    ///
    /// # Examples
    /// ```no_run
    /// use paramguard_core::config::manager::ConfigManager;
    ///
    /// let mut manager = ConfigManager::new();
    /// manager.update_config("config", r#"{"updated": true}"#).unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns error if:
    /// - Configuration doesn't exist
    /// - New content is invalid
    /// - File update fails
    pub fn update_config(&mut self, name: &str, content: &str) -> Result<(), ConfigError> {
        // Get existing config, using proper ConfigNotFound error
        let config = self
            .configs
            .get(name)
            .ok_or_else(|| ConfigError::ConfigNotFound(name.to_string()))?;

        let config_path = config.path.clone();
        let config_format = config.format.clone();

        // Create temporary config to validate new content
        let temp_config = ConfigFile {
            name: name.to_string(),
            path: config_path.clone(),
            format: config_format.clone(),
            content: content.to_string(),
            last_modified: Utc::now(),
        };

        // Validate new content format
        self.validate_format(&temp_config).map_err(|e| {
            if e.is_format_error() {
                ConfigError::ValidationError(format!(
                    "New content for '{}' is not valid {}: {}",
                    name,
                    config_format.as_extension().to_uppercase(),
                    e
                ))
            } else {
                e
            }
        })?;

        // Verify file still exists
        if !config_path.exists() {
            return Err(ConfigError::ConfigNotFound(format!(
                "Config file no longer exists at: {}",
                config_path.to_string_lossy()
            )));
        }

        // Update file on disk
        fs::write(&config_path, content).map_err(|e| {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                ConfigError::PermissionDenied(format!(
                    "Cannot write to config file: {}",
                    config_path.to_string_lossy()
                ))
            } else {
                ConfigError::ReadError(e)
            }
        })?;

        // Update in memory
        if let Some(config) = self.configs.get_mut(name) {
            config.content = content.to_string();
            config.last_modified = Utc::now();
        }

        Ok(())
    }

    /// Deletes a configuration file.
    ///
    /// This function will:
    /// - Remove the configuration from managed configs
    /// - Delete the file from disk
    ///
    /// # Arguments
    /// * `name` - Name of the configuration to delete
    ///
    /// # Returns
    /// * `Ok(())` if deletion was successful
    /// * `Err(ConfigError)` if deletion failed
    ///
    /// # Examples
    /// ```no_run
    /// use paramguard_core::config::manager::ConfigManager;
    ///
    /// let mut manager = ConfigManager::new();
    /// manager.delete_config("old_config").unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns error if:
    /// - Configuration doesn't exist
    /// - File deletion fails
    pub fn delete_config(&mut self, name: &str) -> Result<(), ConfigError> {
        // Remove from managed configs first, using proper ConfigNotFound error
        let config = self
            .configs
            .remove(name)
            .ok_or_else(|| ConfigError::ConfigNotFound(name.to_string()))?;

        // Check if file exists before attempting deletion
        if !config.path.exists() {
            return Ok(()); // File already gone, consider it a success
        }

        // Delete file from disk
        fs::remove_file(&config.path).map_err(|e| {
            // Put config back in managed configs if file deletion fails
            self.configs.insert(name.to_string(), config.clone());

            match e.kind() {
                std::io::ErrorKind::PermissionDenied => ConfigError::PermissionDenied(format!(
                    "Cannot delete config file: {}",
                    config.path.to_string_lossy()
                )),
                _ => ConfigError::ReadError(e),
            }
        })?;

        Ok(())
    }

    /// Validates the format and content of a configuration file.
    ///
    /// This function performs format-specific validation:
    /// - JSON: Validates JSON syntax
    /// - YAML: Validates YAML syntax
    /// - TOML: Validates TOML syntax
    /// - INI: Validates section headers and key-value pairs
    /// - ENV: Validates environment variable format
    ///
    /// # Arguments
    /// * `config` - The configuration file to validate
    ///
    /// # Returns
    /// * `Ok(())` if validation succeeds
    /// * `Err(ConfigError)` if validation fails
    ///
    /// # Examples
    /// ```no_run
    /// use paramguard_core::config::{types::*, manager::ConfigManager};
    /// use std::path::PathBuf;
    /// use chrono::Utc;
    ///
    /// let manager = ConfigManager::new();
    /// let config = ConfigFile {
    ///     name: "test".to_string(),
    ///     path: PathBuf::from("test.json"),
    ///     format: ConfigFormat::Json,
    ///     content: r#"{"valid": "json"}"#.to_string(),
    ///     last_modified: Utc::now(),
    /// };
    ///
    /// manager.validate_format(&config).unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns error if:
    /// - Content doesn't match the specified format
    /// - Syntax is invalid for the format
    pub fn validate_format(&self, config: &ConfigFile) -> Result<(), ConfigError> {
        let format_name = config.format.as_extension().to_uppercase();
        match config.format {
            ConfigFormat::Json => {
                serde_json::from_str::<serde_json::Value>(&config.content).map_err(|e| {
                    ConfigError::ParseError(format!(
                        "Invalid JSON: {} in file '{}'",
                        e,
                        config.path.to_string_lossy()
                    ))
                })?;
            }
            ConfigFormat::Yaml => {
                serde_yaml_ng::from_str::<serde_yaml_ng::Value>(&config.content).map_err(|e| {
                    ConfigError::ParseError(format!(
                        "Invalid YAML: {} in file '{}'",
                        e,
                        config.path.to_string_lossy()
                    ))
                })?;
            }
            ConfigFormat::Toml => {
                toml::from_str::<toml::Value>(&config.content).map_err(|e| {
                    ConfigError::ParseError(format!(
                        "Invalid TOML: {} in file '{}'",
                        e,
                        config.path.to_string_lossy()
                    ))
                })?;
            }
            ConfigFormat::Ini | ConfigFormat::Cfg => {
                let mut in_section = false;
                for (line_num, line) in config.content.lines().enumerate() {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
                        continue;
                    }

                    if line.starts_with('[') {
                        if !line.ends_with(']') {
                            return Err(ConfigError::ParseError(format!(
                                "Invalid {}: Unclosed section header on line {} in file '{}'",
                                format_name,
                                line_num + 1,
                                config.path.to_string_lossy()
                            )));
                        }
                        in_section = true;
                        continue;
                    }

                    if !line.contains('=') {
                        return Err(ConfigError::ParseError(format!(
                            "Invalid {}: Line {} missing '=' in file '{}': '{}'",
                            format_name,
                            line_num + 1,
                            config.path.to_string_lossy(),
                            line
                        )));
                    }

                    // Validate key format
                    let key = line.split('=').next().unwrap().trim();
                    if key.is_empty() {
                        return Err(ConfigError::ParseError(format!(
                            "Invalid {}: Empty key on line {} in file '{}'",
                            format_name,
                            line_num + 1,
                            config.path.to_string_lossy()
                        )));
                    }
                }
            }
            ConfigFormat::Env => {
                for (line_num, line) in config.content.lines().enumerate() {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with('#') {
                        continue;
                    }

                    if !line.contains('=') {
                        return Err(ConfigError::ParseError(format!(
                            "Invalid ENV: Line {} missing '=' in file '{}': '{}'",
                            line_num + 1,
                            config.path.to_string_lossy(),
                            line
                        )));
                    }

                    // Validate environment variable name format
                    let key = line.split('=').next().unwrap().trim();
                    if key.is_empty() {
                        return Err(ConfigError::ParseError(format!(
                            "Invalid ENV: Empty variable name on line {} in file '{}'",
                            line_num + 1,
                            config.path.to_string_lossy()
                        )));
                    }

                    // Check for valid environment variable name (alphanumeric and underscore)
                    if !key.chars().all(|c| c.is_alphanumeric() || c == '_') {
                        return Err(ConfigError::ParseError(format!(
                            "Invalid ENV: Invalid variable name '{}' on line {} in file '{}' \
                            (must contain only letters, numbers, and underscores)",
                            key,
                            line_num + 1,
                            config.path.to_string_lossy()
                        )));
                    }
                }
            }
            ConfigFormat::Nix => {
                let mut context_stack = Vec::new();
                let mut in_string = false;
                let mut string_delimiter = '"';

                // Keep track of assignments on the current line
                let mut current_line_assignments = Vec::new();
                let mut current_line_start = 0;

                let content_chars: Vec<char> = config.content.chars().collect();
                let mut i = 0;

                while i < content_chars.len() {
                    let c = content_chars[i];

                    // Track line changes
                    if c == '\n' {
                        // Check assignments on the previous line
                        if current_line_assignments.len() > 1 {
                            // Get the content of this line
                            let line_content: String =
                                content_chars[current_line_start..i].iter().collect();

                            // For multiple assignments on one line, each must end with a semicolon
                            for &pos in
                                &current_line_assignments[..current_line_assignments.len() - 1]
                            {
                                let after_pos = &line_content[pos..];
                                if !after_pos.contains(';') {
                                    return Err(ConfigError::ParseError(
                                        "Missing semicolon between assignments on the same line"
                                            .to_string(),
                                    ));
                                }
                            }

                            // Last assignment needs a semicolon if it's not followed by a block
                            let last_pos = *current_line_assignments.last().unwrap();
                            let after_last = &line_content[last_pos..];
                            if !after_last.contains(';')
                                && !after_last.contains('{')
                                && !after_last.contains('}')
                            {
                                return Err(ConfigError::ParseError(
                                    "Missing semicolon after assignment".to_string(),
                                ));
                            }
                        }

                        current_line_assignments.clear();
                        current_line_start = i + 1;
                    }

                    // Handle string literals
                    if (c == '"' || c == '\'') && (!in_string || c == string_delimiter) {
                        if in_string && i > 0 && content_chars[i - 1] == '\\' {
                            i += 1;
                            continue;
                        }
                        if !in_string {
                            string_delimiter = c;
                        }
                        in_string = !in_string;
                        i += 1;
                        continue;
                    }

                    if in_string {
                        i += 1;
                        continue;
                    }

                    // Skip comments
                    if c == '#' {
                        while i < content_chars.len() && content_chars[i] != '\n' {
                            i += 1;
                        }
                        continue;
                    }

                    match c {
                        '{' => {
                            context_stack.push(('{', i));
                        }
                        '}' => {
                            if context_stack.is_empty() {
                                return Err(ConfigError::ParseError(
                                    "Unexpected closing brace".to_string(),
                                ));
                            }

                            let (_, open_pos) = context_stack.pop().unwrap();

                            // If this brace closes an attribute set that's used as a value,
                            // it needs to be followed by a semicolon
                            if open_pos > 0 {
                                let before_open: String =
                                    content_chars[open_pos - 1..open_pos].iter().collect();
                                if before_open.trim() == "=" {
                                    // Look ahead for a semicolon
                                    let mut found_semicolon = false;
                                    let mut j = i + 1;
                                    while j < content_chars.len()
                                        && content_chars[j].is_whitespace()
                                    {
                                        j += 1;
                                    }
                                    if j < content_chars.len() && content_chars[j] == ';' {
                                        found_semicolon = true;
                                    }

                                    if !found_semicolon {
                                        return Err(ConfigError::ParseError(
                                            "Missing semicolon after closing brace of attribute set value".to_string()
                                        ));
                                    }
                                }
                            }
                        }
                        '=' => {
                            if !in_string && i > 0 && i < content_chars.len() - 1 {
                                // Make sure this is a real assignment
                                let prev = content_chars[i - 1];
                                let next = content_chars[i + 1];
                                if prev != '=' && next != '=' {
                                    current_line_assignments.push(i);
                                }
                            }
                        }
                        _ => {}
                    }

                    i += 1;
                }

                // Check unclosed structures
                if !context_stack.is_empty() {
                    return Err(ConfigError::ParseError(
                        "Unclosed braces in configuration".to_string(),
                    ));
                }

                if in_string {
                    return Err(ConfigError::ParseError(
                        "Unterminated string literal".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }

    /// Lists all managed configuration files.
    ///
    /// # Returns
    /// A vector of references to all managed configuration files.
    ///
    /// # Examples
    /// ```
    /// use paramguard_core::config::manager::ConfigManager;
    ///
    /// let manager = ConfigManager::new();
    /// let configs = manager.list_configs();
    /// for config in configs {
    ///     println!("Config name: {}", config.name);
    /// }
    /// ```
    pub fn list_configs(&self) -> Vec<&ConfigFile> {
        self.configs.values().collect()
    }

    /// Checks if a configuration with the given name exists.
    ///
    /// # Arguments
    /// * `name` - Name of the configuration to check
    ///
    /// # Returns
    /// `true` if the configuration exists, `false` otherwise
    ///
    /// # Examples
    /// ```
    /// use paramguard_core::config::manager::ConfigManager;
    ///
    /// let manager = ConfigManager::new();
    /// if manager.exists("config") {
    ///     println!("Configuration exists!");
    /// }
    /// ```
    pub fn exists(&self, name: &str) -> bool {
        self.configs.contains_key(name)
    }

    /// Detects the configuration format from a file path.
    ///
    /// This function examines the file extension to determine the appropriate
    /// configuration format.
    ///
    /// # Arguments
    /// * `path` - Path to the configuration file
    ///
    /// # Returns
    /// * `Ok(ConfigFormat)` if a supported format is detected
    /// * `Err(ConfigError)` if the format is not supported
    ///
    /// # Examples
    /// ```no_run
    /// use paramguard_core::config::manager::ConfigManager;
    /// use std::path::Path;
    ///
    /// let format = ConfigManager::detect_format(Path::new("config.json")).unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns error if:
    /// - File has no extension
    /// - Extension is not supported
    pub fn detect_format(path: &Path) -> Result<ConfigFormat, ConfigError> {
        // Get filename for better error messages
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or_else(|| {
                ConfigError::InvalidFormat(format!(
                    "File '{}' has no extension. Supported extensions are: {}",
                    file_name,
                    ConfigFormat::supported_extensions().join(", ")
                ))
            })?;

        ConfigFormat::from_extension(extension).ok_or_else(|| {
            ConfigError::InvalidFormat(format!(
                "Unsupported file extension '.{}' for file '{}'. Supported extensions are: {}",
                extension,
                file_name,
                ConfigFormat::supported_extensions().join(", ")
            ))
        })
    }
}
