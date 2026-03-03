const LOCKFILE_NAME: &str = "bento.lock";

pub struct Lockfile {
    packages: Vec<String>,
}
