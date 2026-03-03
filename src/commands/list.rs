use crate::cli;
use crate::config;
use crate::config::DependencySection;
use crate::config::DependencySpec;
use anyhow::Result;

pub fn run(args: cli::List) -> Result<()> {
    let config = config::Manifest::load()?;
    println!(
        "Project: {} v{}",
        config.project.name, config.project.version
    );
    let dependencies = match config.dependencies.packages {
        Some(dependencies) => dependencies,
        None => {
            println!("No dependencies found");
            return Ok(());
        }
    };

    match args.package {
        None => list_all_dependencies(&dependencies),
        Some(name) => find_dependency(&dependencies, &name),
    }

    Ok(())
}

fn list_all_dependencies(dependencies: &DependencySection) {
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

fn find_dependency(dependencies: &DependencySection, name: &str) {
    match dependencies.get(name) {
        Some(spec) => match spec {
            DependencySpec::Simple(version) => println!("{}: {}", name, version),
            DependencySpec::Detailed { version, features } => {
                println!("{}: {} features={:?}", name, version, features);
            }
        },
        None => println!("Dependency '{}' not found", name),
    };
}
