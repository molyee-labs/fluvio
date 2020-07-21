use std::process::Command;

use log::debug;
use serde::Deserialize;

use super::*;

pub fn repo_add() {
    // add repo
    Command::new("helm")
        .arg("repo")
        .arg("add")
        .arg("fluvio")
        .arg("https://infinyon.github.io/charts")
        .inherit();
}

pub fn repo_update() {
    // add repo
    Command::new("helm").arg("repo").arg("update").inherit();
}

#[derive(Debug, Deserialize)]
struct Chart {
    name: String,
    version: String,
}

pub fn check_chart_version_exists(name: &str, version: &str) -> bool {
    let versions = core_chart_versions(name);
    versions
        .iter()
        .filter(|chart| chart.name == name && chart.version == version)
        .count()
        > 0
}

fn core_chart_versions(name: &str) -> Vec<Chart> {
    let mut cmd = Command::new("helm");
    cmd.arg("search")
        .arg("repo")
        .arg(name)
        .arg("--output")
        .arg("json");

    debug!("command {:?}", cmd);

    let output = cmd
        .output()
        .expect("unable to query for versions of fluvio-core in helm repo");

    debug!("command output {:?}", output);

    serde_json::from_slice(&output.stdout).expect("invalid json from helm search")
}
