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
- Node Schema: `image_path` is an `image` artifact input; image outputs are `image` artifacts; `strength`, `steps`, `guidance`, and `count` expose editor ranges.

## Runtime

Run `lfw sync lightflow.flux.image_edit --auto-model --apply` before a real edit run. The runtime uses `flux-native` when available, otherwise `LIGHTFLOW_FLUX_RUNNER` receives `--task image-edit`, source image path, prompt, model paths, and output path.

## Usage

```bash
lfw sync lightflow.flux.image_edit --auto-model --apply
lfw run lightflow.flux.image_edit \
  -i image_path='"input.png"' \
  -i prompt='"make the lighting softer"' \
  -i output_path='"out/edit.png"'
```

## Validation

```bash
lfw node test lightflow.flux.image_edit
```
## API Usage

Start `lfw serve`, then call the workflow through the shared HTTP run contract. Adjust `inputs` to match the workflow contract above.

```bash
curl -sS -X POST http://127.0.0.1:5174/workflows/lightflow.flux.image_edit/run \
  -H 'content-type: application/json' \
  -d '{"inputs":{}}'
```
