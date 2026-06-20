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

When `use_flux=true` and LightFlow is not built with `flux-native`, configure the external runner first:

```bash
export LIGHTFLOW_SD_CLI=/path/to/stable-diffusion.cpp/build/bin/sd-cli
export LIGHTFLOW_FLUX_RUNNER="$PWD/scripts/sd-cli-flux-runner"
```

Keep router smoke tests small. Real FLUX generation loads large model files, and multi-image generation in `lightflow.flux.text_to_image` is serial unless the runner provides persistence or batching.

## Usage

```bash
lfw run lightflow.flux.text_to_image_router \
  -i use_flux=false \
  -i prompt='"fast preview"' \
  -i output_path='"out/router-preview.png"'
```

Real backend smoke test:

```bash
lfw run lightflow.flux.text_to_image_router \
  -i use_flux=true \
  -i prompt='"router real FLUX smoke test, blue sphere"' \
  -i width=128 \
  -i height=96 \
  -i seed=13 \
  -i steps=1 \
  -i guidance=1.0 \
  -i output_path='"/tmp/lightflow-flux-test/router-real.png"'
```

## Validation

```bash
lfw node test lightflow.flux.text_to_image_router
```
