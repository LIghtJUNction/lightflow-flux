use lightflow::workflow::*;

pub fn define() -> WorkflowSpec {
    workflow("lightflow.flux.preview_text_to_image")
        .version("0.1.0")
        .name("FLUX Preview Text To Image")
        .description("Deterministic preview fallback for testing FLUX text-to-image pipelines.")
        .input("use_flux", "boolean")
        .input("prompt", "text")
        .input("negative", "text")
        .input("width", "integer")
        .input("height", "integer")
        .input("seed", "integer")
        .input("output_path", "path")
        .output("image", "artifact")
        .output("image_path", "path")
        .builtin_runtime(
            "image_runtime",
            "lightflow.image.generate",
            "builtin.preview.v1",
        )
        .build()
}
