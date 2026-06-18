---
name: LightFlow FLUX Inpaint
description: Use this skill when working with the lightflow.flux.inpaint workflow, masks, local repainting, or FLUX inpaint model sync.
version: 0.1.0
---

# LightFlow FLUX Inpaint

Use `lightflow.flux.inpaint` for masked local repainting with a PNG mask.

## Workflow

- Workflow id: `lightflow.flux.inpaint`
- Inputs: `image_path`, `mask_path`, `prompt`, `negative`, `strength`, `feather_px`, `dilate_px`, `invert_mask`, `seed`, `count`, `steps`, `guidance`, `output_path`, `output_template`, `model`.
- Outputs: `image`, `image_path`, `images`, `image_paths`.
- Runtime capability: `lightflow.image.inpaint`.
- Model requirements: `flux_model`, `llm_model`, and `vae_model`.

## Mask Contract

White mask pixels are repainted, black pixels are preserved, and gray values are soft weights. Set `invert_mask=true` to flip that convention.

## Usage

```bash
lfw sync lightflow.flux.inpaint --auto-model --apply
lfw run lightflow.flux.inpaint \
  -i image_path='"input.png"' \
  -i mask_path='"mask.png"' \
  -i prompt='"repair the scratched area"' \
  -i output_path='"out/inpaint.png"'
```
