---
name: LightFlow FLUX Text To Image Router
description: Use this skill when working with the lightflow.flux.text_to_image_router workflow or routing image generation between real FLUX and preview runtimes.
version: 0.1.0
---

# LightFlow FLUX Text To Image Router

Use `lightflow.flux.text_to_image_router` to route generation to the real FLUX workflow when `use_flux=true`, or to the deterministic preview workflow otherwise.

## Workflow

- Workflow id: `lightflow.flux.text_to_image_router`
- Inputs: `use_flux`, `prompt`, `negative`, `width`, `height`, `seed`, `steps`, `guidance`, `output_path`.
- Outputs: `image`, `image_path`.
- Branches: `lightflow.flux.text_to_image` or `lightflow.flux.preview_text_to_image`.
- Node Schema: output ports are `image` artifacts; `use_flux` is a toggle; generation controls expose editor ranges.

## Routing

Use `use_flux=false` for fast local preview checks. Use `use_flux=true` only after syncing model requirements for `lightflow.flux.text_to_image`.

## Usage

```bash
lfw run lightflow.flux.text_to_image_router \
  -i use_flux=false \
  -i prompt='"fast preview"' \
  -i output_path='"out/router-preview.png"'
```

## Validation

```bash
lfw node test lightflow.flux.text_to_image_router
```
