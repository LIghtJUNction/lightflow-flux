---
name: LightFlow FLUX Image Edit
description: Use this skill when working with the lightflow.flux.image_edit workflow, prompt-guided image editing, or FLUX image-edit model sync.
version: 0.1.0
---

# LightFlow FLUX Image Edit

Use `lightflow.flux.image_edit` to edit an existing image from a text prompt while preserving source composition.

## Workflow

- Workflow id: `lightflow.flux.image_edit`
- Inputs: `image_path`, `prompt`, `negative`, `strength`, `seed`, `count`, `steps`, `guidance`, `output_path`, `output_template`, `model`.
- Outputs: `image`, `image_path`, `images`, `image_paths`.
- Runtime capability: `lightflow.image.edit`.
- Model requirements: `flux_model`, `llm_model`, and `vae_model`.

## Usage

```bash
lfw sync lightflow.flux.image_edit --auto-model --apply
lfw run lightflow.flux.image_edit \
  -i image_path='"input.png"' \
  -i prompt='"make the lighting softer"' \
  -i output_path='"out/edit.png"'
```
