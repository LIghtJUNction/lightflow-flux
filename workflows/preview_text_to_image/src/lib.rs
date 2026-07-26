use lightflow::preload::*;
use lightflow::runner::Response;
use lightflow::serde_json::{Map, Value};

pub const WORKFLOW_ID: &str = "lightflow.flux_preview_text_to_image";
pub const WORKFLOW_VERSION: &str = env!("CARGO_PKG_VERSION");
const SOURCE_DIGEST: u64 = source_digest(include_bytes!("lib.rs"));

pub fn define() -> WorkflowSpec {
    workflow! {
        name: "FLUX Preview Text To Image",
        description: "Deterministic preview fallback for testing FLUX text-to-image pipelines.",
        input "use_flux": "boolean" {
            description: "Compatibility flag used by router workflows; ignored by the preview runtime.",
            required: false,
            default: false,
            widget: "toggle",
        }
        input "prompt": "text" {
            description: "Prompt text used to generate deterministic preview pixels.",
            required: true,
            widget: "prompt",
        }
        input "negative": "text" {
            description: "Optional negative prompt preserved in artifact metadata.",
            required: false,
            widget: "textarea",
        }
        input "width": "integer" {
            description: "Requested output width in pixels.",
            required: false,
            default: 512,
            range: [64, 2048, 8],
            widget: "number",
        }
        input "height": "integer" {
            description: "Requested output height in pixels.",
            required: false,
            default: 512,
            range: [64, 2048, 8],
            widget: "number",
        }
        input "seed": "integer" {
            description: "Optional deterministic seed.",
            required: false,
            widget: "number",
        }
        input "output_path": "path" {
            description: "Optional destination PNG path.",
            required: false,
            widget: "file_save",
            artifact: "image",
        }
        output "image": "artifact" {
            description: "Generated preview image artifact metadata.",
            artifact: "image",
        }
        output "image_path": "path" {
            description: "Path to the generated preview PNG image.",
            artifact: "image",
        }
    }
        .builtin_runtime("image_runtime", "lightflow.image.generate", "runner.v1")
        .build()
}

pub fn execute(inputs: &Map<String, Value>) -> Result<Response, lightflow_flux_runtime::FluxError> {
    execute_with_models(inputs, &Default::default())
}

pub fn execute_with_models(
    inputs: &Map<String, Value>,
    models: &std::collections::BTreeMap<String, lightflow::runner::ModelBinding>,
) -> Result<Response, lightflow_flux_runtime::FluxError> {
    lightflow_flux_runtime::execute_with_models(
        WORKFLOW_ID,
        WORKFLOW_VERSION,
        lightflow_flux_runtime::Task::PreviewTextToImage,
        inputs,
        models,
        &format!("lightflow.flux_preview_text_to_image.source.fnv1a64:{SOURCE_DIGEST:016x}"),
    )
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
