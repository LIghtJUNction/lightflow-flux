use lightflow::workflow::*;

pub fn define() -> WorkflowSpec {
    workflow("lightflow.flux.text_to_image_router")
        .version("0.1.0")
        .name("FLUX Text To Image Router")
        .description(
            "Route text-to-image generation to the real FLUX runtime or a preview fallback.",
        )
        .input("use_flux", "boolean")
        .input("prompt", "text")
        .input("negative", "text")
        .input("width", "integer")
        .input("height", "integer")
        .input("seed", "integer")
        .input("steps", "integer")
        .input("guidance", "number")
        .input("output_path", "path")
        .output("image", "artifact")
        .output("image_path", "path")
        .if_node(
            "route",
            "use_flux",
            true,
            "lightflow.flux.text_to_image",
            "lightflow.flux.preview_text_to_image",
        )
        .build()
}
