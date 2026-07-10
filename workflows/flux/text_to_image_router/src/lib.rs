use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow!()
        .name("FLUX Text To Image Router")
        .description(
            "Route text-to-image generation to the real FLUX runtime or a preview fallback.",
        )
        .input("use_flux", "boolean")
        .input_description(
            "use_flux",
            "Route to real FLUX when true, or deterministic preview when false.",
        )
        .input_required("use_flux", false)
        .input_default_json("use_flux", "false")
        .input_widget("use_flux", "toggle")
        .input("prompt", "text")
        .input_description("prompt", "Prompt text forwarded to the selected branch.")
        .input_required("prompt", true)
        .input_widget("prompt", "prompt")
        .input("negative", "text")
        .input_description(
            "negative",
            "Optional negative prompt forwarded to the selected branch.",
        )
        .input_required("negative", false)
        .input_widget("negative", "textarea")
        .input("width", "integer")
        .input_description("width", "Output width in pixels.")
        .input_required("width", false)
        .input_default_json("width", "512")
        .input_range("width", 64.0, 2048.0, 8.0)
        .input_widget("width", "number")
        .input("height", "integer")
        .input_description("height", "Output height in pixels.")
        .input_required("height", false)
        .input_default_json("height", "512")
        .input_range("height", 64.0, 2048.0, 8.0)
        .input_widget("height", "number")
        .input("seed", "integer")
        .input_description(
            "seed",
            "Optional sampling seed forwarded to the selected branch.",
        )
        .input_required("seed", false)
        .input_widget("seed", "number")
        .input("steps", "integer")
        .input_description(
            "steps",
            "Denoising step count forwarded to the selected branch.",
        )
        .input_required("steps", false)
        .input_default_json("steps", "20")
        .input_range("steps", 1.0, 80.0, 1.0)
        .input_widget("steps", "slider")
        .input("guidance", "number")
        .input_description(
            "guidance",
            "Prompt guidance scale forwarded to the selected branch.",
        )
        .input_required("guidance", false)
        .input_default_json("guidance", "3.5")
        .input_range("guidance", 0.0, 20.0, 0.1)
        .input_widget("guidance", "slider")
        .input("output_path", "path")
        .input_description("output_path", "Optional destination PNG path.")
        .input_required("output_path", false)
        .input_widget("output_path", "file_save")
        .input_artifact_kind("output_path", "image")
        .output("image", "artifact")
        .output_description("image", "Image artifact metadata from the selected branch.")
        .output_artifact_kind("image", "image")
        .output("image_path", "path")
        .output_description("image_path", "Path to the generated PNG image.")
        .output_artifact_kind("image_path", "image")
        .if_node(
            "route",
            "use_flux",
            true,
            "lightflow.flux_text_to_image",
            "lightflow.flux_preview_text_to_image",
        )
        .build()
}
