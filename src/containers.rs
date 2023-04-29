use crate::utils::{get_image_id, get_name, get_status, Status};
use std::process::Command;
use std::thread::spawn;

pub fn update_container(container: &str) -> std::process::Child {
    let container_name = get_name(container);
    Command::new("podman")
        .arg("pull")
        .arg(container_name)
        .spawn()
        .expect("podman pull failed to start")
}

pub fn update_containers(splits: Vec<String>) {
    let old_containers: Vec<String> = splits
        .into_iter()
        .filter(|s| get_status(s) == Status::Old)
        .collect();

    // Make a vector to hold the children which are spawned.
    let mut children: Vec<std::thread::JoinHandle<_>> = vec![];

    for c in old_containers {
        // Spin up another thread
        children.push(spawn(move || {
            let status = update_container(&c)
                .wait()
                .expect("podman pull failed to run");

            if !status.success() {
                println!("podman pull failed to run");
                std::process::exit(1);
            }
        }));
    }
    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}

pub fn remove_container(container: &str) -> std::process::Child {
    let container_image_id = get_image_id(container);
    Command::new("podman")
        .arg("rmi")
        .arg(container_image_id)
        .arg("-f")
        .spawn()
        .expect("podman rmi failed to start")
}

pub fn remove_containers(splits: Vec<String>) {
    let containers: Vec<String> = splits
        .into_iter()
        .filter(|s| get_name(s).contains("<none>"))
        .collect();

    // Make a vector to hold the children which are spawned.
    let mut children: Vec<std::thread::JoinHandle<_>> = vec![];

    for c in containers {
        // Spin up another thread
        children.push(spawn(move || {
            let status = remove_container(&c)
                .wait()
                .expect("podman rmi failed to run");

            if !status.success() {
                println!("podman rmi failed to run");
                std::process::exit(1);
            }
        }));
    }
    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}
