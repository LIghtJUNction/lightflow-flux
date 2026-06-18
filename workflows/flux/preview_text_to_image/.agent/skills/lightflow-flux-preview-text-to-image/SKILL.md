---
name: LightFlow FLUX Preview Text To Image
description: Use this skill when testing FLUX text-to-image routing without loading the real FLUX runtime.
version: 0.1.0
---

# LightFlow FLUX Preview Text To Image

Use `lightflow.flux.preview_text_to_image` as a deterministic preview fallback for pipeline and control-flow tests.

## Workflow

- Workflow id: `lightflow.flux.preview_text_to_image`
- Inputs: `use_flux`, `prompt`, `negative`, `width`, `height`, `seed`, `output_path`.
- Outputs: `image`, `image_path`.
- Runtime: `builtin.preview.v1`.

## Usage

```bash
lfw run lightflow.flux.preview_text_to_image \
  -i prompt='"a small cat photo"' \
  -i width=256 \
  -i height=256 \
  -i output_path='"out/preview.png"'
```
