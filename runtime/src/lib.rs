//! Package-owned execution for the FLUX workflow family.

use lightflow::runner::ModelBinding;
use lightflow::runner::Response;
use lightflow::serde_json::{Map, Value, json};
use lightflow::workflow::WorkflowArtifact;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};

mod backend;
mod io;
#[cfg(feature = "native")]
mod native;
#[cfg(feature = "native")]
mod native_session;
mod process;

use backend::{Backend, run_external, run_native};
#[cfg(test)]
use io::preview_pixels;
use io::{
    AtomicOutputs, display_path, managed_artifact_paths, optional_f64, optional_i64,
    optional_string, optional_u64, output_paths, required_file, required_string, serde_value,
    validate_png, write_preview_png,
};

const SOURCE_DIGEST: u64 = source_digest(include_bytes!("lib.rs"))
    ^ source_digest(include_bytes!("backend.rs")).rotate_left(7)
    ^ source_digest(include_bytes!("io.rs")).rotate_left(13)
    ^ source_digest(include_bytes!("native.rs")).rotate_left(27)
    ^ source_digest(include_bytes!("native_session.rs")).rotate_left(41);
const RUNNER_ENV: &str = "LIGHTFLOW_FLUX_RUNNER";
const BACKEND_ENV: &str = "LIGHTFLOW_FLUX_BACKEND";

/// FLUX operation implemented by one leaf workflow.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Task {
    TextToImage,
    PreviewTextToImage,
    ImageEdit,
    Inpaint,
}

impl Task {
    fn as_str(self) -> &'static str {
        match self {
            Self::TextToImage | Self::PreviewTextToImage => "text-to-image",
            Self::ImageEdit => "image-edit",
            Self::Inpaint => "inpaint",
        }
    }

    fn capability(self) -> &'static str {
        match self {
            Self::TextToImage | Self::PreviewTextToImage => "lightflow.image.generate",
            Self::ImageEdit => "lightflow.image.edit",
            Self::Inpaint => "lightflow.image.inpaint",
        }
    }
}

/// Execute one FLUX leaf through the deterministic preview or configured backend.
pub fn execute(
    workflow_id: &str,
    workflow_version: &str,
    task: Task,
    inputs: &Map<String, Value>,
    leaf_identity: &str,
) -> Result<Response, FluxError> {
    execute_with_models(
        workflow_id,
        workflow_version,
        task,
        inputs,
        &BTreeMap::new(),
        leaf_identity,
    )
}

/// Execute one FLUX leaf with host-resolved runner model bindings.
pub fn execute_with_models(
    workflow_id: &str,
    workflow_version: &str,
    task: Task,
    inputs: &Map<String, Value>,
    model_bindings: &BTreeMap<String, ModelBinding>,
    leaf_identity: &str,
) -> Result<Response, FluxError> {
    let request = Request::from_inputs(workflow_id, task, inputs)?;
    let backend = if task == Task::PreviewTextToImage {
        Backend::Preview
    } else {
        Backend::from_environment()?
    };
    let transaction = AtomicOutputs::new(&request.output_paths)?;
    for (index, output_path) in transaction.staged_paths().enumerate() {
        match backend {
            Backend::Preview => write_preview_png(
                output_path,
                request.width,
                request.height,
                request.seed.saturating_add(index as i64) as u64,
                &request.prompt,
            )?,
            Backend::External => run_external(&request, model_bindings, output_path, index)?,
            Backend::Native => run_native(&request, model_bindings, output_path, index)?,
        }
        validate_png(output_path)?;
    }
    transaction.commit()?;
    let artifact_paths = managed_artifact_paths(&request.output_paths, workflow_id, request.seed)?;
    let artifacts = request
        .output_paths
        .iter()
        .zip(&artifact_paths)
        .enumerate()
        .map(|(index, (_, artifact_path))| request.artifact(artifact_path, backend, index))
        .collect::<Vec<_>>();

    if artifacts.is_empty() {
        return Err(FluxError::InvalidInput("count must be at least one"));
    }
    let first_output = request
        .output_paths
        .first()
        .ok_or(FluxError::InvalidInput("count must be at least one"))?;
    let artifact_values = artifacts
        .iter()
        .map(serde_value)
        .collect::<Result<Vec<_>, _>>()?;
    let mut outputs = Map::from_iter([
        ("image".to_owned(), artifact_values[0].clone()),
        ("image_path".to_owned(), display_path(first_output).into()),
    ]);
    if task != Task::PreviewTextToImage {
        outputs.insert("images".to_owned(), Value::Array(artifact_values));
        outputs.insert(
            "image_paths".to_owned(),
            Value::Array(
                request
                    .output_paths
                    .iter()
                    .map(|path| display_path(path).into())
                    .collect(),
            ),
        );
    }
    let runtime_identity = implementation_identity();
    let resolved_models = lightflow::serde_json::to_value(model_bindings)
        .map_err(|error| FluxError::Backend(error.to_string()))?;
    Ok(Response {
        outputs,
        artifacts,
        replay_fingerprint: Map::from_iter([
            ("workflow_version".to_owned(), json!(workflow_version)),
            ("leaf_implementation".to_owned(), json!(leaf_identity)),
            (
                "implementation".to_owned(),
                json!(format!("{leaf_identity}+{runtime_identity}")),
            ),
            ("runtime_implementation".to_owned(), json!(runtime_identity)),
            ("backend".to_owned(), json!(backend.as_str())),
            (
                "backend_implementation".to_owned(),
                json!(backend.implementation_identity()),
            ),
            ("resolved_models".to_owned(), resolved_models),
        ]),
    })
}

/// Deterministic identity for all shared FLUX adapter source.
pub fn implementation_identity() -> String {
    format!("lightflow.flux.runtime.source.fnv1a64:{SOURCE_DIGEST:016x}")
}

#[derive(Debug)]
struct Request {
    workflow_id: String,
    task: Task,
    prompt: String,
    negative: String,
    width: u32,
    height: u32,
    seed: i64,
    count: usize,
    steps: i64,
    guidance: f64,
    strength: f64,
    image_path: Option<PathBuf>,
    mask_path: Option<PathBuf>,
    output_paths: Vec<PathBuf>,
}

impl Request {
    fn from_inputs(
        workflow_id: &str,
        task: Task,
        inputs: &Map<String, Value>,
    ) -> Result<Self, FluxError> {
        let prompt = required_string(inputs, "prompt")?.to_owned();
        let negative = optional_string(inputs, "negative")?
            .unwrap_or_default()
            .to_owned();
        let width = optional_u64(inputs, "width")?
            .unwrap_or(512)
            .clamp(64, 2048) as u32;
        let height = optional_u64(inputs, "height")?
            .unwrap_or(512)
            .clamp(64, 2048) as u32;
        let seed = optional_i64(inputs, "seed")?.unwrap_or(42);
        let mut count = None;
        for name in ["count", "num_images", "batch_count"] {
            if count.is_none() {
                count = optional_u64(inputs, name)?;
            }
        }
        let count = count.unwrap_or(1).clamp(1, 256) as usize;
        let steps = optional_i64(inputs, "steps")?.unwrap_or(20).clamp(1, 80);
        let guidance = optional_f64(inputs, "guidance")?.unwrap_or(3.5);
        let strength = optional_f64(inputs, "strength")?.unwrap_or(match task {
            Task::TextToImage | Task::PreviewTextToImage => 0.0,
            Task::ImageEdit => 0.75,
            Task::Inpaint => 0.85,
        });
        if !(0.0..=1.0).contains(&strength) {
            return Err(FluxError::InvalidInput("strength must be between 0 and 1"));
        }
        let image_path = match task {
            Task::ImageEdit | Task::Inpaint => Some(required_file(inputs, "image_path")?),
            Task::TextToImage | Task::PreviewTextToImage => None,
        };
        let mask_path = match task {
            Task::Inpaint => Some(required_file(inputs, "mask_path")?),
            _ => None,
        };
        let output_paths = output_paths(workflow_id, inputs, seed as u64, count)?;
        Ok(Self {
            workflow_id: workflow_id.to_owned(),
            task,
            prompt,
            negative,
            width,
            height,
            seed,
            count,
            steps,
            guidance,
            strength,
            image_path,
            mask_path,
            output_paths,
        })
    }

    fn artifact(&self, output_path: &Path, backend: Backend, index: usize) -> WorkflowArtifact {
        let mut metadata = Map::from_iter([
            ("capability".to_owned(), self.task.capability().into()),
            ("backend".to_owned(), backend.as_str().into()),
            ("task".to_owned(), self.task.as_str().into()),
            ("workflow_id".to_owned(), self.workflow_id.clone().into()),
            ("prompt".to_owned(), self.prompt.clone().into()),
            ("width".to_owned(), self.width.into()),
            ("height".to_owned(), self.height.into()),
            (
                "seed".to_owned(),
                self.seed.saturating_add(index as i64).into(),
            ),
            ("index".to_owned(), (index + 1).into()),
            ("count".to_owned(), self.count.into()),
            ("steps".to_owned(), self.steps.into()),
            ("guidance".to_owned(), self.guidance.into()),
            ("strength".to_owned(), self.strength.into()),
        ]);
        if let Some(path) = &self.image_path {
            metadata.insert("source_image_path".to_owned(), display_path(path).into());
        }
        if let Some(path) = &self.mask_path {
            metadata.insert("mask_path".to_owned(), display_path(path).into());
        }
        WorkflowArtifact {
            id: format!("image-{}", index + 1),
            kind: "image".to_owned(),
            path: display_path(output_path),
            mime_type: "image/png".to_owned(),
            metadata,
        }
    }
}

#[cfg(feature = "native")]
type NativeResult<T> = Result<T, FluxError>;

/// Error returned by package-owned FLUX execution.
#[derive(Debug)]
pub enum FluxError {
    MissingInput(&'static str),
    InvalidInput(&'static str),
    InvalidInputType(&'static str),
    InvalidInputPath(&'static str, PathBuf),
    Configuration(String),
    Backend(String),
    Io(std::io::Error),
}

impl fmt::Display for FluxError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingInput(name) => write!(formatter, "required input `{name}` is missing"),
            Self::InvalidInput(message) => formatter.write_str(message),
            Self::InvalidInputType(name) => {
                write!(formatter, "input `{name}` has invalid JSON type")
            }
            Self::InvalidInputPath(name, path) => {
                write!(
                    formatter,
                    "input `{name}` is not a file: {}",
                    path.display()
                )
            }
            Self::Configuration(message) => {
                write!(formatter, "FLUX configuration error: {message}")
            }
            Self::Backend(message) => write!(formatter, "FLUX backend failed: {message}"),
            Self::Io(error) => write!(formatter, "FLUX I/O failed: {error}"),
        }
    }
}

impl Error for FluxError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            _ => None,
        }
    }
}

impl From<std::io::Error> for FluxError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

const fn source_digest(source: &[u8]) -> u64 {
    let mut digest = 0xcbf2_9ce4_8422_2325_u64;
    let mut index = 0;
    while index < source.len() {
        digest ^= source[index] as u64;
        digest = digest.wrapping_mul(0x0000_0100_0000_01b3);
        index += 1;
    }
    digest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unconfigured_real_backend_fails_closed() {
        let error = Backend::from_environment().expect_err("backend must be configured");
        assert!(error.to_string().contains("no FLUX backend configured"));
    }

    #[test]
    fn preview_pixels_are_deterministic() {
        assert_eq!(
            preview_pixels(2, 2, 7, "prompt"),
            preview_pixels(2, 2, 7, "prompt")
        );
    }

    #[test]
    fn legacy_batch_aliases_keep_the_256_image_limit() {
        let inputs = Map::from_iter([
            ("prompt".to_owned(), "batch".into()),
            ("batch_count".to_owned(), 300.into()),
            ("output_path".to_owned(), "out/image.png".into()),
        ]);
        let request =
            Request::from_inputs("lightflow.test", Task::TextToImage, &inputs).expect("request");
        assert_eq!(request.count, 256);
        assert_eq!(request.output_paths.len(), 256);
    }
}
