use super::*;

#[test]
fn output_paths_reject_escape_and_duplicates() {
    let inputs = Map::from_iter([("output_path".to_owned(), "../escape.png".into())]);
    assert!(output_paths("lightflow.test", &inputs, 1, 1).is_err());
    let inputs = Map::from_iter([("output_template".to_owned(), "same.png".into())]);
    assert!(output_paths("lightflow.test", &inputs, 1, 2).is_err());
}

#[test]
fn output_paths_preserve_legacy_templates_and_extensions() {
    let inputs = Map::from_iter([(
        "output_path".to_owned(),
        "out/cat-{index:03}-{index0}-{seed}-{workflow_id}.webp".into(),
    )]);
    assert_eq!(
        output_paths("lightflow.test", &inputs, 80, 2).expect("paths"),
        [
            PathBuf::from("out/cat-001-0-80-lightflow.test.webp"),
            PathBuf::from("out/cat-002-1-81-lightflow.test.webp"),
        ]
    );

    let inputs = Map::from_iter([("output_path".to_owned(), "out/cat.jpg".into())]);
    assert_eq!(
        output_paths("lightflow.test", &inputs, 80, 2).expect("paths"),
        [
            PathBuf::from("out/cat-001.jpg"),
            PathBuf::from("out/cat-002.jpg"),
        ]
    );
}

#[cfg(unix)]
#[test]
fn atomic_outputs_reject_symlink_parent_before_writing() {
    use std::os::unix::fs::symlink;

    let root = test_root("atomic-symlink-parent");
    let outside = test_root("atomic-symlink-outside");
    let linked_parent = root.join("linked");
    symlink(&outside, &linked_parent).expect("symlink");
    let final_path = linked_parent.join("nested/output.png");

    let error = match AtomicOutputs::new(&[final_path]) {
        Ok(_) => panic!("symlink parent must fail"),
        Err(error) => error,
    };

    assert!(error.to_string().contains("symbolic link"));
    assert!(!outside.join("nested").exists());
    assert_eq!(fs::read_dir(&outside).expect("outside entries").count(), 0);
    fs::remove_dir_all(root).expect("cleanup root");
    fs::remove_dir_all(outside).expect("cleanup outside");
}

#[test]
fn invalid_batch_staging_preserves_all_existing_outputs() {
    let root = test_root("atomic-invalid");
    let finals = [root.join("first.png"), root.join("second.png")];
    fs::write(&finals[0], b"first sentinel").expect("first sentinel");
    fs::write(&finals[1], b"second sentinel").expect("second sentinel");
    {
        let transaction = AtomicOutputs::new(&finals).expect("transaction");
        let staged = transaction.staged_paths().collect::<Vec<_>>();
        fs::write(staged[0], b"\x89PNG\r\n\x1a\nvalid").expect("first staged");
        fs::write(staged[1], b"invalid").expect("second staged");
        assert!(validate_png(staged[0]).is_ok());
        assert!(validate_png(staged[1]).is_err());
    }
    assert_eq!(fs::read(&finals[0]).expect("first"), b"first sentinel");
    assert_eq!(fs::read(&finals[1]).expect("second"), b"second sentinel");
    fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn valid_batch_commits_all_outputs_together() {
    let root = test_root("atomic-valid");
    let finals = [root.join("first.png"), root.join("second.png")];
    fs::write(&finals[0], b"first sentinel").expect("first sentinel");
    fs::write(&finals[1], b"second sentinel").expect("second sentinel");
    let transaction = AtomicOutputs::new(&finals).expect("transaction");
    for path in transaction.staged_paths() {
        fs::write(path, b"\x89PNG\r\n\x1a\nvalid").expect("staged");
        validate_png(path).expect("valid PNG");
    }
    transaction.commit().expect("commit");
    assert!(fs::read(&finals[0]).expect("first").starts_with(b"\x89PNG"));
    assert!(
        fs::read(&finals[1])
            .expect("second")
            .starts_with(b"\x89PNG")
    );
    fs::remove_dir_all(root).expect("cleanup");
}

fn test_root(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let root = std::env::temp_dir().join(format!(
        "lightflow-flux-{name}-{}-{nonce}",
        std::process::id()
    ));
    fs::create_dir_all(&root).expect("root");
    root
}
