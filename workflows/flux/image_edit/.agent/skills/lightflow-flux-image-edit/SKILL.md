---
name: LightFlow FLUX Image Edit
description: Use this skill when working with the lightflow.flux.image_edit workflow, selecting FLUX edit models, or configuring prompt-based image edits.
version: 0.1.0
---

# LightFlow FLUX Image Edit

Use `lightflow.flux.image_edit` to declare prompt-guided image editing with FLUX model requirements.

## Workflow

- Workflow id: `lightflow.flux.image_edit`
- Required inputs: `image_path`, `prompt`.
- Common inputs: `negative`, `strength`, `seed`, `count`, `steps`, `guidance`, `output_path`, `output_template`, `model`.
- Outputs: `image`, `image_path`, `images`, `image_paths`.
- Runtime capability: `lightflow.image.edit`.

## Setup

```bash
lfw sync lightflow.flux.image_edit --auto-model --apply
```

This sync selects the FLUX.2 klein GGUF model, the Qwen3 LLM GGUF, and the
FLUX.2 VAE. The GGUF file is the quantized diffusion model; the VAE and LLM are
separate runtime assets required to decode images and encode prompts.

Build LightFlow with the native FLUX backend when possible:

```bash
cargo run --manifest-path /path/to/LightFlow/Cargo.toml --features flux-native --bin lfw -- \
  run lightflow.flux.image_edit ...
```

Builds without `flux-native` can set `LIGHTFLOW_FLUX_RUNNER` to
`scripts/sd-cli-flux-runner`. The fallback runner receives
`--task image-edit`, `--image <image_path>`, `--strength <number>`, prompt
fields, sampling settings, output path, and the locked `flux_model`,
`llm_model`, and `vae_model` paths from `lfw.lock`.

Use `count` to produce multiple edited variants. Seeds increment from the base
`seed`. Use `output_template` with placeholders `{index}`, `{index0}`,
`{index:03}`, `{seed}`, and `{workflow_id}`. If no output path is provided,
images default to the user's XDG Pictures directory under
`lightflow/<workflow_id>/`.

## Example

```bash
lfw run lightflow.flux.image_edit \
  -i image_path='"input/source.png"' \
  -i prompt='"make the lighting warmer"' \
  -i strength=0.55 \
  -i count=3 \
  -i output_template='"out/edit-{index:03}.png"'
```
