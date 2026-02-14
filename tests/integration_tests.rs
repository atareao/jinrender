use std::fs;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_cli_basic_functionality() {
    let temp_dir = tempdir().unwrap();
    let template_path = temp_dir.path().join("test_template.jinja");
    let output_path = temp_dir.path().join("output.txt");

    // Create a simple template
    let template_content = "Hello, {{ env.USER | default('World') }}!";
    fs::write(&template_path, template_content).unwrap();

    // Run the jinrender command
    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "-j",
            template_path.to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute jinrender");

    // Check that the command succeeded
    if !output.status.success() {
        eprintln!(
            "Command failed with stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        panic!("Command failed");
    }

    // Check that output file was created and contains expected content
    assert!(output_path.exists());
    let result_content = fs::read_to_string(&output_path).unwrap();

    // Should contain "Hello, " and some username or "World"
    assert!(result_content.starts_with("Hello, "));
    assert!(result_content.ends_with("!"));
}

#[test]
fn test_cli_missing_template_file() {
    let temp_dir = tempdir().unwrap();
    let nonexistent_path = temp_dir.path().join("nonexistent.jinja");
    let output_path = temp_dir.path().join("output.txt");

    // Run the jinrender command with nonexistent template
    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "-j",
            nonexistent_path.to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute jinrender");

    // Command should fail
    assert!(!output.status.success());
}

#[test]
fn test_cli_complex_template() {
    let temp_dir = tempdir().unwrap();
    let template_path = temp_dir.path().join("complex_template.jinja");
    let output_path = temp_dir.path().join("complex_output.txt");

    // Create a more complex template
    let template_content = r#"System Information:
User: {{ env.USER | default('Unknown') }}
Home: {{ env.HOME | default('/unknown') }}
Path length: {{ env.PATH | length }}
Current directory: {{ env.PWD | default('Unknown') }}"#;

    fs::write(&template_path, template_content).unwrap();

    // Run the jinrender command
    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "-j",
            template_path.to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute jinrender");

    // Check that the command succeeded
    if !output.status.success() {
        eprintln!(
            "Command failed with stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        panic!("Command failed");
    }

    // Check output content
    let result_content = fs::read_to_string(&output_path).unwrap();

    // Should contain system information sections
    assert!(result_content.contains("System Information:"));
    assert!(result_content.contains("User:"));
    assert!(result_content.contains("Home:"));
    assert!(result_content.contains("Path length:"));
    assert!(result_content.contains("Current directory:"));
}

#[test]
fn test_cli_help_option() {
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .output()
        .expect("Failed to execute jinrender");

    // Command should succeed
    assert!(output.status.success());

    let help_text = String::from_utf8_lossy(&output.stdout);

    // Should contain help information
    assert!(help_text.contains("Render jinja templates"));
    assert!(help_text.contains("--jinja"));
    assert!(help_text.contains("--output"));
    assert!(help_text.contains("--help"));
}

#[test]
fn test_cli_version_option() {
    let output = Command::new("cargo")
        .args(["run", "--", "--version"])
        .output()
        .expect("Failed to execute jinrender");

    // Command should succeed
    assert!(output.status.success());

    let version_text = String::from_utf8_lossy(&output.stdout);

    // Should contain version information
    assert!(version_text.contains("0.1.1"));
}

#[test]
fn test_cli_svg_template() {
    let temp_dir = tempdir().unwrap();
    let template_path = temp_dir.path().join("cover_template.svg");
    let output_path = temp_dir.path().join("cover_output.svg");

    // Create an SVG template (similar to the use case mentioned in README)
    let svg_template = r##"<svg xmlns="http://www.w3.org/2000/svg" width="800" height="600">
  <rect width="800" height="600" fill="#2c3e50"/>
  <text x="400" y="200" font-family="Arial" font-size="48" fill="white" text-anchor="middle">
    {{ env.PODCAST_TITLE | default("My Podcast") }}
  </text>
  <text x="400" y="300" font-family="Arial" font-size="32" fill="#ecf0f1" text-anchor="middle">
    Episode {{ env.EPISODE_NUMBER | default("1") }}
  </text>
</svg>"##;

    fs::write(&template_path, svg_template).unwrap();

    // Set environment variables for the test
    std::env::set_var("PODCAST_TITLE", "Test Podcast");
    std::env::set_var("EPISODE_NUMBER", "42");

    // Run the jinrender command
    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "-j",
            template_path.to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute jinrender");

    // Check that the command succeeded
    if !output.status.success() {
        eprintln!(
            "Command failed with stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        panic!("Command failed");
    }

    // Check output content
    let result_content = fs::read_to_string(&output_path).unwrap();

    // Should contain the replaced values
    assert!(result_content.contains("Test Podcast"));
    assert!(result_content.contains("Episode 42"));
    assert!(result_content.contains("<svg"));
    assert!(result_content.contains("</svg>"));

    // Clean up environment variables
    std::env::remove_var("PODCAST_TITLE");
    std::env::remove_var("EPISODE_NUMBER");
}

#[test]
fn test_cli_missing_arguments() {
    // Test with missing jinja argument
    let output = Command::new("cargo")
        .args(["run", "--", "-o", "output.txt"])
        .output()
        .expect("Failed to execute jinrender");

    assert!(!output.status.success());

    // Test with missing output argument
    let output = Command::new("cargo")
        .args(["run", "--", "-j", "template.jinja"])
        .output()
        .expect("Failed to execute jinrender");

    assert!(!output.status.success());

    // Test with no arguments
    let output = Command::new("cargo")
        .args(["run", "--"])
        .output()
        .expect("Failed to execute jinrender");

    assert!(!output.status.success());
}
