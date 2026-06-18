---
name: LightFlow FLUX Text To Image
description: This skill should be used when working with the lightflow.flux.text_to_image workflow, generating images from prompts, selecting FLUX models, or composing text-to-image output with other LightFlow workflows.
version: 0.1.0
---

# LightFlow FLUX Text To Image

Use this skill to generate an image from a text prompt with `lightflow.flux.text_to_image`.

## Workflow

- Workflow id: `lightflow.flux.text_to_image`
- Required input `prompt`: text prompt.
- Common inputs: `negative`, `width`, `height`, `seed`, `count`, `steps`, `guidance`, `output_path`, `output_template`, `model`.
- Output `image_path`: generated PNG path.
- Output `image`: image artifact metadata.
- Batch outputs: `image_paths` and `images`.

## Setup

Run model sync before using a FLUX runtime:

```bash
lfw sync lightflow.flux.text_to_image --auto-model --apply
```

This sync selects the FLUX.2 klein GGUF model, the Qwen3 LLM GGUF, and the
FLUX.2 VAE. The GGUF file is the quantized diffusion model; the VAE and LLM are
separate runtime assets required to decode images and encode prompts.

This workflow declares the FLUX runtime capability. A LightFlow binary built
with `--features flux-native` runs it through the native Rust backend. Builds
without that feature can set `LIGHTFLOW_FLUX_RUNNER` to an executable that
writes a PNG to `--output` and accepts LightFlow's FLUX runner arguments:

```text
--task <text-to-image|image-edit|inpaint>
--prompt <text>
--negative <text>
--width <pixels>
--height <pixels>
--seed <integer>
--steps <integer>
--guidance <number>
--cfg-scale <number>
--strength <number>
--output <png-path>
--flux-model <path>
--llm-model <path>
--vae-model <path>
```

The backend owns sampling. LightFlow owns workflow resolution, model path
lookup from `lfw.lock`, batch scheduling, and artifact metadata.

Use `count` to generate multiple images in one workflow call. Seeds increment
per image from the base `seed`. Use `output_template` to name files; supported
placeholders are `{index}`, `{index0}`, `{index:03}`, `{seed}`, and
`{workflow_id}`. If no output path is provided, generated images are written to
the user's XDG Pictures directory under `lightflow/<workflow_id>/`.

This project includes `scripts/sd-cli-flux-runner`, an adapter for
stable-diffusion.cpp. Set `LIGHTFLOW_SD_CLI` to the built `sd-cli` executable
and `LIGHTFLOW_FLUX_RUNNER="$PWD/scripts/sd-cli-flux-runner"`.

## Example

```bash
lfw run lightflow.flux.text_to_image \
  -i prompt='"a small cat photo"' \
  -i width=768 \
  -i height=768 \
  -i seed=42 \
  -i count=5 \
  -i output_template='"out/cat-{index:03}-{seed}.png"'
```
