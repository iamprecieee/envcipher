use std::fs;

use assert_cmd::Command as AssertCommand;
use tempfile::TempDir;

// Helper to get the binary path
fn envcipher_cmd() -> AssertCommand {
    AssertCommand::new(env!("CARGO_BIN_EXE_envcipher"))
}

#[test]
fn test_help_command() {
    envcipher_cmd()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("Encipher .env files"));
}

#[test]
fn test_init_command() {
    let temp = TempDir::new().unwrap();
    let current_dir = temp.path();

    envcipher_cmd()
        .current_dir(current_dir)
        .arg("init")
        .assert()
        .success()
        .stdout(predicates::str::contains("Initialization complete!"));

    assert!(current_dir.join(".envcipher.json").exists());
}

#[test]
fn test_lock_unlock_cycle() {
    let temp = TempDir::new().unwrap();
    let current_dir = temp.path();

    // 1. Init
    envcipher_cmd()
        .current_dir(current_dir)
        .arg("init")
        .assert()
        .success();

    // 2. Create .env content
    let env_path = current_dir.join(".env");
    fs::write(&env_path, "Before=Content").unwrap();

    // 3. Lock
    envcipher_cmd()
        .current_dir(current_dir)
        .arg("lock")
        .assert()
        .success()
        .stdout(predicates::str::contains("Locked!"));

    let locked_content = fs::read_to_string(&env_path).unwrap();
    assert!(locked_content.starts_with("ENVCIPHER:v1:"));
    assert!(!locked_content.contains("Before=Content"));

    // 4. Status check
    envcipher_cmd()
        .current_dir(current_dir)
        .arg("status")
        .assert()
        .success()
        .stdout(predicates::str::contains("Locked (enciphered)"));

    // 5. Unlock
    envcipher_cmd()
        .current_dir(current_dir)
        .arg("unlock")
        .assert()
        .success()
        .stdout(predicates::str::contains("Unlocked!"));

    let unlocked_content = fs::read_to_string(&env_path).unwrap();
    assert_eq!(unlocked_content, "Before=Content");
}

#[test]
fn test_lock_fails_without_init() {
    let temp = TempDir::new().unwrap();
    let current_dir = temp.path();

    fs::write(current_dir.join(".env"), "SECRET=true").unwrap();

    envcipher_cmd()
        .current_dir(current_dir)
        .arg("lock")
        .assert()
        .failure()
        .stderr(predicates::str::contains("envcipher init"));
}

#[test]
fn test_run_command_injects_env() {
    let temp = TempDir::new().unwrap();
    let current_dir = temp.path();

    // 1. Init
    envcipher_cmd()
        .current_dir(current_dir)
        .arg("init")
        .assert()
        .success();

    // 2. Create .env with secret
    let env_path = current_dir.join(".env");
    fs::write(&env_path, "TEST_SECRET=supersecure").unwrap();

    // 3. Lock
    envcipher_cmd()
        .current_dir(current_dir)
        .arg("lock")
        .assert()
        .success();

    // 4. Run command to echo secret
    #[cfg(unix)]
    let (prog, args) = ("sh", vec!["-c", "echo $TEST_SECRET"]);
    #[cfg(windows)]
    let (prog, args) = ("cmd", vec!["/C", "echo %TEST_SECRET%"]);

    envcipher_cmd()
        .current_dir(current_dir)
        .arg("run")
        .arg("--")
        .arg(prog)
        .args(args)
        .assert()
        .success()
        .stdout(predicates::str::contains("supersecure"));

    // 5. Verify file is still enciphered on disk
    let content = fs::read_to_string(&env_path).unwrap();
    assert!(content.starts_with("ENVCIPHER:v1:"));
}

#[test]
fn test_key_export_import_cycle() {
    let temp = TempDir::new().unwrap();
    let dir_a = temp.path().join("project_a");
    let dir_b = temp.path().join("project_b");
    fs::create_dir(&dir_a).unwrap();
    fs::create_dir(&dir_b).unwrap();

    // 1. Init in A
    envcipher_cmd()
        .current_dir(&dir_a)
        .arg("init")
        .assert()
        .success();

    // Create secret content and lock
    fs::write(dir_a.join(".env"), "SHARED=secret").unwrap();
    envcipher_cmd()
        .current_dir(&dir_a)
        .arg("lock")
        .assert()
        .success();

    // 2. Export key from A
    let output = envcipher_cmd()
        .current_dir(&dir_a)
        .env("NO_COLOR", "1")
        .arg("export-key")
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let key_line = stdout
        .lines()
        .find(|l| l.len() > 40 && !l.contains(' ') && !l.contains('─'))
        .expect("Cannot find key in export output");

    // 3. Copy enciphered file to B
    fs::copy(dir_a.join(".env"), dir_b.join(".env")).unwrap();

    // 4. Import key in B
    envcipher_cmd()
        .current_dir(&dir_b)
        .arg("import-key")
        .arg(key_line)
        .assert()
        .success();

    // 5. Verify unlock in B works
    envcipher_cmd()
        .current_dir(&dir_b)
        .arg("unlock")
        .assert()
        .success();

    let content = fs::read_to_string(dir_b.join(".env")).unwrap();
    assert_eq!(content, "SHARED=secret");
}
