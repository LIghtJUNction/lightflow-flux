use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow!()
        .name("FLUX Preview Text To Image")
        .description("Deterministic preview fallback for testing FLUX text-to-image pipelines.")
        .input("use_flux", "boolean")
        .input_description(
            "use_flux",
            "Compatibility flag used by router workflows; ignored by the preview runtime.",
        )
        .input_required("use_flux", false)
        .input_default_json("use_flux", "false")
        .input_widget("use_flux", "toggle")
        .input("prompt", "text")
        .input_description(
            "prompt",
            "Prompt text used to generate deterministic preview pixels.",
        )
        .input_required("prompt", true)
        .input_widget("prompt", "prompt")
        .input("negative", "text")
        .input_description(
            "negative",
            "Optional negative prompt preserved in artifact metadata.",
        )
        .input_required("negative", false)
        .input_widget("negative", "textarea")
        .input("width", "integer")
        .input_description("width", "Requested output width in pixels.")
        .input_required("width", false)
        .input_default_json("width", "512")
        .input_range("width", 64.0, 2048.0, 8.0)
        .input_widget("width", "number")
        .input("height", "integer")
        .input_description("height", "Requested output height in pixels.")
        .input_required("height", false)
        .input_default_json("height", "512")
        .input_range("height", 64.0, 2048.0, 8.0)
        .input_widget("height", "number")
        .input("seed", "integer")
        .input_description("seed", "Optional deterministic seed.")
        .input_required("seed", false)
        .input_widget("seed", "number")
        .input("output_path", "path")
        .input_description("output_path", "Optional destination PNG path.")
        .input_required("output_path", false)
        .input_widget("output_path", "file_save")
        .input_artifact_kind("output_path", "image")
        .output("image", "artifact")
        .output_description("image", "Generated preview image artifact metadata.")
        .output_artifact_kind("image", "image")
        .output("image_path", "path")
        .output_description("image_path", "Path to the generated preview PNG image.")
        .output_artifact_kind("image_path", "image")
        .builtin_runtime(
            "image_runtime",
            "lightflow.image.generate",
            "builtin.preview.v1",
        )
        .build()
}
