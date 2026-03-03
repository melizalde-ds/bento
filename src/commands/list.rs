use crate::cli;
use crate::config;
use crate::config::DependencySection;
use crate::config::DependencySpec;
use anyhow::Result;

pub fn run(_args: cli::List) -> Result<()> {
    let config = config::ProjectConfig::load()?;
    println!(
        "Project: {} v{}",
        config.project.name, config.project.version
    );
    match config.dependencies {
        Some(deps) => list_all_dependencies(deps),
        None => println!("No dependencies found"),
    }
    Ok(())
}

fn list_all_dependencies(dependencies: DependencySection) {
    if dependencies.is_empty() {
        println!("No dependencies found");
        return;
    }
    for (name, spec) in dependencies {
        match spec {
            DependencySpec::Simple(version) => println!("{}: {}", name, version),
            DependencySpec::Detailed { version, features } => {
                println!("{}: {} features={:?}", name, version, features);
            }
        }
    }
}
