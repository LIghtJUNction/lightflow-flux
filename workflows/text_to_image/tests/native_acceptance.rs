//! Two deliberately distinct native acceptance layers:
//! - `native_test_seam_*` proves the product runner's native branch, FFI image
//!   ownership/release, PNG encoding, atomic commit, and response contract.
//!   It does not prove real model inference.
//! - `real_model_*` performs actual inference when all three model environment
//!   variables are supplied, and fails closed on partial configuration.

#![cfg(feature = "native-test-seam")]

use lightflow::runner::{ModelBinding, PROTOCOL, Request, Response, WorkflowIdentity};
use lightflow::serde_json::{Map, Value};
use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

const RUNNER: &str = env!("CARGO_BIN_EXE_lightflow-flux-text-to-image-runner");

#[test]
fn native_test_seam_exercises_runner_ffi_png_and_atomic_commit() {
    let root = test_root("seam");
    let models = fixture_models(&root);
    let response = run_native(
        &root,
        models,
        [("prompt".to_owned(), "native seam".into())]
            .into_iter()
            .chain([
                ("width".to_owned(), 64.into()),
                ("height".to_owned(), 64.into()),
                ("output_path".to_owned(), "out/native-seam.png".into()),
            ])
            .collect(),
        true,
    );

    let output = root.join("out/native-seam.png");
    assert!(
        fs::read(&output)
            .expect("PNG")
            .starts_with(b"\x89PNG\r\n\x1a\n")
    );
    assert_eq!(response.outputs["image_path"], "out/native-seam.png");
    assert_eq!(response.artifacts[0].path, "out/native-seam.png");
    assert_eq!(response.replay_fingerprint["backend"], "native");
    assert!(
        !root
            .join("out")
            .read_dir()
            .expect("output directory")
            .any(|entry| {
                entry
                    .expect("output entry")
                    .file_name()
                    .to_string_lossy()
                    .contains(".lightflow-")
            })
    );
    fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn native_test_seam_preserves_xdg_defaults_and_padded_templates() {
    let root = test_root("legacy-io");
    let config = root.join(".config");
    fs::create_dir_all(&config).expect("XDG config");
    fs::write(
        config.join("user-dirs.dirs"),
        "XDG_PICTURES_DIR=\"$HOME/Images\"\n",
    )
    .expect("XDG pictures");
    let models = fixture_models(&root);
    let response = run_native(
        &root,
        models.clone(),
        Map::from_iter([
            ("prompt".to_owned(), "two images".into()),
            ("seed".to_owned(), 90.into()),
            ("count".to_owned(), 2.into()),
            ("width".to_owned(), 64.into()),
            ("height".to_owned(), 64.into()),
        ]),
        true,
    );
    let expected = [
        root.join("Images/lightflow/lightflow_flux_text_to_image/90-001.png"),
        root.join("Images/lightflow/lightflow_flux_text_to_image/91-002.png"),
    ];
    assert_eq!(
        response.outputs["image_paths"],
        Value::Array(
            expected
                .iter()
                .map(|path| path.display().to_string().into())
                .collect()
        )
    );
    assert!(
        expected
            .iter()
            .all(|path| fs::read(path).is_ok_and(|bytes| bytes.starts_with(b"\x89PNG")))
    );

    let templated = run_native(
        &root,
        models,
        Map::from_iter([
            ("prompt".to_owned(), "templated".into()),
            ("seed".to_owned(), 80.into()),
            ("count".to_owned(), 2.into()),
            ("width".to_owned(), 64.into()),
            ("height".to_owned(), 64.into()),
            (
                "output_path".to_owned(),
                "~/Images/cat-{index:03}-{seed}.webp".into(),
            ),
        ]),
        true,
    );
    assert_eq!(
        templated.outputs["image_paths"],
        json_array([
            root.join("Images/cat-001-80.webp"),
            root.join("Images/cat-002-81.webp"),
        ])
    );
    fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn real_model_native_runner_smoke_when_models_are_configured() {
    let configured = [
        std::env::var_os("LIGHTFLOW_FLUX_REAL_FLUX_MODEL"),
        std::env::var_os("LIGHTFLOW_FLUX_REAL_LLM_MODEL"),
        std::env::var_os("LIGHTFLOW_FLUX_REAL_VAE_MODEL"),
    ];
    if configured.iter().all(Option::is_none) {
        eprintln!("real native inference skipped: no multi-GB model fixture configured");
        return;
    }
    assert!(
        configured.iter().all(Option::is_some),
        "real native inference requires all LIGHTFLOW_FLUX_REAL_*_MODEL variables"
    );

    let root = test_root("real");
    let models = bindings([
        ("flux_model", configured[0].clone().expect("flux")),
        ("llm_model", configured[1].clone().expect("llm")),
        ("vae_model", configured[2].clone().expect("vae")),
    ]);
    let response = run_native(
        &root,
        models,
        Map::from_iter([
            ("prompt".to_owned(), "a small red square".into()),
            ("width".to_owned(), 64.into()),
            ("height".to_owned(), 64.into()),
            ("steps".to_owned(), 1.into()),
            ("output_path".to_owned(), "out/native-real.png".into()),
        ]),
        false,
    );

    assert_eq!(response.replay_fingerprint["backend"], "native");
    assert!(
        fs::read(root.join("out/native-real.png"))
            .expect("real native PNG")
            .starts_with(b"\x89PNG\r\n\x1a\n")
    );
    fs::remove_dir_all(root).expect("cleanup");
}

fn run_native(
    root: &Path,
    models: BTreeMap<String, ModelBinding>,
    inputs: Map<String, Value>,
    enable_test_seam: bool,
) -> Response {
    let request = Request {
        protocol: PROTOCOL.to_owned(),
        workflow: WorkflowIdentity {
            id: lightflow_flux_text_to_image::WORKFLOW_ID.to_owned(),
            version: lightflow_flux_text_to_image::WORKFLOW_VERSION.to_owned(),
        },
        inputs,
        models,
    };
    let mut command = Command::new(RUNNER);
    command
        .current_dir(root)
        .env("HOME", root)
        .env("XDG_CONFIG_HOME", root.join(".config"))
        .env_remove("XDG_PICTURES_DIR")
        .env("LIGHTFLOW_FLUX_BACKEND", "native")
        .env_remove("LIGHTFLOW_FLUX_RUNNER")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if enable_test_seam {
        command.env("LIGHTFLOW_FLUX_NATIVE_TEST_SEAM", "1");
    } else {
        command.env_remove("LIGHTFLOW_FLUX_NATIVE_TEST_SEAM");
    }
    let mut child = command.spawn().expect("spawn product runner");
    lightflow::serde_json::to_writer(child.stdin.as_mut().expect("runner stdin"), &request)
        .expect("write runner request");
    child
        .stdin
        .take()
        .expect("runner stdin")
        .flush()
        .expect("flush request");
    let output = child.wait_with_output().expect("runner output");
    assert!(
        output.status.success(),
        "native runner failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    lightflow::serde_json::from_slice(&output.stdout).expect("runner response")
}

fn json_array<const N: usize>(paths: [PathBuf; N]) -> Value {
    Value::Array(
        paths
            .into_iter()
            .map(|path| path.display().to_string().into())
            .collect(),
    )
}

fn fixture_models(root: &Path) -> BTreeMap<String, ModelBinding> {
    let model_dir = root.join("models");
    fs::create_dir_all(&model_dir).expect("model directory");
    let paths = [
        ("flux_model", model_dir.join("flux.gguf")),
        ("llm_model", model_dir.join("llm.gguf")),
        ("vae_model", model_dir.join("vae.safetensors")),
    ];
    for (_, path) in &paths {
        fs::write(path, b"test seam only; not model inference").expect("fixture model");
    }
    bindings(paths.map(|(id, path)| (id, path.into_os_string())))
}

fn bindings<const N: usize>(
    entries: [(&str, std::ffi::OsString); N],
) -> BTreeMap<String, ModelBinding> {
    entries
        .into_iter()
        .map(|(requirement_id, path)| {
            (
                requirement_id.to_owned(),
                ModelBinding {
                    requirement_id: requirement_id.to_owned(),
                    variant_id: "native-acceptance".to_owned(),
                    path: PathBuf::from(path),
                    sha256: None,
                    size_bytes: None,
                    snapshot_revision: None,
                },
            )
        })
        .collect()
}

fn test_root(label: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let root = std::env::temp_dir().join(format!(
        "lightflow-flux-native-{label}-{}-{nonce}",
        std::process::id()
    ));
    fs::create_dir_all(&root).expect("test root");
    root
}
