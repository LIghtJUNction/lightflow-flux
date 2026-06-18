---
name: LightFlow FLUX Preview Text To Image
description: Use this skill when working with the lightflow.flux.preview_text_to_image deterministic preview workflow or testing FLUX pipelines without a model backend.
version: 0.1.0
---

# LightFlow FLUX Preview Text To Image

Use `lightflow.flux.preview_text_to_image` as a deterministic preview fallback for tests and low-cost pipeline checks.

## Workflow

- Workflow id: `lightflow.flux.preview_text_to_image`
- Inputs: `use_flux`, `prompt`, `negative`, `width`, `height`, `seed`, `output_path`.
- Outputs: `image`, `image_path`.
- Built-in runtime: `lightflow.image.generate` with engine `builtin.preview.v1`.

## Usage

```bash
lfw run lightflow.flux.preview_text_to_image \
  -i prompt='"preview image"' \
  -i width=512 \
  -i height=512 \
  -i output_path='"out/preview.png"'
```
