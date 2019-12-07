use ansi_term::Color;
use std::process::Command;
use std::{io, str};

use crate::common::{self, TestCommand};

#[test]
fn test_render_commit_hash() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

    let mut git_output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .current_dir(repo_dir.as_path())
        .output()?
        .stdout;
    git_output.truncate(7);
    let expected_hash = str::from_utf8(&git_output).unwrap();

    let output = common::render_module("git_commit")
        .use_config(toml::toml! {
            [git_commit]
                disabled = false
        })
        .arg("--path")
        .arg(repo_dir)
        .output()?;

    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format!(
        "{} ",
        Color::Green.bold().paint(format!("({})", expected_hash))
    );

    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn test_render_commit_hash_len_override() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

    let mut git_output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .current_dir(repo_dir.as_path())
        .output()?
        .stdout;
    git_output.truncate(14);
    let expected_hash = str::from_utf8(&git_output).unwrap();

    let output = common::render_module("git_commit")
        .use_config(toml::toml! {
            [git_commit]
                disabled = false
                commit_hash_length = 14
        })
        .arg("--path")
        .arg(repo_dir)
        .output()?;

    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format!(
        "{} ",
        Color::Green.bold().paint(format!("({})", expected_hash))
    );

    assert_eq!(expected, actual);
    Ok(())
}