use lightflow::workflow::*;

pub fn define() -> WorkflowSpec {
    workflow("lightflow.flux.image_edit")
        .version("0.1.0")
        .name("FLUX Image Edit")
        .description(
            "Edit an input image with a FLUX prompt while preserving the source composition.",
        )
        .input("image_path", "path")
        .input("prompt", "text")
        .input("negative", "text")
        .input("strength", "number")
        .input("seed", "integer")
        .input("count", "integer")
        .input("steps", "integer")
        .input("guidance", "number")
        .input("output_path", "path")
        .input("output_template", "path")
        .input("model", "text")
        .output("image", "artifact")
        .output("image_path", "path")
        .output("images", "artifact[]")
        .output("image_paths", "path[]")
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
