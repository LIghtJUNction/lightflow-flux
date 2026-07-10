use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow!()
        .name("FLUX Inpaint")
        .description("Perform masked local repainting with a canonical black/white mask image.")
        .input("image_path", "path")
        .input_description("image_path", "Source PNG image path.")
        .input_required("image_path", true)
        .input_widget("image_path", "image")
        .input_artifact_kind("image_path", "image")
        .input("mask_path", "path")
        .input_description("mask_path", "PNG mask path. White pixels are repainted, black pixels are preserved, and gray pixels are soft weights.")
        .input_required("mask_path", true)
        .input_widget("mask_path", "image")
        .input_artifact_kind("mask_path", "mask")
        .input("prompt", "text")
        .input_description("prompt", "Inpaint prompt sent to the FLUX runtime.")
        .input_required("prompt", true)
        .input_widget("prompt", "prompt")
        .input("negative", "text")
        .input_description("negative", "Optional negative prompt.")
        .input_required("negative", false)
        .input_widget("negative", "textarea")
        .input("strength", "number")
        .input_description("strength", "Inpaint strength inside the masked area.")
        .input_required("strength", false)
        .input_default_json("strength", "0.75")
        .input_range("strength", 0.0, 1.0, 0.01)
        .input_widget("strength", "slider")
        .input("feather_px", "integer")
        .input_description("feather_px", "Runtime preprocessing hint for mask edge feathering in pixels.")
        .input_required("feather_px", false)
        .input_default_json("feather_px", "0")
        .input_range("feather_px", 0.0, 256.0, 1.0)
        .input_widget("feather_px", "number")
        .input("dilate_px", "integer")
        .input_description("dilate_px", "Runtime preprocessing hint for mask dilation in pixels.")
        .input_required("dilate_px", false)
        .input_default_json("dilate_px", "0")
        .input_range("dilate_px", 0.0, 256.0, 1.0)
        .input_widget("dilate_px", "number")
        .input("invert_mask", "boolean")
        .input_description("invert_mask", "Invert the mask convention before sampling.")
        .input_required("invert_mask", false)
        .input_default_json("invert_mask", "false")
        .input_widget("invert_mask", "toggle")
        .input("seed", "integer")
        .input_description("seed", "Optional sampling seed.")
        .input_required("seed", false)
        .input_widget("seed", "number")
        .input("count", "integer")
        .input_description("count", "Number of inpainted images to generate.")
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
        .input_description("output_path", "Optional destination path for a single PNG output.")
        .input_required("output_path", false)
        .input_widget("output_path", "file_save")
        .input_artifact_kind("output_path", "image")
        .input("output_template", "path")
        .input_description("output_template", "Optional output path template for multi-image generation.")
        .input_required("output_template", false)
        .input_widget("output_template", "file_save")
        .input_artifact_kind("output_template", "image")
        .input("model", "text")
        .input_description("model", "Optional FLUX inpaint model variant id.")
        .input_required("model", false)
        .input_widget("model", "model_select")
        .input_model_requirement("model", "flux_model")
        .output("image", "artifact")
        .output_description("image", "First inpainted image artifact metadata.")
        .output_artifact_kind("image", "image")
        .output_model_requirement("image", "flux_model")
        .output("image_path", "path")
        .output_description("image_path", "Path to the first inpainted PNG image.")
        .output_artifact_kind("image_path", "image")
        .output_model_requirement("image_path", "flux_model")
        .output("images", "artifact[]")
        .output_description("images", "All inpainted image artifacts.")
        .output_artifact_kind("images", "image")
        .output_model_requirement("images", "flux_model")
        .output("image_paths", "path[]")
        .output_description("image_paths", "Paths to all inpainted PNG images.")
        .output_artifact_kind("image_paths", "image")
        .output_model_requirement("image_paths", "flux_model")
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
