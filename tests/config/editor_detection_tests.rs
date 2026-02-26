use narsil_mcp::config::editor::{
    detect_available_editors, get_editor_config_path, EditorConfig, EditorType,
};
use std::path::PathBuf;

#[test]
fn test_claude_desktop_macos_path() {
    #[cfg(target_os = "macos")]
    {
        let path = get_editor_config_path(EditorType::ClaudeDesktop);
        assert!(path
            .to_string_lossy()
            .contains("Library/Application Support/Claude"));
        assert!(path
            .to_string_lossy()
            .ends_with("claude_desktop_config.json"));
    }
}

#[test]
fn test_claude_desktop_windows_path() {
    #[cfg(target_os = "windows")]
    {
        let path = get_editor_config_path(EditorType::ClaudeDesktop);
        assert!(path.to_string_lossy().contains("Claude"));
        assert!(path
            .to_string_lossy()
            .ends_with("claude_desktop_config.json"));
    }
}

#[test]
fn test_claude_code_path() {
    let path = get_editor_config_path(EditorType::ClaudeCode);
    assert!(path.to_string_lossy().contains(".claude"));
    assert!(path.to_string_lossy().ends_with("claude_code_config.json"));
}

#[test]
fn test_gemini_cli_path() {
    let path = get_editor_config_path(EditorType::GeminiCLI);
    assert!(path.to_string_lossy().contains(".gemini"));
    assert!(path.to_string_lossy().ends_with("settings.json"));
}

#[test]
fn test_zed_macos_path() {
    #[cfg(target_os = "macos")]
    {
        let path = get_editor_config_path(EditorType::Zed);
        assert!(path.to_string_lossy().contains(".config/zed"));
        assert!(path.to_string_lossy().ends_with("settings.json"));
    }
}

#[test]
fn test_zed_windows_path() {
    #[cfg(target_os = "windows")]
    {
        let path = get_editor_config_path(EditorType::Zed);
        assert!(path.to_string_lossy().contains("Zed"));
        assert!(path.to_string_lossy().ends_with("settings.json"));
    }
}

#[test]
fn test_vscode_workspace_path() {
    let current_dir = std::env::current_dir().unwrap();
    let path = get_editor_config_path(EditorType::VSCode);
    assert_eq!(path, current_dir.join(".vscode").join("mcp.json"));
}

#[test]
fn test_jetbrains_workspace_path() {
    let current_dir = std::env::current_dir().unwrap();
    let path = get_editor_config_path(EditorType::JetBrains);
    assert_eq!(path, current_dir.join(".idea").join("mcp.json"));
}

#[test]
fn test_editor_type_display() {
    assert_eq!(EditorType::ClaudeDesktop.to_string(), "Claude Desktop");
    assert_eq!(EditorType::ClaudeCode.to_string(), "Claude Code");
    assert_eq!(EditorType::GeminiCLI.to_string(), "Gemini CLI");
    assert_eq!(EditorType::Zed.to_string(), "Zed");
    assert_eq!(EditorType::VSCode.to_string(), "VS Code");
    assert_eq!(EditorType::JetBrains.to_string(), "JetBrains IDEs");
}

#[test]
fn test_detect_available_editors_none() {
    // In test environment, likely no editor configs exist
    let editors = detect_available_editors();
    // Should return empty or only editors with existing config files
    // This test will vary by environment
    assert!(editors.len() <= 6);
}

#[cfg(test)]
mod integration {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_detect_available_editors_with_files() {
        let temp = tempdir().unwrap();

        // Create mock config files
        let claude_config = temp.path().join("claude_desktop_config.json");
        fs::write(&claude_config, "{}").unwrap();

        // This test is aspirational - we need to make detect_available_editors
        // accept a custom search path for testing
        // For now, just verify the function doesn't panic
        let _ = detect_available_editors();
    }
}

#[test]
fn test_editor_config_struct() {
    let config = EditorConfig {
        editor_type: EditorType::ClaudeDesktop,
        config_path: PathBuf::from("/test/path/config.json"),
        exists: false,
    };

    assert_eq!(config.editor_type, EditorType::ClaudeDesktop);
    assert_eq!(config.config_path, PathBuf::from("/test/path/config.json"));
    assert!(!config.exists);
}
