mod storage;

use std::path::Path;
use storage::repository::{find_repository_root, init_repository, is_initialized};

fn main() {
    let current_dir = Path::new(".");

    // Check if we're already in a repository
    if let Some(root) = find_repository_root(current_dir) {
        println!("Found existing repository at: {}", root.display());
        return;
    }

    // Check if current directory is initialized
    if is_initialized(current_dir) {
        println!("Repository already initialized");
        return;
    }

    // Initialize new repository
    match init_repository(current_dir) {
        Ok(()) => println!("Origis initialized successfully"),
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("origis v{}", env!("CARGO_PKG_VERSION"));
}
