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
        .input("steps", "integer")
        .input("guidance", "number")
        .input("output_path", "path")
        .input("model", "text")
        .output("image", "artifact")
        .output("image_path", "path")
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
            "ae_model",
            "flux-ae",
            "vae",
            "safetensors",
            "black-forest-labs/FLUX.1-dev",
            "ae.safetensors",
        )
        .hf_model(
            "clip_model",
            "clip-l",
            "text-encoder",
            "safetensors",
            "comfyanonymous/flux_text_encoders",
            "clip_l.safetensors",
        )
        .hf_model(
            "t5_model",
            "t5xxl-fp8",
            "text-encoder",
            "safetensors",
            "comfyanonymous/flux_text_encoders",
            "t5xxl_fp8_e4m3fn.safetensors",
        )
        .hf_model(
            "t5_model",
            "t5xxl-fp16",
            "text-encoder",
            "safetensors",
            "comfyanonymous/flux_text_encoders",
            "t5xxl_fp16.safetensors",
        )
        .build()
}
