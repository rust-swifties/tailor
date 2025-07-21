use anyhow::{Context, Result, bail};
use clap::Parser;
use log::{error, info, warn};
use std::fs::{File, metadata};
use std::path::Path;
use std::process::Command;

/// tail a file or execite the fallback command
///
/// Attempts to tail a file. If the file doesn't exist or can't be read,
/// runs the specified fallback command instead passing the specified file.
///
/// Examples:
///   tailor file.txt touch                         # tail file.txt, or touch file.txt
///   tailor file.txt chmod 755                     # tail file.txt, or chmod 755 file.txt
///   tailor config.json cp config.template.json    # tail file.txt, or cp config.template.json config.json
#[derive(Parser, Debug)]
#[command(version, about, long_about, verbatim_doc_comment)]
struct Args {
    file: String,

    #[arg(trailing_var_arg = true, num_args = 0.., help = "Command to run if file can't be tailed")]
    command: Vec<String>,
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();

    if can_tail_file(&args.file)? {
        run_command("tail", &[&args.file])?;
    } else {
        if args.command.is_empty() {
            bail!(
                "File '{}' is not readable and no fallback command specified.",
                args.file
            );
        }
        let mut command_args: Vec<&str> = args.command[1..].iter().map(|s| s.as_str()).collect();
        command_args.push(args.file.as_str());
        info!(
            "file {} cannot be tailed, executing: {} {}",
            args.file,
            &args.command[0],
            command_args.join(" ")
        );
        run_command(&args.command[0], &command_args)?;
    }

    Ok(())
}

fn can_tail_file(file_path: &str) -> Result<bool> {
    if !Path::new(file_path).exists() {
        return Ok(false);
    }
    let meta =
        metadata(file_path).with_context(|| format!("failed to read metadata for {file_path}"))?;
    if meta.is_dir() {
        warn!("{file_path} is a directory");
        return Ok(false);
    }
    match File::open(file_path) {
        Ok(_) => Ok(true),
        Err(e) => {
            warn!("cannot read file {file_path}: {e}");
            Ok(false)
        }
    }
}

fn run_command(command: &str, args: &[&str]) -> Result<()> {
    let mut cmd = Command::new(command);
    cmd.args(args);
    let status = cmd
        .status()
        .with_context(|| format!("failed to execute command '{command}'"))?;
    if !status.success() {
        let exit_code = status.code().unwrap_or(-1);
        error!("command '{command}' failed with exit code: {exit_code}");
        std::process::exit(exit_code);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_cmd::assert::OutputAssertExt;
    use assert_cmd::cargo::CommandCargoExt;
    use predicates::str::contains;
    use std::os::unix::fs::PermissionsExt;
    use std::{env, fs};
    use tempfile::{NamedTempFile, tempdir};

    #[test]
    fn test_can_tail_file_with_existing_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path().to_str().unwrap();
        let result = can_tail_file(file_path).unwrap();
        assert!(result, "Should be able to tail an existing readable file");
    }

    #[test]
    fn test_can_tail_file_with_nonexistent_file() {
        let result = can_tail_file("/tmp/nonexistent_file_12345").unwrap();
        assert!(!result, "Should not be able to tail a nonexistent file");
    }

    #[test]
    fn test_can_tail_file_with_directory() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().to_str().unwrap();
        let result = can_tail_file(dir_path).unwrap();
        assert!(!result, "Should not be able to tail a directory");
    }

    #[test]
    #[cfg(unix)]
    fn test_can_tail_file_with_unreadable_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path().to_str().unwrap();
        let mut perms = fs::metadata(file_path).unwrap().permissions();
        perms.set_mode(0o000);
        fs::set_permissions(file_path, perms).unwrap();

        let result = can_tail_file(file_path).unwrap();
        assert!(!result, "Should not be able to tail an unreadable file");

        let mut perms = fs::metadata(file_path).unwrap().permissions();
        perms.set_mode(0o644);
        fs::set_permissions(file_path, perms).unwrap();
    }

    #[test]
    fn test_run_command_success() {
        let result = run_command("true", &[]);
        assert!(result.is_ok(), "Command 'true' should succeed");
    }

    #[test]
    fn test_run_command_with_args() {
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path().to_str().unwrap();

        let result = run_command("touch", &[file_path]);
        assert!(result.is_ok(), "Touch command should succeed");
        assert!(
            Path::new(file_path).exists(),
            "File should be created by touch"
        );
    }

    #[test]
    fn test_run_command_calling_tailor_again() {
        use tempfile::tempdir;

        let temp_dir = tempdir().unwrap();
        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");
        let file3 = temp_dir.path().join("file3.txt");

        assert!(!file1.exists(), "file1 should not exist initially");
        assert!(!file2.exists(), "file2 should not exist initially");
        assert!(!file3.exists(), "file3 should not exist initially");

        let file1_str = file1.to_str().unwrap();
        let file2_str = file2.to_str().unwrap();
        let file3_str = file3.to_str().unwrap();

        let tailor_bin = assert_cmd::cargo::cargo_bin("tailor");
        let current_path = env::var("PATH").unwrap_or_default();
        let tailor_dir = tailor_bin.parent().unwrap();
        let new_path = format!("{}:{}", tailor_dir.to_str().unwrap(), current_path);

        let mut cmd = Command::cargo_bin("tailor").unwrap();
        cmd.env("PATH", new_path)
            .arg(file1_str)
            .arg("tailor")
            .arg(file2_str)
            .arg("touch")
            .arg(file3_str)
            .assert()
            .success();

        assert!(
            file1.exists(),
            "file1.txt should be created by recursive fallback"
        );
        assert!(
            file2.exists(),
            "file2.txt should be created by recursive fallback"
        );
        assert!(
            file3.exists(),
            "file3.txt should be created by touch command"
        );
    }

    #[test]
    fn test_run_command_nonexistent_command() {
        let result = run_command("nonexistent_command_12345", &[]);
        assert!(result.is_err(), "Nonexistent command should return error");
    }

    #[test]
    fn test_main_fails_without_fallback_command() {
        let mut cmd = Command::cargo_bin("tailor").unwrap();
        cmd.arg("/tmp/nonexistent_file_12345")
            .assert()
            .failure()
            .stderr(contains("no fallback command"));
    }
}
