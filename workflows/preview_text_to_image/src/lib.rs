use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
        input "use_flux": "boolean" {
            description: "Compatibility flag used by router workflows; ignored by the preview runtime.",
            required: false,
            default: false,
            widget: "toggle",
        }
        input "prompt": "text" {
            description: "Prompt text used to generate deterministic preview pixels.",
            required: true,
            widget: "prompt",
        }
        input "negative": "text" {
            description: "Optional negative prompt preserved in artifact metadata.",
            required: false,
            widget: "textarea",
        }
        input "width": "integer" {
            description: "Requested output width in pixels.",
            required: false,
            default: 512,
            range: [64, 2048, 8],
            widget: "number",
        }
        input "height": "integer" {
            description: "Requested output height in pixels.",
            required: false,
            default: 512,
            range: [64, 2048, 8],
            widget: "number",
        }
        input "seed": "integer" {
            description: "Optional deterministic seed.",
            required: false,
            widget: "number",
        }
        input "output_path": "path" {
            description: "Optional destination PNG path.",
            required: false,
            widget: "file_save",
            artifact: "image",
        }
        output "image": "artifact" {
            description: "Generated preview image artifact metadata.",
            artifact: "image",
        }
        output "image_path": "path" {
            description: "Path to the generated preview PNG image.",
            artifact: "image",
        }
    }
        .name("FLUX Preview Text To Image")
        .description("Deterministic preview fallback for testing FLUX text-to-image pipelines.")
        .builtin_runtime("image_runtime", "lightflow.image.generate", "builtin.preview.v1")
        .build()
}
