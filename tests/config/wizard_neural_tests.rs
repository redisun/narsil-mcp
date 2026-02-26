use narsil_mcp::config::wizard::{ApiProvider, NeuralWizard};
use serde_json::json;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_api_provider_from_string() {
    assert_eq!(ApiProvider::parse("voyage"), Some(ApiProvider::Voyage));
    assert_eq!(ApiProvider::parse("openai"), Some(ApiProvider::OpenAI));
    assert_eq!(ApiProvider::parse("custom"), Some(ApiProvider::Custom));
    assert_eq!(ApiProvider::parse("invalid"), None);
}

#[test]
fn test_api_provider_env_var_name() {
    assert_eq!(ApiProvider::Voyage.env_var_name(), "VOYAGE_API_KEY");
    assert_eq!(ApiProvider::OpenAI.env_var_name(), "OPENAI_API_KEY");
    assert_eq!(ApiProvider::Custom.env_var_name(), "EMBEDDING_API_KEY");
}

#[test]
fn test_validate_api_key_format() {
    // Voyage keys start with "pa-"
    assert!(NeuralWizard::validate_key_format(
        "pa-abc123xyz",
        ApiProvider::Voyage
    ));
    assert!(!NeuralWizard::validate_key_format(
        "invalid",
        ApiProvider::Voyage
    ));

    // OpenAI keys start with "sk-"
    assert!(NeuralWizard::validate_key_format(
        "sk-abc123xyz",
        ApiProvider::OpenAI
    ));
    assert!(!NeuralWizard::validate_key_format(
        "invalid",
        ApiProvider::OpenAI
    ));

    // Custom accepts anything non-empty
    assert!(NeuralWizard::validate_key_format(
        "anything",
        ApiProvider::Custom
    ));
    assert!(!NeuralWizard::validate_key_format("", ApiProvider::Custom));
}

#[tokio::test]
async fn test_add_api_key_to_claude_desktop_config_new_file() {
    let temp = tempdir().unwrap();
    let config_path = temp.path().join("claude_desktop_config.json");

    let wizard = NeuralWizard::new();
    wizard
        .add_to_editor_config(&config_path, "VOYAGE_API_KEY", "pa-test123")
        .await
        .unwrap();

    // Read the file and verify structure
    let content = fs::read_to_string(&config_path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();

    assert_eq!(
        parsed["mcpServers"]["narsil-mcp"]["env"]["VOYAGE_API_KEY"],
        "pa-test123"
    );
}

#[tokio::test]
async fn test_add_api_key_to_claude_desktop_config_existing_server() {
    let temp = tempdir().unwrap();
    let config_path = temp.path().join("claude_desktop_config.json");

    // Create existing config
    let existing = json!({
        "mcpServers": {
            "narsil-mcp": {
                "command": "narsil-mcp",
                "args": ["--repos", "~/code"]
            }
        }
    });
    fs::write(
        &config_path,
        serde_json::to_string_pretty(&existing).unwrap(),
    )
    .unwrap();

    let wizard = NeuralWizard::new();
    wizard
        .add_to_editor_config(&config_path, "VOYAGE_API_KEY", "pa-test123")
        .await
        .unwrap();

    // Verify it added env without removing existing args
    let content = fs::read_to_string(&config_path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();

    assert_eq!(parsed["mcpServers"]["narsil-mcp"]["command"], "narsil-mcp");
    assert_eq!(parsed["mcpServers"]["narsil-mcp"]["args"][0], "--repos");
    assert_eq!(
        parsed["mcpServers"]["narsil-mcp"]["env"]["VOYAGE_API_KEY"],
        "pa-test123"
    );
}

#[tokio::test]
async fn test_add_api_key_to_zed_config_new_file() {
    let temp = tempdir().unwrap();
    let config_path = temp.path().join("settings.json");

    let wizard = NeuralWizard::new();
    wizard
        .add_to_editor_config(&config_path, "VOYAGE_API_KEY", "pa-test123")
        .await
        .unwrap();

    let content = fs::read_to_string(&config_path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();

    assert_eq!(
        parsed["context_servers"]["narsil-mcp"]["env"]["VOYAGE_API_KEY"],
        "pa-test123"
    );
}

#[tokio::test]
async fn test_add_api_key_to_vscode_config_new_file() {
    let temp = tempdir().unwrap();
    let config_path = temp.path().join("mcp.json");

    let wizard = NeuralWizard::new();
    wizard
        .add_to_editor_config(&config_path, "VOYAGE_API_KEY", "pa-test123")
        .await
        .unwrap();

    let content = fs::read_to_string(&config_path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();

    assert_eq!(
        parsed["servers"]["narsil-mcp"]["env"]["VOYAGE_API_KEY"],
        "pa-test123"
    );
}

#[tokio::test]
async fn test_add_api_key_to_gemini_cli_config_new_file() {
    let temp = tempdir().unwrap();
    let config_dir = temp.path().join(".gemini");
    fs::create_dir(&config_dir).unwrap();
    let config_path = config_dir.join("settings.json");

    let wizard = NeuralWizard::new();
    wizard
        .add_to_editor_config(&config_path, "VOYAGE_API_KEY", "pa-test123")
        .await
        .unwrap();

    let content = fs::read_to_string(&config_path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();

    assert_eq!(
        parsed["mcpServers"]["narsil-mcp"]["env"]["VOYAGE_API_KEY"],
        "pa-test123"
    );
}

#[tokio::test]
async fn test_add_api_key_updates_existing_env() {
    let temp = tempdir().unwrap();
    let config_path = temp.path().join("claude_desktop_config.json");

    // Create config with existing env var
    let existing = json!({
        "mcpServers": {
            "narsil-mcp": {
                "command": "narsil-mcp",
                "env": {
                    "OPENAI_API_KEY": "sk-old123"
                }
            }
        }
    });
    fs::write(
        &config_path,
        serde_json::to_string_pretty(&existing).unwrap(),
    )
    .unwrap();

    let wizard = NeuralWizard::new();
    wizard
        .add_to_editor_config(&config_path, "VOYAGE_API_KEY", "pa-new123")
        .await
        .unwrap();

    let content = fs::read_to_string(&config_path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();

    // Both keys should exist
    assert_eq!(
        parsed["mcpServers"]["narsil-mcp"]["env"]["OPENAI_API_KEY"],
        "sk-old123"
    );
    assert_eq!(
        parsed["mcpServers"]["narsil-mcp"]["env"]["VOYAGE_API_KEY"],
        "pa-new123"
    );
}

#[tokio::test]
async fn test_wizard_handles_invalid_json() {
    let temp = tempdir().unwrap();
    let config_path = temp.path().join("invalid.json");

    // Write invalid JSON
    fs::write(&config_path, "{ invalid json }").unwrap();

    let wizard = NeuralWizard::new();
    let result = wizard
        .add_to_editor_config(&config_path, "VOYAGE_API_KEY", "pa-test123")
        .await;

    // Should return an error
    assert!(result.is_err());
}

#[test]
fn test_wizard_sanitizes_api_key() {
    // Should trim whitespace
    assert_eq!(NeuralWizard::sanitize_api_key("  pa-abc123  "), "pa-abc123");

    // Should remove quotes
    assert_eq!(NeuralWizard::sanitize_api_key("\"pa-abc123\""), "pa-abc123");

    // Should remove single quotes
    assert_eq!(NeuralWizard::sanitize_api_key("'pa-abc123'"), "pa-abc123");
}

#[tokio::test]
async fn test_wizard_creates_parent_directories() {
    let temp = tempdir().unwrap();
    let nested_path = temp
        .path()
        .join("a")
        .join("b")
        .join("c")
        .join("claude_desktop_config.json");

    let wizard = NeuralWizard::new();
    wizard
        .add_to_editor_config(&nested_path, "VOYAGE_API_KEY", "pa-test123")
        .await
        .unwrap();

    // Verify file was created and directories exist
    assert!(nested_path.exists());
    assert!(nested_path.parent().unwrap().exists());
}

#[test]
fn test_get_config_key_for_editor() {
    use narsil_mcp::config::editor::EditorType;

    assert_eq!(
        NeuralWizard::get_config_key_for_editor(EditorType::ClaudeDesktop),
        "mcpServers"
    );
    assert_eq!(
        NeuralWizard::get_config_key_for_editor(EditorType::ClaudeCode),
        "mcpServers"
    );
    assert_eq!(
        NeuralWizard::get_config_key_for_editor(EditorType::GeminiCLI),
        "mcpServers"
    );
    assert_eq!(
        NeuralWizard::get_config_key_for_editor(EditorType::Zed),
        "context_servers"
    );
    assert_eq!(
        NeuralWizard::get_config_key_for_editor(EditorType::VSCode),
        "servers"
    );
    assert_eq!(
        NeuralWizard::get_config_key_for_editor(EditorType::JetBrains),
        "servers"
    );
}
