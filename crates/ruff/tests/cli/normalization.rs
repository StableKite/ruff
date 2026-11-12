use anyhow::Result;

use crate::{CliTest, missing_file_error_filter};

#[test]
fn missing_file_filter_retains_other_error_codes_and_messages() -> Result<()> {
    let filter = regex::Regex::new(&missing_file_error_filter())?;
    let missing = std::io::Error::from_raw_os_error(2).to_string();
    let line = format!("  Cause: {missing}\n");
    assert_eq!(
        filter.replace_all(&line, "No such file or directory (os error 2)"),
        "  Cause: No such file or directory (os error 2)\n"
    );
    for other in [
        std::io::Error::from_raw_os_error(3).to_string(),
        std::io::Error::from_raw_os_error(13).to_string(),
        "unrelated message (os error 2)".to_string(),
        format!("{missing} extra text"),
    ] {
        assert!(!filter.is_match(&other), "{other}");
    }
    Ok(())
}

#[test]
fn version_filters_escape_regex_metacharacters() -> Result<()> {
    for version in ["0.17.4", "0.17.4+stablekite.1"] {
        let filter = regex::Regex::new(&regex::escape(version))?;
        assert_eq!(filter.replace_all(version, "[VERSION]"), "[VERSION]");
        assert!(!filter.is_match(&version.replace('.', "x")));
        if version.contains('+') {
            assert!(!filter.is_match(&version.replace('+', "")));
        }
    }
    Ok(())
}

#[test]
fn required_version_exact_and_public_specifiers_accept_the_actual_version() -> Result<()> {
    let version = env!("CARGO_PKG_VERSION");
    let public = version
        .split_once('+')
        .map_or(version, |(public, _)| public);
    for specifier in [
        format!("=={version}"),
        format!("=={public}"),
        format!(">={public}"),
    ] {
        let test = CliTest::with_files([
            (
                "ruff.toml",
                format!("required-version = \"{specifier}\"\n").as_str(),
            ),
            ("clean.py", "answer = 42\n"),
        ])?;
        let output = test
            .command()
            .args(["check", "--no-cache", "--config", "ruff.toml", "clean.py"])
            .output()?;
        assert_eq!(
            output.status.code(),
            Some(0),
            "{specifier}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(output.stdout, b"All checks passed!\n");
        assert!(output.stderr.is_empty());
    }
    Ok(())
}

#[test]
fn required_version_rejects_a_different_exact_version() -> Result<()> {
    let test = CliTest::with_file("ruff.toml", "required-version = '==0.1.0'\n")?;
    test.write_file("clean.py", "answer = 42\n")?;
    let output = test
        .command()
        .args(["check", "--no-cache", "--config", "ruff.toml", "clean.py"])
        .output()?;
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8(output.stderr)?;
    assert!(stderr.contains("Required version `==0.1.0` does not match the running version"));
    assert!(stderr.contains(env!("CARGO_PKG_VERSION")));
    Ok(())
}

#[test]
fn required_version_rejects_ordered_local_specifiers() -> Result<()> {
    for operator in [">", ">=", "<", "<="] {
        let specifier = format!("{operator}0.17.4+stablekite.1");
        let test = CliTest::with_file("ruff.toml", &format!("required-version = '{specifier}'\n"))?;
        test.write_file("clean.py", "answer = 42\n")?;
        let output = test
            .command()
            .args(["check", "--no-cache", "--config", "ruff.toml", "clean.py"])
            .output()?;
        assert_eq!(output.status.code(), Some(2), "{specifier}");
        assert!(output.stdout.is_empty());
        let stderr = String::from_utf8(output.stderr)?;
        assert!(stderr.contains("Failed to parse version:"), "{stderr}");
        assert!(
            stderr.contains("incompatible with versions containing non-empty local segments"),
            "{stderr}"
        );
        assert!(stderr.contains(&specifier), "{stderr}");
    }
    Ok(())
}
