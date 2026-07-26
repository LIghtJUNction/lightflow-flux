use lightflow::preload::*;
use lightflow::runner::Response;
use lightflow::serde_json::{Map, Value};

pub const WORKFLOW_ID: &str = "lightflow.flux_text_to_image";
pub const WORKFLOW_VERSION: &str = env!("CARGO_PKG_VERSION");
const SOURCE_DIGEST: u64 = source_digest(include_bytes!("lib.rs"));

pub fn define() -> WorkflowSpec {
    workflow! {
        name: "FLUX Text To Image",
        description: "Generate an image from a prompt with FLUX.2 klein GGUF models.",
        input "prompt": "text" {
            description: "Prompt text sent to the FLUX image generation runtime.",
            required: true,
            widget: "prompt",
        }
        input "negative": "text" {
            description: "Optional negative prompt.",
            required: false,
            widget: "textarea",
        }
        input "width": "integer" {
            description: "Output width in pixels.",
            required: false,
            default: 1024,
            range: [64, 2048, 8],
            widget: "number",
        }
        input "height": "integer" {
            description: "Output height in pixels.",
            required: false,
            default: 1024,
            range: [64, 2048, 8],
            widget: "number",
        }
        input "seed": "integer" {
            description: "Optional sampling seed.",
            required: false,
            widget: "number",
        }
        input "count": "integer" {
            description: "Number of images to generate.",
            required: false,
            default: 1,
            range: [1, 256, 1],
            widget: "number",
        }
        input "steps": "integer" {
            description: "Denoising step count.",
            required: false,
            default: 20,
            range: [1, 80, 1],
            widget: "slider",
        }
        input "guidance": "number" {
            description: "Prompt guidance scale.",
            required: false,
            default: 3.5,
            range: [0, 20, 0.1],
            widget: "slider",
        }
        input "output_path": "path" {
            description: "Optional destination path for a single PNG output.",
            required: false,
            widget: "file_save",
            artifact: "image",
        }
        input "output_template": "path" {
            description: "Optional output path template for multi-image generation.",
            required: false,
            widget: "file_save",
            artifact: "image",
        }
        input "model": "text" {
            description: "Optional FLUX model variant id.",
            required: false,
            widget: "model_select",
            model: "flux_model",
        }
        output "image": "artifact" {
            description: "First generated image artifact metadata.",
            artifact: "image",
            model: "flux_model",
        }
        output "image_path": "path" {
            description: "Path to the first generated PNG image.",
            artifact: "image",
            model: "flux_model",
        }
        output "images": "artifact[]" {
            description: "All generated image artifacts.",
            artifact: "image",
            model: "flux_model",
        }
        output "image_paths": "path[]" {
            description: "Paths to all generated PNG images.",
            artifact: "image",
            model: "flux_model",
        }
    }
    .builtin_runtime("flux_runtime", "lightflow.image.generate", "runner.v1")
    .hf_model(
        "flux_model",
        "flux2-klein-q4-k-m",
        "text-to-image",
        "gguf",
        "unsloth/FLUX.2-klein-9B-GGUF",
        "flux-2-klein-9b-Q4_K_M.gguf",
    )
    .hf_model(
        "flux_model",
        "flux2-klein-q3-k-m",
        "text-to-image",
        "gguf",
        "unsloth/FLUX.2-klein-9B-GGUF",
        "flux-2-klein-9b-Q3_K_M.gguf",
    )
    .hf_model(
        "flux_model",
        "flux2-klein-q5-k-m",
        "text-to-image",
        "gguf",
        "unsloth/FLUX.2-klein-9B-GGUF",
        "flux-2-klein-9b-Q5_K_M.gguf",
    )
    .hf_model(
        "flux_model",
        "flux2-klein-q8",
        "text-to-image",
        "gguf",
        "unsloth/FLUX.2-klein-9B-GGUF",
        "flux-2-klein-9b-Q8_0.gguf",
    )
    .hf_model(
        "llm_model",
        "qwen3-8b-q4-k-m",
        "language-model",
        "gguf",
        "unsloth/Qwen3-8B-GGUF",
        "Qwen3-8B-Q4_K_M.gguf",
    )
    .hf_model(
        "vae_model",
        "flux2-vae",
        "vae",
        "safetensors",
        "black-forest-labs/FLUX.2-dev",
        "vae/diffusion_pytorch_model.safetensors",
    )
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
        lightflow_flux_runtime::Task::TextToImage,
        inputs,
        models,
        &format!("lightflow.flux_text_to_image.source.fnv1a64:{SOURCE_DIGEST:016x}"),
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
