use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
        name: "FLUX Text To Image Router",
        description: "Route text-to-image generation to the real FLUX runtime or a preview fallback.",
        input "use_flux": "boolean" {
            description: "Route to real FLUX when true, or deterministic preview when false.",
            required: false,
            default: false,
            widget: "toggle",
        }
        input "prompt": "text" {
            description: "Prompt text forwarded to the selected branch.",
            required: true,
            widget: "prompt",
        }
        input "negative": "text" {
            description: "Optional negative prompt forwarded to the selected branch.",
            required: false,
            widget: "textarea",
        }
        input "width": "integer" {
            description: "Output width in pixels.",
            required: false,
            default: 512,
            range: [64, 2048, 8],
            widget: "number",
        }
        input "height": "integer" {
            description: "Output height in pixels.",
            required: false,
            default: 512,
            range: [64, 2048, 8],
            widget: "number",
        }
        input "seed": "integer" {
            description: "Optional sampling seed forwarded to the selected branch.",
            required: false,
            widget: "number",
        }
        input "steps": "integer" {
            description: "Denoising step count forwarded to the selected branch.",
            required: false,
            default: 20,
            range: [1, 80, 1],
            widget: "slider",
        }
        input "guidance": "number" {
            description: "Prompt guidance scale forwarded to the selected branch.",
            required: false,
            default: 3.5,
            range: [0, 20, 0.1],
            widget: "slider",
        }
        input "output_path": "path" {
            description: "Optional destination PNG path.",
            required: false,
            widget: "file_save",
            artifact: "image",
        }
        output "image": "artifact" {
            description: "Image artifact metadata from the selected branch.",
            artifact: "image",
        }
        output "image_path": "path" {
            description: "Path to the generated PNG image.",
            artifact: "image",
        }
    }
    .if_node(
        "route",
        "use_flux",
        true,
        "lightflow.flux_text_to_image",
        "lightflow.flux_preview_text_to_image",
    )
    .build()
}
