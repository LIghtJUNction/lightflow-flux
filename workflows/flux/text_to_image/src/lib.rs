use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow!()
        .name("FLUX Text To Image")
        .description("Generate an image from a prompt with FLUX.2 klein GGUF models.")
        .input("prompt", "text")
        .input_description(
            "prompt",
            "Prompt text sent to the FLUX image generation runtime.",
        )
        .input_required("prompt", true)
        .input_widget("prompt", "prompt")
        .input("negative", "text")
        .input_description("negative", "Optional negative prompt.")
        .input_required("negative", false)
        .input_widget("negative", "textarea")
        .input("width", "integer")
        .input_description("width", "Output width in pixels.")
        .input_required("width", false)
        .input_default_json("width", "1024")
        .input_range("width", 64.0, 2048.0, 8.0)
        .input_widget("width", "number")
        .input("height", "integer")
        .input_description("height", "Output height in pixels.")
        .input_required("height", false)
        .input_default_json("height", "1024")
        .input_range("height", 64.0, 2048.0, 8.0)
        .input_widget("height", "number")
        .input("seed", "integer")
        .input_description("seed", "Optional sampling seed.")
        .input_required("seed", false)
        .input_widget("seed", "number")
        .input("count", "integer")
        .input_description("count", "Number of images to generate.")
        .input_required("count", false)
        .input_default_json("count", "1")
        .input_range("count", 1.0, 16.0, 1.0)
        .input_widget("count", "number")
        .input("steps", "integer")
        .input_description("steps", "Denoising step count.")
        .input_required("steps", false)
        .input_default_json("steps", "20")
        .input_range("steps", 1.0, 80.0, 1.0)
        .input_widget("steps", "slider")
        .input("guidance", "number")
        .input_description("guidance", "Prompt guidance scale.")
        .input_required("guidance", false)
        .input_default_json("guidance", "3.5")
        .input_range("guidance", 0.0, 20.0, 0.1)
        .input_widget("guidance", "slider")
        .input("output_path", "path")
        .input_description(
            "output_path",
            "Optional destination path for a single PNG output.",
        )
        .input_required("output_path", false)
        .input_widget("output_path", "file_save")
        .input_artifact_kind("output_path", "image")
        .input("output_template", "path")
        .input_description(
            "output_template",
            "Optional output path template for multi-image generation.",
        )
        .input_required("output_template", false)
        .input_widget("output_template", "file_save")
        .input_artifact_kind("output_template", "image")
        .input("model", "text")
        .input_description("model", "Optional FLUX model variant id.")
        .input_required("model", false)
        .input_widget("model", "model_select")
        .input_model_requirement("model", "flux_model")
        .output("image", "artifact")
        .output_description("image", "First generated image artifact metadata.")
        .output_artifact_kind("image", "image")
        .output_model_requirement("image", "flux_model")
        .output("image_path", "path")
        .output_description("image_path", "Path to the first generated PNG image.")
        .output_artifact_kind("image_path", "image")
        .output_model_requirement("image_path", "flux_model")
        .output("images", "artifact[]")
        .output_description("images", "All generated image artifacts.")
        .output_artifact_kind("images", "image")
        .output_model_requirement("images", "flux_model")
        .output("image_paths", "path[]")
        .output_description("image_paths", "Paths to all generated PNG images.")
        .output_artifact_kind("image_paths", "image")
        .output_model_requirement("image_paths", "flux_model")
        .runtime("flux_runtime", "lightflow.image.generate")
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
