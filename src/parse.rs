use std::process::Command;

pub fn parse_podman_output() -> Vec<String> {
    let output = Command::new("podman")
        .args(["image", "list"])
        .output()
        .expect("failed to execute podman image list");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut splits: Vec<String> = stdout
        .lines()
        .map(std::string::ToString::to_string)
        .collect();
    // the first index is the header
    splits.remove(0);
    splits
}
