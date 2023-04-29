#[cfg(test)]
mod tests {
    use crate::utils::{get_image_id, get_name, get_status, Status};

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
