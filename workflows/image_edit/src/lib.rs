use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
        input "image_path": "path" {
            description: "Source PNG image path.",
            required: true,
            widget: "image",
            artifact: "image",
        }
        input "prompt": "text" {
            description: "Edit prompt sent to the FLUX runtime.",
            required: true,
            widget: "prompt",
        }
        input "negative": "text" {
            description: "Optional negative prompt.",
            required: false,
            widget: "textarea",
        }
        input "strength": "number" {
            description: "Edit strength from source image preservation to full redraw.",
            required: false,
            default: 0.75,
            range: [0, 1, 0.01],
            widget: "slider",
        }
        input "seed": "integer" {
            description: "Optional sampling seed.",
            required: false,
            widget: "number",
        }
        input "count": "integer" {
            description: "Number of edited images to generate.",
            required: false,
            default: 1,
            range: [1, 16, 1],
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
            description: "Optional FLUX image-edit model variant id.",
            required: false,
            widget: "model_select",
            model: "flux_model",
        }
        output "image": "artifact" {
            description: "First edited image artifact metadata.",
            artifact: "image",
            model: "flux_model",
        }
        output "image_path": "path" {
            description: "Path to the first edited PNG image.",
            artifact: "image",
            model: "flux_model",
        }
        output "images": "artifact[]" {
            description: "All edited image artifacts.",
            artifact: "image",
            model: "flux_model",
        }
        output "image_paths": "path[]" {
            description: "Paths to all edited PNG images.",
            artifact: "image",
            model: "flux_model",
        }
    }
    .name("FLUX Image Edit")
    .description("Edit an input image with a FLUX prompt while preserving the source composition.")
    .runtime("flux_runtime", "lightflow.image.edit")
    .hf_model(
        "flux_model",
        "flux2-klein-q4-k-m",
        "image-edit",
        "gguf",
        "unsloth/FLUX.2-klein-9B-GGUF",
        "flux-2-klein-9b-Q4_K_M.gguf",
    )
    .hf_model(
        "flux_model",
        "flux2-klein-q3-k-m",
        "image-edit",
        "gguf",
        "unsloth/FLUX.2-klein-9B-GGUF",
        "flux-2-klein-9b-Q3_K_M.gguf",
    )
    .hf_model(
        "flux_model",
        "flux2-klein-q5-k-m",
        "image-edit",
        "gguf",
        "unsloth/FLUX.2-klein-9B-GGUF",
        "flux-2-klein-9b-Q5_K_M.gguf",
    )
    .hf_model(
        "flux_model",
        "flux2-klein-q8",
        "image-edit",
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
