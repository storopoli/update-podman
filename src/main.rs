use std::process::Command;

#[derive(Debug, PartialEq)]
enum Status {
    New,
    Old,
}

fn get_status(str: &str) -> Status {
    let splits: Vec<String> = str.split_whitespace().map(|s| s.to_string()).collect();
    let age = &splits[4];
    match age.contains("day") || age.contains("hour") {
        true => Status::New,
        false => Status::Old,
    }
}

fn get_name(str: &str) -> String {
    let splits: Vec<String> = str.split_whitespace().map(|s| s.to_string()).collect();
    let name = &splits[0];
    name.to_owned()
}

fn get_image_id(str: &str) -> String {
    let splits: Vec<String> = str.split_whitespace().map(|s| s.to_string()).collect();
    let name = &splits[2];
    name.to_owned()
}

fn parse_podman_output() -> Vec<String> {
    let output = Command::new("podman")
        .args(["image", "list"])
        .output()
        .expect("failed to execute podman image list");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut splits: Vec<String> = stdout.lines().map(|s| s.to_string()).collect();
    // the first index is the header
    splits.remove(0);
    splits
}

fn update_container(container: &str) -> std::process::Child {
    let container_name = get_name(container);
    Command::new("podman")
        .arg("pull")
        .arg(container_name)
        .spawn()
        .expect("podman pull failed to start")
}

fn update_containers(splits: Vec<String>) {
    let old_containers: Vec<String> = splits
        .into_iter()
        .filter(|s| get_status(s) == Status::Old)
        .collect();

    for c in old_containers {
        let status = update_container(&c)
            .wait()
            .expect("podman pull failed to run");

        if !status.success() {
            println!("podman pull failed to run");
            std::process::exit(1);
        }
    }
}

fn remove_container(container: &str) -> std::process::Child {
    let container_image_id = get_image_id(container);
    Command::new("podman")
        .arg("rmi")
        .arg(container_image_id)
        .arg("-f")
        .spawn()
        .expect("podman rmi failed to start")
}

fn remove_containers(splits: Vec<String>) {
    let containers: Vec<String> = splits
        .into_iter()
        .filter(|s| get_name(s).contains("<none>"))
        .collect();

    for c in containers {
        let status = remove_container(&c)
            .wait()
            .expect("podman rmi failed to run");

        if !status.success() {
            println!("podman rmi failed to run");
            std::process::exit(1);
        }
    }
}

fn main() {
    let podman_containers = parse_podman_output();
    update_containers(podman_containers);
    let podman_containers = parse_podman_output();
    remove_containers(podman_containers);
}

#[cfg(test)]
mod tests {
    use super::get_image_id;
    use super::get_name;
    use super::get_status;
    use super::Status;

    #[test]
    fn test_get_status_old() {
        let str =
        "ghcr.io/owner/container                 latest      667a7cd45f0a  3 weeks ago   3.85 gb";
        assert_eq!(Status::Old, get_status(str));
    }

    #[test]
    fn test_get_status_new() {
        let str =
        "ghcr.io/owner/container                 latest      667a7cd45f0a  3 days ago   1.23 gb";
        assert_eq!(Status::New, get_status(str));
    }

    #[test]
    fn test_get_name() {
        let str =
        "ghcr.io/owner/container                 latest      667a7cd45f0a  3 days ago   1.23 gb";
        assert_eq!("ghcr.io/owner/container".to_owned(), get_name(str));
    }

    #[test]
    fn test_get_image_id() {
        let str =
        "ghcr.io/owner/container                 latest      667a7cd45f0a  3 days ago   1.23 gb";
        assert_eq!("667a7cd45f0a".to_owned(), get_image_id(str));
    }
}
