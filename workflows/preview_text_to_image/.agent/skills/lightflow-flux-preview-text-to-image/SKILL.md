---
name: LightFlow FLUX Preview Text To Image
description: Use this skill when working with the lightflow.flux_preview_text_to_image deterministic preview workflow or testing FLUX pipelines without a model backend.
version: 0.1.0
---

# LightFlow FLUX Preview Text To Image

Use `lightflow.flux_preview_text_to_image` as a deterministic preview fallback for tests and low-cost pipeline checks.

The source definition now uses the canonical
`workflow! { name, description, ... }` form. This source-only DSL migration
does not change execution behavior or the input/output contract below.

## Workflow

- Workflow id: `lightflow.flux_preview_text_to_image`
- Inputs: `use_flux`, `prompt`, `negative`, `width`, `height`, `seed`, `output_path`.
- Outputs: `image`, `image_path`.
- Package-owned runtime: `lightflow.image.generate` with engine `runner.v1`.
- Node Schema: output ports are `image` artifacts; width and height include editor ranges; `use_flux` is kept for router compatibility.

## Runtime

Use this workflow for deterministic offline checks. The workflow package owns
the PNG generator, so the host has no preview-image callback. It does not
require synced FLUX models and is not quality-equivalent to a real FLUX backend.

## Usage

```bash
lfw run lightflow.flux_preview_text_to_image \
  -i prompt='"preview image"' \
  -i width=512 \
  -i height=512 \
  -i output_path='"out/preview.png"'
```

## Validation

```bash
lfw node test lightflow.flux_preview_text_to_image
```
## API Usage

Start `lfw serve`, then call the workflow through the shared HTTP run contract. Adjust `inputs` to match the workflow contract above.

```bash
curl -sS -X POST http://127.0.0.1:5174/workflows/lightflow.flux_preview_text_to_image/run \
  -H 'content-type: application/json' \
  -d '{"inputs":{}}'
```
