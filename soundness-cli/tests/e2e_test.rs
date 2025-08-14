use anyhow::Result;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
use tempfile::tempdir;

fn run_cli_command(args: &[&str], stdin_data: Option<&str>) -> Result<String> {
    let mut command = Command::new("cargo");
    command.arg("run").arg("--").args(args);

    if stdin_data.is_some() {
        command.stdin(Stdio::piped());
    }
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let mut child = command.spawn()?;

    if let (Some(data), Some(mut stdin)) = (stdin_data, child.stdin.take()) {
        stdin.write_all(data.as_bytes())?;
    }

    let output = child.wait_with_output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        anyhow::bail!(
            "Command failed with status: {}\n---\nSTDOUT:\n{}\n---\nSTDERR:\n{}",
            output.status,
            stdout,
            stderr
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[test]
fn test_e2e_flow() -> Result<()> {
    // Create a temporary directory for test files
    let temp_dir = tempdir()?;

    // --- Generate a test key pair ---
    let key_name = "test_key";
    let password = "test_password\n";
    let stdin_data = format!("{}{}", password, password);
    let gen_output = run_cli_command(&["generate-key", "--name", key_name], Some(&stdin_data))?;
    assert!(gen_output.contains("Generated new key pair"));

    // --- Create test files ---
    let proof_content = "test proof content";
    let elf_content = "test elf content";
    let proof_path = temp_dir.path().join("test.proof");
    let elf_path = temp_dir.path().join("test.elf");
    fs::write(&proof_path, proof_content)?;
    fs::write(&elf_path, elf_content)?;

    // --- Send the files ---
    // The `send` command will fail because there is no server running.
    // We can only test that it attempts to send.
    // We also need to provide the password for signing.
    let send_result = run_cli_command(
        &[
            "send",
            "--proof-file",
            proof_path.to_str().unwrap(),
            "--elf-file",
            elf_path.to_str().unwrap(),
            "--key-name",
            key_name,
            "--proving-system",
            "sp1",
        ],
        Some(password),
    );

    // Expect an error because the server is not running
    assert!(send_result.is_err());
    if let Err(e) = send_result {
        let error_msg = e.to_string();
        assert!(
            error_msg.contains("Failed to send request")
                || error_msg.contains("connection refused")
        );
    }

    // --- List keys ---
    let list_output = run_cli_command(&["list-keys"], None)?;
    assert!(list_output.contains(key_name));

    Ok(())
}
