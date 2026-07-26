use super::{BACKEND_ENV, FluxError, RUNNER_ENV, Request};
use lightflow::runner::ModelBinding;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Copy)]
pub(super) enum Backend {
    Preview,
    External,
    Native,
}

impl Backend {
    pub(super) fn from_environment() -> Result<Self, FluxError> {
        match std::env::var(BACKEND_ENV).as_deref() {
            Ok("external") | Ok("runner") => Ok(Self::External),
            Ok("native") => Ok(Self::Native),
            Ok(value) => Err(FluxError::Configuration(format!(
                "{BACKEND_ENV} must be external or native; got {value}"
            ))),
            Err(_) if std::env::var_os(RUNNER_ENV).is_some() => Ok(Self::External),
            Err(_) => Err(FluxError::Configuration(format!(
                "no FLUX backend configured; set {RUNNER_ENV} to an executable backend or {BACKEND_ENV}=native"
            ))),
        }
    }

    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::Preview => "preview",
            Self::External => "external",
            Self::Native => "native",
        }
    }

    pub(super) fn implementation_identity(self) -> String {
        match self {
            Self::Preview => "lightflow.flux.preview.v1".to_owned(),
            Self::External => std::env::var_os(RUNNER_ENV)
                .map(PathBuf::from)
                .map(|path| format!("external:{}", path.display()))
                .unwrap_or_else(|| "external:unconfigured".to_owned()),
            Self::Native => format!(
                "lightflow.flux.native.{}:{}",
                env!("CARGO_PKG_VERSION"),
                native_acceleration()
            ),
        }
    }
}

pub(super) fn run_external(
    request: &Request,
    model_bindings: &BTreeMap<String, ModelBinding>,
    output_path: &Path,
    index: usize,
) -> Result<(), FluxError> {
    let runner = std::env::var_os(RUNNER_ENV)
        .map(PathBuf::from)
        .ok_or_else(|| FluxError::Configuration(format!("{RUNNER_ENV} is required")))?;
    if !runner.is_file() {
        return Err(FluxError::Configuration(format!(
            "{RUNNER_ENV} does not point to a file: {}",
            runner.display()
        )));
    }
    let models = Models::from_bindings(model_bindings)?;
    let mut command = Command::new(&runner);
    command
        .arg("--task")
        .arg(request.task.as_str())
        .arg("--prompt")
        .arg(&request.prompt)
        .arg("--negative")
        .arg(&request.negative)
        .arg("--width")
        .arg(request.width.to_string())
        .arg("--height")
        .arg(request.height.to_string())
        .arg("--seed")
        .arg(request.seed.saturating_add(index as i64).to_string())
        .arg("--steps")
        .arg(request.steps.to_string())
        .arg("--guidance")
        .arg(request.guidance.to_string())
        .arg("--strength")
        .arg(request.strength.to_string())
        .arg("--output")
        .arg(output_path)
        .arg("--flux-model")
        .arg(models.flux)
        .arg("--llm-model")
        .arg(models.llm)
        .arg("--vae-model")
        .arg(models.vae);
    if let Some(path) = &request.image_path {
        command.arg("--image").arg(path);
    }
    if let Some(path) = &request.mask_path {
        command.arg("--mask").arg(path);
    }
    let output = super::process::run(&mut command, "FLUX external backend")?;
    if !output.status.success() {
        return Err(FluxError::Backend(format!(
            "FLUX backend {} failed with {}: {}",
            runner.display(),
            output.status,
            String::from_utf8_lossy(&output.stderr)
        )));
    }
    Ok(())
}

pub(super) fn run_native(
    request: &Request,
    model_bindings: &BTreeMap<String, ModelBinding>,
    output_path: &Path,
    index: usize,
) -> Result<(), FluxError> {
    #[cfg(feature = "native")]
    {
        let models = Models::from_bindings(model_bindings)?;
        super::native::generate(super::native::NativeFluxRequest {
            task: request.task.as_str(),
            prompt: &request.prompt,
            negative: &request.negative,
            width: request.width as i32,
            height: request.height as i32,
            seed: request.seed.saturating_add(index as i64),
            steps: request.steps as i32,
            guidance: request.guidance as f32,
            cfg_scale: 1.0,
            strength: request.strength as f32,
            image_path: request.image_path.as_deref(),
            mask_path: request.mask_path.as_deref(),
            output_path,
            diffusion_model: &models.flux,
            llm_model: &models.llm,
            vae_model: &models.vae,
        })
    }
    #[cfg(not(feature = "native"))]
    {
        let _ = (request, model_bindings, output_path, index);
        Err(FluxError::Configuration(
            "native FLUX backend requested, but lightflow-flux-runtime was not built with --features native"
                .to_owned(),
        ))
    }
}

struct Models {
    flux: PathBuf,
    llm: PathBuf,
    vae: PathBuf,
}

impl Models {
    fn from_bindings(bindings: &BTreeMap<String, ModelBinding>) -> Result<Self, FluxError> {
        Ok(Self {
            flux: required_model_binding(bindings, "flux_model")?,
            llm: required_model_binding(bindings, "llm_model")?,
            vae: required_model_binding(bindings, "vae_model")?,
        })
    }
}

fn required_model_binding(
    bindings: &BTreeMap<String, ModelBinding>,
    requirement_id: &str,
) -> Result<PathBuf, FluxError> {
    let path = bindings
        .get(requirement_id)
        .map(|binding| binding.path.clone())
        .ok_or_else(|| {
            FluxError::Configuration(format!(
                "runner request is missing resolved model binding `{requirement_id}`"
            ))
        })?;
    if !path.is_file() {
        return Err(FluxError::Configuration(format!(
            "resolved model binding `{requirement_id}` does not point to a file: {}",
            path.display()
        )));
    }
    Ok(path)
}

fn native_acceleration() -> &'static str {
    if cfg!(feature = "native-cuda") {
        "cuda"
    } else if cfg!(feature = "native-vulkan") {
        "vulkan"
    } else if cfg!(feature = "native") {
        "cpu"
    } else {
        "unavailable"
    }
}
