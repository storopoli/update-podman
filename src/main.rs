mod containers;
mod parse;
mod test_utils;
mod utils;
use crate::containers::remove_containers;
use crate::containers::update_containers;
use crate::parse::parse_podman_output;

fn main() {
    let podman_containers = parse_podman_output();
    update_containers(podman_containers);
    let podman_containers = parse_podman_output();
    remove_containers(podman_containers);
}
