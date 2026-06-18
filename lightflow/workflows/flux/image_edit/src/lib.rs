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
        .input("steps", "integer")
        .input("guidance", "number")
        .input("output_path", "path")
        .input("model", "text")
        .output("image", "artifact")
        .output("image_path", "path")
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
