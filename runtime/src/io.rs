use crate::FluxError;
use lightflow::serde_json::{Map, Value};
use lightflow::workflow::WorkflowArtifact;
use std::fs;
use std::io::BufWriter;
use std::path::{Component, Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub(super) fn output_paths(
    workflow_id: &str,
    inputs: &Map<String, Value>,
    seed: u64,
    count: usize,
) -> Result<Vec<PathBuf>, FluxError> {
    let output = optional_string(inputs, "output_path")?;
    let template = optional_string(inputs, "output_template")?
        .or_else(|| output.filter(|path| path_contains_template(path)));
    if let Some(template) = template {
        return validate_output_paths(
            (1..=count)
                .map(|index| {
                    expand_tilde(PathBuf::from(render_output_template(
                        template,
                        workflow_id,
                        index,
                        seed.saturating_add(index as u64 - 1),
                    )))
                })
                .collect(),
        );
    }
    if let Some(path) = output.map(PathBuf::from).map(expand_tilde) {
        if count == 1 {
            return validate_output_paths(vec![path]);
        }
        return validate_output_paths(
            (1..=count)
                .map(|index| indexed_output_path(&path, index))
                .collect(),
        );
    }
    let directory = default_picture_directory().join(workflow_id.replace('.', "_"));
    validate_output_paths(
        (1..=count)
            .map(|index| {
                let image_seed = seed.saturating_add(index as u64 - 1);
                if count == 1 {
                    directory.join(format!("{image_seed}.png"))
                } else {
                    directory.join(format!("{image_seed}-{index:03}.png"))
                }
            })
            .collect(),
    )
}

fn validate_output_paths(paths: Vec<PathBuf>) -> Result<Vec<PathBuf>, FluxError> {
    let mut seen = std::collections::BTreeSet::new();
    for path in &paths {
        if path.as_os_str().is_empty()
            || path
                .components()
                .any(|component| matches!(component, Component::ParentDir | Component::Prefix(_)))
        {
            return Err(FluxError::Backend(format!(
                "output path must not traverse a parent directory: {}",
                path.display()
            )));
        }
        if !seen.insert(path.clone()) {
            return Err(FluxError::Backend(format!(
                "output paths must be unique: {}",
                path.display()
            )));
        }
    }
    Ok(paths)
}

fn path_contains_template(path: &str) -> bool {
    path.contains("{index") || path.contains("{seed}") || path.contains("{workflow_id}")
}

fn render_output_template(template: &str, workflow_id: &str, index: usize, seed: u64) -> String {
    let mut output = template
        .replace("{index}", &index.to_string())
        .replace("{index0}", &(index - 1).to_string())
        .replace("{seed}", &seed.to_string())
        .replace("{workflow_id}", workflow_id);
    for width in 1..=9 {
        let placeholder = format!("{{index:0{width}}}");
        let value = format!("{index:0width$}");
        output = output.replace(&placeholder, &value);
    }
    output
}

fn indexed_output_path(path: &Path, index: usize) -> PathBuf {
    let parent = path.parent().unwrap_or_else(|| Path::new(""));
    let stem = path
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("image");
    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("png");
    parent.join(format!("{stem}-{index:03}.{extension}"))
}

fn expand_tilde(path: PathBuf) -> PathBuf {
    let Some(path_text) = path.to_str() else {
        return path;
    };
    if path_text == "~" {
        return std::env::var_os("HOME").map(PathBuf::from).unwrap_or(path);
    }
    if let Some(rest) = path_text.strip_prefix("~/")
        && let Some(home) = std::env::var_os("HOME")
    {
        return PathBuf::from(home).join(rest);
    }
    path
}

fn default_picture_directory() -> PathBuf {
    let home = std::env::var_os("HOME").map(PathBuf::from);
    let pictures = std::env::var_os("XDG_PICTURES_DIR")
        .map(PathBuf::from)
        .filter(|path| !path.as_os_str().is_empty())
        .or_else(|| {
            let home = home.as_ref()?;
            let config_home = std::env::var_os("XDG_CONFIG_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|| home.join(".config"));
            let source = fs::read_to_string(config_home.join("user-dirs.dirs")).ok()?;
            parse_xdg_picture_directory(&source, home)
        })
        .or_else(|| home.as_ref().map(|home| home.join("Pictures")))
        .unwrap_or_else(|| PathBuf::from("Pictures"));
    pictures.join("lightflow")
}

fn parse_xdg_picture_directory(source: &str, home: &Path) -> Option<PathBuf> {
    for line in source.lines().map(str::trim) {
        if line.starts_with('#') {
            continue;
        }
        let Some(value) = line
            .strip_prefix("XDG_PICTURES_DIR")
            .and_then(|line| line.strip_prefix('='))
        else {
            continue;
        };
        let value = value.trim().trim_matches('"');
        if value.is_empty() {
            return None;
        }
        if let Some(suffix) = value
            .strip_prefix("$HOME/")
            .or_else(|| value.strip_prefix("${HOME}/"))
        {
            return Some(home.join(suffix));
        }
        if matches!(value, "$HOME" | "${HOME}") {
            return Some(home.to_path_buf());
        }
        return Some(PathBuf::from(value));
    }
    None
}

pub(super) struct AtomicOutputs {
    entries: Vec<AtomicOutput>,
}

struct AtomicOutput {
    final_path: PathBuf,
    staged_path: PathBuf,
    backup_path: PathBuf,
}

impl AtomicOutputs {
    pub(super) fn new(final_paths: &[PathBuf]) -> Result<Self, FluxError> {
        for final_path in final_paths {
            reject_symlink_parents(final_path)?;
        }
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| FluxError::Backend(error.to_string()))?
            .as_nanos();
        let mut entries = Vec::with_capacity(final_paths.len());
        for (index, final_path) in final_paths.iter().enumerate() {
            let parent = final_path.parent().unwrap_or_else(|| Path::new("."));
            fs::create_dir_all(parent)?;
            let name = final_path
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or("image.png");
            let suffix = format!("{}-{nonce}-{index}", std::process::id());
            entries.push(AtomicOutput {
                final_path: final_path.clone(),
                staged_path: parent.join(format!(".{name}.lightflow-stage-{suffix}")),
                backup_path: parent.join(format!(".{name}.lightflow-backup-{suffix}")),
            });
        }
        Ok(Self { entries })
    }

    pub(super) fn staged_paths(&self) -> impl Iterator<Item = &Path> {
        self.entries.iter().map(|entry| entry.staged_path.as_path())
    }

    pub(super) fn commit(mut self) -> Result<(), FluxError> {
        let mut backed_up = 0;
        for entry in &self.entries {
            if entry.final_path.exists()
                && let Err(error) = fs::rename(&entry.final_path, &entry.backup_path)
            {
                self.restore_backups(backed_up);
                return Err(error.into());
            }
            backed_up += 1;
        }

        for (committed, entry) in self.entries.iter().enumerate() {
            if let Err(error) = fs::rename(&entry.staged_path, &entry.final_path) {
                for rollback in self.entries.iter().take(committed) {
                    let _ = fs::remove_file(&rollback.final_path);
                }
                self.restore_backups(backed_up);
                return Err(error.into());
            }
        }
        for entry in &self.entries {
            if entry.backup_path.exists() {
                fs::remove_file(&entry.backup_path)?;
            }
        }
        self.entries.clear();
        Ok(())
    }

    fn restore_backups(&self, count: usize) {
        for entry in self.entries.iter().take(count).rev() {
            if entry.backup_path.exists() {
                let _ = fs::rename(&entry.backup_path, &entry.final_path);
            }
        }
    }
}

fn reject_symlink_parents(path: &Path) -> Result<(), FluxError> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    for ancestor in parent.ancestors() {
        if ancestor.as_os_str().is_empty() {
            continue;
        }
        match fs::symlink_metadata(ancestor) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return Err(FluxError::Backend(format!(
                    "output parent must not be a symbolic link: {}",
                    ancestor.display()
                )));
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
    }
    Ok(())
}

impl Drop for AtomicOutputs {
    fn drop(&mut self) {
        for entry in &self.entries {
            let _ = fs::remove_file(&entry.staged_path);
            if entry.backup_path.exists() && !entry.final_path.exists() {
                let _ = fs::rename(&entry.backup_path, &entry.final_path);
            }
        }
    }
}

pub(super) fn write_preview_png(
    path: &Path,
    width: u32,
    height: u32,
    seed: u64,
    prompt: &str,
) -> Result<(), FluxError> {
    let file = fs::File::create(path)?;
    let mut encoder = png::Encoder::new(BufWriter::new(file), width, height);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder
        .write_header()
        .map_err(|error| FluxError::Backend(error.to_string()))?;
    writer
        .write_image_data(&preview_pixels(width, height, seed, prompt))
        .map_err(|error| FluxError::Backend(error.to_string()))
}

pub(super) fn preview_pixels(width: u32, height: u32, seed: u64, prompt: &str) -> Vec<u8> {
    let prompt_mix = stable_seed(prompt);
    let mut data = Vec::with_capacity(width as usize * height as usize * 3);
    for y in 0..height {
        for x in 0..width {
            let base = seed ^ prompt_mix ^ ((x as u64) << 32) ^ y as u64;
            data.push(((x * 255 / width) as u8) ^ base as u8);
            data.push(((y * 255 / height) as u8) ^ (base >> 8) as u8);
            data.push((((x + y) * 127 / (width + height)) as u8) ^ (base >> 16) as u8);
        }
    }
    data
}

fn stable_seed(value: &str) -> u64 {
    value
        .as_bytes()
        .iter()
        .fold(0xcbf2_9ce4_8422_2325_u64, |digest, byte| {
            (digest ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3)
        })
}

pub(super) fn validate_png(path: &Path) -> Result<(), FluxError> {
    let bytes = fs::read(path)?;
    if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        Ok(())
    } else {
        Err(FluxError::Backend(format!(
            "backend did not write a PNG: {}",
            path.display()
        )))
    }
}

pub(super) fn managed_artifact_paths(
    output_paths: &[PathBuf],
    workflow_id: &str,
    seed: i64,
) -> Result<Vec<PathBuf>, FluxError> {
    let project_root = std::env::current_dir()?.canonicalize()?;
    let mut artifact_paths = vec![None; output_paths.len()];
    let mut external = Vec::new();
    for (index, output_path) in output_paths.iter().enumerate() {
        let canonical = output_path.canonicalize()?;
        if let Ok(relative) = canonical.strip_prefix(&project_root) {
            artifact_paths[index] = Some(relative.to_path_buf());
        } else {
            let managed = PathBuf::from(".lightflow")
                .join("artifacts")
                .join("flux")
                .join(format!(
                    "{}-{seed}-{:03}.png",
                    safe_path_segment(workflow_id),
                    index + 1
                ));
            external.push((index, output_path, managed));
        }
    }

    if !external.is_empty() {
        let managed = external
            .iter()
            .map(|(_, _, path)| path.clone())
            .collect::<Vec<_>>();
        let transaction = AtomicOutputs::new(&managed)?;
        for ((_, source, _), staged) in external.iter().zip(transaction.staged_paths()) {
            fs::copy(source, staged)?;
            validate_png(staged)?;
        }
        transaction.commit()?;
        for (index, _, managed) in external {
            artifact_paths[index] = Some(managed);
        }
    }

    artifact_paths
        .into_iter()
        .map(|path| {
            path.ok_or_else(|| {
                FluxError::Backend("failed to resolve managed artifact path".to_owned())
            })
        })
        .collect()
}

fn safe_path_segment(value: &str) -> String {
    value
        .chars()
        .map(|character| match character {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => character,
            _ => '_',
        })
        .collect()
}

pub(super) fn required_file(
    inputs: &Map<String, Value>,
    name: &'static str,
) -> Result<PathBuf, FluxError> {
    let path = PathBuf::from(required_string(inputs, name)?);
    if path.is_file() {
        Ok(path)
    } else {
        Err(FluxError::InvalidInputPath(name, path))
    }
}

pub(super) fn required_string<'a>(
    inputs: &'a Map<String, Value>,
    name: &'static str,
) -> Result<&'a str, FluxError> {
    optional_string(inputs, name)?.ok_or(FluxError::MissingInput(name))
}

pub(super) fn optional_string<'a>(
    inputs: &'a Map<String, Value>,
    name: &'static str,
) -> Result<Option<&'a str>, FluxError> {
    inputs
        .get(name)
        .map(|value| value.as_str().ok_or(FluxError::InvalidInputType(name)))
        .transpose()
}

pub(super) fn optional_u64(
    inputs: &Map<String, Value>,
    name: &'static str,
) -> Result<Option<u64>, FluxError> {
    inputs
        .get(name)
        .map(|value| value.as_u64().ok_or(FluxError::InvalidInputType(name)))
        .transpose()
}

pub(super) fn optional_i64(
    inputs: &Map<String, Value>,
    name: &'static str,
) -> Result<Option<i64>, FluxError> {
    inputs
        .get(name)
        .map(|value| value.as_i64().ok_or(FluxError::InvalidInputType(name)))
        .transpose()
}

pub(super) fn optional_f64(
    inputs: &Map<String, Value>,
    name: &'static str,
) -> Result<Option<f64>, FluxError> {
    inputs
        .get(name)
        .map(|value| value.as_f64().ok_or(FluxError::InvalidInputType(name)))
        .transpose()
}

pub(super) fn serde_value(artifact: &WorkflowArtifact) -> Result<Value, FluxError> {
    lightflow::serde_json::to_value(artifact).map_err(|error| FluxError::Backend(error.to_string()))
}

pub(super) fn display_path(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

#[cfg(test)]
mod tests;
