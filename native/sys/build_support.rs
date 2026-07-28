use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn source_files(root: &Path) -> io::Result<Vec<PathBuf>> {
    fn visit(root: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
        let mut entries = fs::read_dir(root)?.collect::<Result<Vec<_>, _>>()?;
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            let path = entry.path();
            if entry.file_name() == ".git" {
                continue;
            }
            let kind = entry.file_type()?;
            if kind.is_dir() {
                visit(&path, files)?;
            } else if kind.is_file() {
                files.push(path);
            }
        }
        Ok(())
    }

    let mut files = Vec::new();
    visit(root, &mut files)?;
    Ok(files)
}

pub fn replace_tree(source: &Path, destination: &Path) -> io::Result<()> {
    if destination.exists() {
        fs::remove_dir_all(destination)?;
    }
    copy_tree(source, destination)
}

fn copy_tree(source: &Path, destination: &Path) -> io::Result<()> {
    fs::create_dir_all(destination)?;
    for file in source_files(source)? {
        let relative = file.strip_prefix(source).map_err(io::Error::other)?;
        let target = destination.join(relative);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(file, target)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn replacement_reflects_source_changes_and_removes_stale_files() {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "lightflow-flux-build-support-{}-{nonce}",
            std::process::id()
        ));
        let source = root.join("source");
        let destination = root.join("destination");
        fs::create_dir_all(source.join("nested")).expect("source");
        fs::write(source.join("nested/model.cpp"), "first").expect("source file");
        replace_tree(&source, &destination).expect("initial sync");
        fs::write(source.join("nested/model.cpp"), "second").expect("changed source");
        fs::write(destination.join("stale.cpp"), "stale").expect("stale target");
        replace_tree(&source, &destination).expect("refresh");
        assert_eq!(
            fs::read_to_string(destination.join("nested/model.cpp")).expect("target"),
            "second"
        );
        assert!(!destination.join("stale.cpp").exists());
        fs::remove_dir_all(root).expect("cleanup");
    }
}
