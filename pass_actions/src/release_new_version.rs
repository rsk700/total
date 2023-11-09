mod version;

use pass_tool::{
    actions::{
        action, command, copy_file_named, create_dir, delete_file, many, replace_in_file_once,
    },
    checks::{check, file_contains_once, is_dir, not_op, stdout_contains_once},
    instruction,
    interfaces::{Action, ActionResult, Check},
    run_cli_with_input, Playbook,
};
use std::{env, path::PathBuf};
use version::Version;

const ABOUT: &str = "Release new version of Total app";
const HELP: &str = "expecting current version and new version, use syntax:
1.0.1->1.0.2";

pub fn main() {
    run_cli_with_input(get_playbook, HELP, include_str!("release_new_version.rs"));
}

fn get_playbook(input: &[u8]) -> Result<Playbook, String> {
    let (current_version, new_version) = parse_input(input)?;
    let rep_path = get_repository_path();
    let cargo_toml = rep_path.join("src-tauri").join("Cargo.toml");
    let tauri_conf_json = rep_path.join("src-tauri").join("tauri.conf.json");
    let releases_path = rep_path.join("_releases");
    let new_release_dir = releases_path.join(new_version.to_string());
    let linux_bin_name = "total.AppImage";
    Ok(Playbook::new(
        "Release new Total app",
        ABOUT,
        [
            ("Current version is less than new version", move || {
                current_version < new_version
            })
                .into_check(),
            ("Current version patch is odd", move || {
                current_version.patch() % 2 != 0
            })
                .into_check(),
            ("New version patch is even", move || {
                new_version.patch() % 2 == 0
            })
                .into_check(),
            check(
                "Release notes ready",
                file_contains_once(
                    rep_path.join("RELEASE_NOTES.md"),
                    format!("# {}", new_version),
                ),
            ),
            check(
                "No uncommited changes in repository",
                stdout_contains_once(
                    ["git", "-C", rep_path.to_str().unwrap(), "status"],
                    "nothing to commit, working tree clean",
                ),
            ),
            check(
                "Repository is in `main` branch",
                stdout_contains_once(
                    [
                        "git",
                        "-C",
                        rep_path.to_str().unwrap(),
                        "rev-parse",
                        "--abbrev-ref",
                        "HEAD",
                    ],
                    "main",
                ),
            ),
            check(
                "No new version tag yet",
                not_op(stdout_contains_once(
                    [
                        "git",
                        "-C",
                        rep_path.to_str().unwrap(),
                        "tag",
                        "-l",
                        &new_version.to_string(),
                    ],
                    new_version.to_string(),
                )),
            ),
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
            instruction(set_dir(&rep_path)),
            instruction(action("Install node modules", command(["npm", "install"])))
                .confirm(is_dir(rep_path.join("node_modules"))),
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
            instruction(action(
                "Commit changes",
                command([
                    "git",
                    "-C",
                    rep_path.to_str().unwrap(),
                    "commit",
                    "-am",
                    &format!("update version {}->{}", current_version, new_version),
                ]),
            )),
            instruction(action(
                "Add release tag",
                command([
                    "git",
                    "-C",
                    rep_path.to_str().unwrap(),
                    "tag",
                    &new_version.to_string(),
                ]),
            )),
            instruction(action(
                "Build linux binary",
                command(["cargo", "tauri", "build"]),
            )),
            instruction(action(
                "Create directory with releases",
                create_dir(&releases_path),
            ))
            .confirm(is_dir(&releases_path)),
            instruction(action(
                "Create directory for new release",
                create_dir(&new_release_dir),
            ))
            .with_env(not_op(is_dir(&new_release_dir))),
            instruction(action(
                "Copy compiled linux binary to releases",
                copy_file_named(
                    rep_path
                        .join("src-tauri")
                        .join("target")
                        .join("release")
                        .join("bundle")
                        .join("appimage")
                        .join(format!("total_{}_amd64.AppImage", new_version)),
                    &new_release_dir,
                    linux_bin_name,
                ),
            )),
            instruction(set_dir(&new_release_dir)),
            instruction(action(
                "Create linux release zip",
                many([
                    command([
                        "zip",
                        &format!("total_{}_linux.zip", new_version),
                        &linux_bin_name,
                    ]),
                    delete_file(&new_release_dir.join(linux_bin_name)),
                ]),
            )),
            // todo: wait for windows binary?
        ],
    ))
}

fn parse_input(input: &[u8]) -> Result<(Version, Version), String> {
    let input = String::from_utf8(input.to_vec()).map_err(|_| HELP)?;
    let mut versions = input.split("->");
    let v1_str = versions.next().ok_or(HELP)?;
    let v2_str = versions.next().ok_or(HELP)?;
    Ok((
        v1_str.parse().map_err(|_| HELP)?,
        v2_str.parse().map_err(|_| HELP)?,
    ))
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

struct SetDir(PathBuf);

impl Action for SetDir {
    fn name(&self) -> &str {
        "SetDir"
    }

    fn run(&self) -> ActionResult {
        if env::set_current_dir(&self.0).is_ok() {
            ActionResult::Ok
        } else {
            ActionResult::Fail
        }
    }

    fn into_action(self) -> Box<dyn Action> {
        Box::new(self)
    }
}

fn set_dir<Dir>(dir: Dir) -> Box<dyn Action>
where
    Dir: Into<PathBuf>,
{
    SetDir(dir.into()).into_action()
}
