const MANIFEST_FILE: &str = "bento.toml";

struct Manifest {
    project: ProjectMetadata,
    packages: PackagesTable,
}

struct ProjectMetadata {
    name: String,
    version: String,
    description: Option<String>,
    author: String,
}

struct PackagesTable {
    packages: Option<String>,
}
