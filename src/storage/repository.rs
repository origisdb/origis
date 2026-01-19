use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Name of the origis directory
const ORIGIS_DIR: &str = ".origis";

/// Name of the snapshots directory
const SNAPSHOTS_DIR: &str = "snapshots";

/// Name of the HEAD file
const HEAD_FILE: &str = "HEAD";

/// Name of the config file
const CONFIG_FILE: &str = "config.toml";

/// Name of the index file
const INDEX_FILE: &str = "index.json";

/// Errors that can occur during repository operations
#[derive(Error, Debug)]
pub enum RepositoryError {
    /// The repository is already initialized in this directory.
    #[error("Origis already initialized")]
    AlreadyInitialized,

    /// A filesystem error occurred.
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// Initializes a new Origis repository in the given path.
///
/// Creates the `.origis` directory structure with:
/// - `snapshots/` directory for storing database snapshots
/// - `HEAD` file to track the current snapshot
/// - `config.toml` for repository configuration
/// - `index.json` for snapshot metadata
///
/// # Arguments
///
/// * `path` - The root directory where `.origis` will be created
///
/// # Errors
///
/// Returns `RepositoryError::AlreadyInitialized` if `.origis` already exists.
/// Returns `RepositoryError::Io` if a filesystem operation fails.
pub fn init_repository(path: &Path) -> Result<(), RepositoryError> {
    if is_initialized(path) {
        return Err(RepositoryError::AlreadyInitialized);
    }

    let origis_path = path.join(ORIGIS_DIR);

    fs::create_dir_all(origis_path.join(SNAPSHOTS_DIR))?;

    fs::write(origis_path.join(HEAD_FILE), "")?;
    fs::write(origis_path.join(CONFIG_FILE), "")?;
    fs::write(origis_path.join(INDEX_FILE), "[]")?;

    Ok(())
}

/// Checks if the given path contains an initialized Origis repository.
///
/// A path is considered initialized if it has a `.origis` directory
/// with a `config.toml` file.
///
/// # Arguments
///
/// * `path` - The directory to check
///
/// # Returns
///
/// Returns `true` if initialized, `false` otherwise.
pub fn is_initialized(path: &Path) -> bool {
    if !path.join(ORIGIS_DIR).exists() {
        return false;
    }

    if !path.join(ORIGIS_DIR).join(CONFIG_FILE).exists() {
        return false;
    }

    true
}

/// Searches for the root of an Origis repository by traversing up the directory tree.
///
/// Starts from `start` and moves up parent directories until it finds a valid
/// Origis repository or reaches the filesystem root.
///
/// # Arguments
///
/// * `start` - The directory to start searching from
///
/// # Returns
///
/// Returns `Some(PathBuf)` containing the repository root if found,
/// or `None` if no repository exists in any parent directory.
pub fn find_repository_root(start: &Path) -> Option<PathBuf> {
    let mut current = start.to_path_buf();

    loop {
        if is_initialized(&current) {
            return Some(current);
        }

        if !current.pop() {
            return None;
        }
    }
}

// ==================== Tests ====================

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    // ==================== init_repository tests ====================

    /// Verifies that `init_repository` creates the complete `.origis` structure.
    ///
    /// Expected structure:
    /// - `.origis/` directory
    /// - `.origis/snapshots/` directory
    /// - `.origis/HEAD` file
    /// - `.origis/config.toml` file
    /// - `.origis/index.json` file
    #[test]
    fn test_init_repository_creates_structure() {
        let temp_dir = std::env::temp_dir().join("origis-test");
        let _ = fs::remove_dir_all(&temp_dir);

        let result = init_repository(&temp_dir);

        assert!(result.is_ok());
        assert!(temp_dir.join(".origis").exists());
        assert!(temp_dir.join(".origis/snapshots").is_dir());
        assert!(temp_dir.join(".origis/HEAD").exists());
        assert!(temp_dir.join(".origis/config.toml").exists());
        assert!(temp_dir.join(".origis/index.json").exists());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// Verifies that `init_repository` fails when called twice on the same directory.
    ///
    /// This prevents accidental re-initialization which could corrupt existing data.
    #[test]
    fn test_init_repository_fails_if_already_initialized() {
        let temp_dir = std::env::temp_dir().join("origis_test_already_init");
        let _ = fs::remove_dir_all(&temp_dir);

        let first = init_repository(&temp_dir);
        assert!(first.is_ok());

        let second = init_repository(&temp_dir);
        assert!(second.is_err());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    // ==================== is_initialized tests ====================

    /// Verifies that `is_initialized` returns `true` for a valid repository.
    ///
    /// A valid repository has both `.origis/` directory and `config.toml` file.
    #[test]
    fn test_is_initialized_returns_true_for_valid_repo() {
        let temp_dir = std::env::temp_dir().join("origis_test_valid_repo");
        let _ = fs::remove_dir_all(&temp_dir);

        fs::create_dir_all(temp_dir.join(".origis")).unwrap();
        fs::write(temp_dir.join(".origis/config.toml"), "").unwrap();

        assert!(is_initialized(&temp_dir));

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// Verifies that `is_initialized` returns `false` for an empty directory.
    ///
    /// A directory without `.origis/` is not a repository.
    #[test]
    fn test_is_initialized_returns_false_for_empty_dir() {
        let temp_dir = std::env::temp_dir().join("origis_test_empty_dir");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        assert!(!is_initialized(&temp_dir));

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// Verifies that `is_initialized` returns `false` when `config.toml` is missing.
    ///
    /// Even if `.origis/` exists, the repository is invalid without `config.toml`.
    #[test]
    fn test_is_initialized_returns_false_for_missing_config() {
        let temp_dir = std::env::temp_dir().join("origis_test_missing_config");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(temp_dir.join(".origis")).unwrap();

        assert!(!is_initialized(&temp_dir));

        let _ = fs::remove_dir_all(&temp_dir);
    }

    // ==================== find_repository_root tests ====================

    /// Verifies that `find_repository_root` finds a repository in the current directory.
    #[test]
    fn test_find_repository_root_finds_repo_in_current_dir() {
        let temp_dir = std::env::temp_dir().join("origis_test_find_current");
        let _ = fs::remove_dir_all(&temp_dir);
        let _ = init_repository(&temp_dir);

        let result = find_repository_root(&temp_dir);

        assert!(result.is_some());
        assert_eq!(result.unwrap(), temp_dir);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// Verifies that `find_repository_root` finds a repository by traversing up.
    ///
    /// When starting from a nested directory (e.g., `src/deep/folder`),
    /// the function should find the repository root in a parent directory.
    #[test]
    fn test_find_repository_root_finds_repo_in_parent_dir() {
        let temp_dir = std::env::temp_dir().join("origis_test_find_parent");
        let _ = fs::remove_dir_all(&temp_dir);
        let _ = init_repository(&temp_dir);

        let nested = temp_dir.join("src/deep/folder");
        fs::create_dir_all(&nested).unwrap();

        let result = find_repository_root(&nested);

        assert!(result.is_some());
        assert_eq!(result.unwrap(), temp_dir);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// Verifies that `find_repository_root` returns `None` when no repository exists.
    ///
    /// If we traverse all the way to the filesystem root without finding `.origis/`,
    /// the function should return `None`.
    #[test]
    fn test_find_repository_root_returns_none_if_no_repo() {
        let temp_dir = std::env::temp_dir().join("origis_test_find_none");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let result = find_repository_root(&temp_dir);

        assert!(result.is_none());

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
