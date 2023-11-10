mod version;

use std::path::PathBuf;

use pass_tool::{
    actions::{action, replace_in_file_once},
    checks::{check, file_contains_once},
    instruction, run_cli_with_input, Playbook,
};
use version::Version;

const ABOUT: &str = "Increase dev version of Total app";
const HELP: &str = "expecting current version, use syntax:
1.0.1";

pub fn main() {
    run_cli_with_input(get_playbook, HELP, include_str!("new_dev_version.rs"));
}

fn get_playbook(input: &[u8]) -> Result<Playbook, String> {
    let current_version = parse_input(input)?;
    // make new_version odd
    let new_version = Version::new(
        current_version.major(),
        current_version.minor(),
        current_version.patch() + 1 + current_version.patch() % 2,
    );
    let rep_path = get_repository_path();
    let cargo_toml = rep_path.join("src-tauri").join("Cargo.toml");
    let tauri_conf_json = rep_path.join("src-tauri").join("tauri.conf.json");
    Ok(Playbook::new(
        "Increase dev version",
        ABOUT,
        [
            check(
                "Current version is in Cargo.toml",
                file_contains_once(&cargo_toml, format!("version = \"{}\"", current_version)),
            ),
            check(
                "Current version is in tauri.conf.json",
                file_contains_once(
                    &tauri_conf_json,
                    format!("\"version\": \"{}\"", current_version),
                ),
            ),
        ],
        [
            instruction(action(
                "Update version in Cargo.toml",
                replace_in_file_once(
                    &cargo_toml,
                    format!("version = \"{}\"", current_version),
                    format!("version = \"{}\"", new_version),
                ),
            )),
            instruction(action(
                "Update version in tauri.conf.json",
                replace_in_file_once(
                    &tauri_conf_json,
                    format!("\"version\": \"{}\"", current_version),
                    format!("\"version\": \"{}\"", new_version),
                ),
            )),
        ],
    ))
}

fn parse_input(input: &[u8]) -> Result<Version, String> {
    let version = String::from_utf8(input.to_vec()).map_err(|_| HELP)?;
    Ok(version.parse().map_err(|_| HELP)?)
}

fn get_repository_path() -> PathBuf {
    std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_owned()
}
