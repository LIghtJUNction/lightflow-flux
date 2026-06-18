---
name: LightFlow FLUX Text To Image Router
description: Use this skill when testing or running conditional routing between real FLUX text-to-image generation and the preview fallback.
version: 0.1.0
---

# LightFlow FLUX Text To Image Router

Use `lightflow.flux.text_to_image_router` to choose between the real FLUX text-to-image workflow and the preview fallback.

## Workflow

- Workflow id: `lightflow.flux.text_to_image_router`
- Set `use_flux=true` to route to `lightflow.flux.text_to_image`.
- Set `use_flux=false` to route to `lightflow.flux.preview_text_to_image`.
- Common inputs: `prompt`, `negative`, `width`, `height`, `seed`, `steps`, `guidance`, `output_path`.
- Outputs: `image`, `image_path`.

## Usage

```bash
lfw run lightflow.flux.text_to_image_router \
  -i use_flux=false \
  -i prompt='"a small cat photo"' \
  -i width=256 \
  -i height=256 \
  -i output_path='"out/router-preview.png"'
```
