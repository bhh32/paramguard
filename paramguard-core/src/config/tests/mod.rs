use crate::config::{manager::ConfigManager, types::ConfigFormat};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_add_config_file() {
    let (temp_dir, mut manager) = tmp_and_mgr();

    // Test JSON config with valid content
    let json_path = temp_dir.path().join("test.json");
    fs::write(&json_path, r#"{"key": "value"}"#).unwrap();
    assert!(manager.add_config_file(&json_path).is_ok());

    // Test YAML config with both .yaml and .yml extensions
    let yaml_path = temp_dir.path().join("test.yaml");
    let yml_path = temp_dir.path().join("test.yml");
    fs::write(&yaml_path, "key: value").unwrap();
    fs::write(&yml_path, "key: value").unwrap();
    assert!(manager.add_config_file(&yaml_path).is_ok());
    assert!(manager.add_config_file(&yml_path).is_ok());

    // Test TOML config
    let toml_path = temp_dir.path().join("test.toml");
    fs::write(&toml_path, r#"key = "value""#).unwrap();
    assert!(manager.add_config_file(&toml_path).is_ok());

    // Test INI config
    let ini_path = temp_dir.path().join("test.ini");
    fs::write(&ini_path, "[section]\nkey=value").unwrap();
    assert!(manager.add_config_file(&ini_path).is_ok());

    // Test ENV config
    let env_path = temp_dir.path().join("test.env");
    fs::write(&env_path, "KEY=value").unwrap();
    assert!(manager.add_config_file(&env_path).is_ok());

    // Test Nix config
    let nix_path = temp_dir.path().join("test.nix");
    fs::write(&nix_path, "{ key = \"value\"; }").unwrap();
    assert!(manager.add_config_file(&nix_path).is_ok());

    // Test invalid file extension
    let invalid_path = temp_dir.path().join("test.invalid");
    fs::write(&invalid_path, "invalid content").unwrap();
    assert!(manager.add_config_file(&invalid_path).is_err());

    // Test duplicate config name
    let duplicate_path = temp_dir.path().join("test.json");
    assert!(manager.add_config_file(&duplicate_path).is_err());

    // Test permission denied scenario
    // Note: This would need proper setup in a real environment

    // Test non-existent file
    let nonexistent_path = temp_dir.path().join("nonexistent.json");
    assert!(manager.add_config_file(&nonexistent_path).is_err());
}

#[test]
fn test_create_config_file() {
    let (temp_dir, mut manager) = tmp_and_mgr();

    // Test creating JSON config with custom content
    let json_path = temp_dir.path().join("new.json");
    assert!(manager
        .create_config_file(
            "new_json",
            &json_path,
            ConfigFormat::Json,
            Some(r#"{"key": "value"}"#)
        )
        .is_ok());

    // Test creating YAML with default content
    let yaml_path = temp_dir.path().join("new.yaml");
    assert!(manager
        .create_config_file("new_yaml", &yaml_path, ConfigFormat::Yaml, None)
        .is_ok());

    // Test creating empty string content (should use format default)
    let toml_path = temp_dir.path().join("new.toml");
    assert!(manager
        .create_config_file("new_toml", &toml_path, ConfigFormat::Toml, Some(""))
        .is_ok());

    // Test creating config with nested directory structure
    let nested_path = temp_dir.path().join("nested/config/new.json");
    assert!(manager
        .create_config_file(
            "nested_json",
            &nested_path,
            ConfigFormat::Json,
            Some(r#"{"nested": true}"#)
        )
        .is_ok());

    // Test creating existing config name
    assert!(manager
        .create_config_file("new_json", &json_path, ConfigFormat::Json, None)
        .is_err());

    // Test creating with invalid content format
    assert!(manager
        .create_config_file(
            "invalid_json",
            &temp_dir.path().join("invalid.json"),
            ConfigFormat::Json,
            Some("invalid json")
        )
        .is_err());
}

#[test]
fn test_update_config() {
    let (temp_dir, mut manager) = tmp_and_mgr();

    // Create and update JSON config
    let json_path = temp_dir.path().join("update.json");
    manager
        .create_config_file(
            "update_json",
            &json_path,
            ConfigFormat::Json,
            Some(r#"{"key": "value"}"#),
        )
        .unwrap();

    // Test successful update
    assert!(manager
        .update_config("update_json", r#"{"key": "new_value"}"#)
        .is_ok());

    // Test update with invalid content
    assert!(manager
        .update_config("update_json", "invalid json")
        .is_err());

    // Test update non-existent config
    assert!(manager
        .update_config("nonexistent", r#"{"key": "value"}"#)
        .is_err());

    // Test update after file deletion
    fs::remove_file(&json_path).unwrap();
    assert!(manager
        .update_config("update_json", r#"{"key": "value"}"#)
        .is_err());
}

#[test]
fn test_delete_config() {
    let (temp_dir, mut manager) = tmp_and_mgr();

    // Create configs for deletion testing
    let json_path = temp_dir.path().join("delete.json");
    manager
        .create_config_file("delete_json", &json_path, ConfigFormat::Json, None)
        .unwrap();

    // Test successful deletion
    assert!(manager.delete_config("delete_json").is_ok());
    assert!(!json_path.exists());

    // Test deleting non-existent config
    assert!(manager.delete_config("nonexistent").is_err());

    // Test deleting already deleted config
    assert!(manager.delete_config("delete_json").is_err());

    // Test deletion when file is already gone
    let yaml_path = temp_dir.path().join("delete.yaml");
    manager
        .create_config_file("delete_yaml", &yaml_path, ConfigFormat::Yaml, None)
        .unwrap();
    fs::remove_file(&yaml_path).unwrap();
    assert!(manager.delete_config("delete_yaml").is_ok());
}

#[test]
fn test_format_validation() {
    let (temp_dir, mut manager) = tmp_and_mgr();

    // Test cases for all supported formats
    let test_cases = vec![
        // JSON format tests
        (ConfigFormat::Json, r#"{"key": "value"}"#, true),
        (ConfigFormat::Json, "invalid json", false),
        (ConfigFormat::Json, r#"{"nested": {"key": "value"}}"#, true),
        // YAML format tests
        (ConfigFormat::Yaml, "key: value", true),
        (ConfigFormat::Yaml, "invalid: : yaml", false),
        (ConfigFormat::Yaml, "nested:\n  key: value", true),
        // TOML format tests
        (ConfigFormat::Toml, r#"key = "value""#, true),
        (ConfigFormat::Toml, "invalid toml", false),
        (ConfigFormat::Toml, "[section]\nkey = \"value\"", true),
        // INI format tests
        (ConfigFormat::Ini, "[section]\nkey=value", true),
        (ConfigFormat::Ini, "invalid ini", false),
        (ConfigFormat::Ini, "key=value", true),
        // ENV format tests
        (ConfigFormat::Env, "KEY=value", true),
        (ConfigFormat::Env, "invalid env", false),
        (ConfigFormat::Env, "COMPLEX_KEY=complex value", true),
        // CFG format tests
        (ConfigFormat::Cfg, "[section]\nkey=value", true),
        (ConfigFormat::Cfg, "key=value", true),
        (ConfigFormat::Cfg, "invalid cfg", false),
        // Nix format tests
        // Basic syntax
        (ConfigFormat::Nix, "{ key = \"value\"; }", true),
        (
            ConfigFormat::Nix,
            "{ key = \"value\"\n  other = 123\n}",
            true,
        ), // No semicolons needed on separate lines
        // Function arguments
        (
            ConfigFormat::Nix,
            "{ config, pkgs, ... }:\n{\n  nixpkgs.config.allowUnfree = true;\n}",
            true,
        ),
        // Lists
        (ConfigFormat::Nix, "{ ports = [ 80 443 ]; }", true),
        (
            ConfigFormat::Nix,
            "{ users = [ \"alice\" \"bob\" ]; }",
            true,
        ),
        // With expressions
        (
            ConfigFormat::Nix,
            "{ packages = with pkgs; [ firefox git ]; }",
            true,
        ),
        // Nested structures
        (
            ConfigFormat::Nix,
            "{ networking = { hostName = \"mycomputer\"; }; }",
            true,
        ),
        // Comments
        (
            ConfigFormat::Nix,
            "# Header comment\n{ # inline comment\n  key = \"value\" # end comment\n}",
            true,
        ),
        (
            ConfigFormat::Nix,
            "/* Multi-line\n   comment */\n{ key = \"value\"; }",
            true,
        ),
        // Invalid cases
        (ConfigFormat::Nix, "{ unmatched = \"brace\"; ", false), // Unmatched brace
        (ConfigFormat::Nix, "{ key = value", false),             // Missing closing brace
        (ConfigFormat::Nix, "{ ports = [ 80 443 ", false),       // Unmatched bracket
        (ConfigFormat::Nix, "{ key = \"value\"; other = 123 }", true),
    ];

    for (idx, (format, content, should_pass)) in test_cases.iter().enumerate() {
        let path = temp_dir
            .path()
            .join(format!("test_{}.{}", idx, format.as_extension()));
        let result = manager.create_config_file(
            &format!("test_{}", idx),
            &path,
            format.clone(),
            Some(content),
        );

        assert_eq!(
            result.is_ok(),
            *should_pass,
            "Failed for format {:?} with content: {}",
            format,
            content
        );
    }
}

// Helper functions
fn tmp_and_mgr() -> (TempDir, ConfigManager) {
    (TempDir::new().unwrap(), ConfigManager::new())
}
