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

## Usage

```bash
lfw sync lightflow.flux.text_to_image --auto-model --apply
lfw run lightflow.flux.text_to_image \
  -i prompt='"a quiet lake at sunrise"' \
  -i width=768 \
  -i height=768 \
  -i output_path='"out/lake.png"'
```
