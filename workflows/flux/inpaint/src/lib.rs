use lightflow::workflow::*;

pub fn define() -> WorkflowSpec {
    workflow("lightflow.flux.inpaint")
        .version("0.1.0")
        .name("FLUX Inpaint")
        .description("Perform masked local repainting with a canonical black/white mask image.")
        .input("image_path", "path")
        .input("mask_path", "path")
        .input("prompt", "text")
        .input("negative", "text")
        .input("strength", "number")
        .input("feather_px", "integer")
        .input("dilate_px", "integer")
        .input("invert_mask", "boolean")
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
        .runtime("flux_runtime", "lightflow.image.inpaint")
        .hf_model(
            "flux_model",
            "flux2-klein-q4-k-m",
            "image-inpaint",
            "gguf",
            "unsloth/FLUX.2-klein-9B-GGUF",
            "flux-2-klein-9b-Q4_K_M.gguf",
        )
        .hf_model(
            "flux_model",
            "flux2-klein-q3-k-m",
            "image-inpaint",
            "gguf",
            "unsloth/FLUX.2-klein-9B-GGUF",
            "flux-2-klein-9b-Q3_K_M.gguf",
        )
        .hf_model(
            "flux_model",
            "flux2-klein-q5-k-m",
            "image-inpaint",
            "gguf",
            "unsloth/FLUX.2-klein-9B-GGUF",
            "flux-2-klein-9b-Q5_K_M.gguf",
        )
        .hf_model(
            "flux_model",
            "flux2-klein-q8",
            "image-inpaint",
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
