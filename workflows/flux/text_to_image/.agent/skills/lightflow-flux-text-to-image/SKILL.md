---
name: LightFlow FLUX Text To Image
description: Use this skill when working with the lightflow.flux.text_to_image workflow, syncing FLUX models, or generating images from prompts.
version: 0.1.0
---

# LightFlow FLUX Text To Image

Use `lightflow.flux.text_to_image` to generate images from text prompts through the LightFlow FLUX runtime.

## Workflow

- Workflow id: `lightflow.flux.text_to_image`
- Inputs: `prompt`, `negative`, `width`, `height`, `seed`, `count`, `steps`, `guidance`, `output_path`, `output_template`, `model`.
- Outputs: `image`, `image_path`, `images`, `image_paths`.
- Runtime capability: `lightflow.image.generate`.
- Model requirements: `flux_model`, `llm_model`, and `vae_model`.
- Node Schema: image outputs are `image` artifacts; `model` is bound to `flux_model`; width, height, count, steps, and guidance include editor ranges.

## Runtime

Run `lfw sync lightflow.flux.text_to_image --auto-model --apply` before a real generation run. The runtime uses `flux-native` when the LightFlow binary is built with that feature, otherwise it can delegate to `LIGHTFLOW_FLUX_RUNNER`.

This project includes a stable-diffusion.cpp adapter. Configure it before real backend runs:

```bash
export LIGHTFLOW_SD_CLI=/path/to/stable-diffusion.cpp/build/bin/sd-cli
export LIGHTFLOW_FLUX_RUNNER="$PWD/scripts/sd-cli-flux-runner"
```

The adapter validates `sd-cli`, model files, and source assets before loading models. `count > 1` currently calls the runner once per image, so smoke tests should use `count=1`, small dimensions, and `steps=1`.

## Usage

```bash
lfw sync lightflow.flux.text_to_image --auto-model --apply
lfw run lightflow.flux.text_to_image \
  -i prompt='"a quiet lake at sunrise"' \
  -i width=768 \
  -i height=768 \
  -i output_path='"out/lake.png"'
```

Fast real-backend smoke test:

```bash
lfw run lightflow.flux.text_to_image \
  -i prompt='"real FLUX smoke test, small red cube"' \
  -i width=128 \
  -i height=96 \
  -i seed=11 \
  -i steps=1 \
  -i guidance=1.0 \
  -i output_path='"/tmp/lightflow-flux-test/runner-real.png"'
```

## Validation

```bash
lfw node test lightflow.flux.text_to_image
```
