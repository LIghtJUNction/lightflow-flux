---
name: LightFlow FLUX Text To Image Router
description: Use this skill when working with the lightflow.flux_text_to_image_router workflow or routing image generation between real FLUX and preview runtimes.
version: 0.1.0
---

# LightFlow FLUX Text To Image Router

Use `lightflow.flux_text_to_image_router` to route generation to the real FLUX workflow when `use_flux=true`, or to the deterministic preview workflow otherwise.

The source definition now uses the canonical
`workflow! { name, description, ... }` form. This source-only DSL migration
does not change execution behavior or the input/output contract below.

## Workflow

- Workflow id: `lightflow.flux_text_to_image_router`
- Inputs: `use_flux`, `prompt`, `negative`, `width`, `height`, `seed`, `steps`, `guidance`, `output_path`.
- Outputs: `image`, `image_path`.
- Branches: `lightflow.flux_text_to_image` or `lightflow.flux_preview_text_to_image`.
- Node Schema: output ports are `image` artifacts; `use_flux` is a toggle; generation controls expose editor ranges.

## Routing

Use `use_flux=false` for fast local preview checks. Use `use_flux=true` only after syncing model requirements for `lightflow.flux_text_to_image`.

When `use_flux=true`, the selected production leaf enables its own validated
Cargo `native` feature. The root LightFlow binary needs no FLUX feature and
cross-request model residency is not guaranteed.

To use the external fallback instead:

```bash
export LIGHTFLOW_SD_CLI=/path/to/stable-diffusion.cpp/build/bin/sd-cli
export LIGHTFLOW_FLUX_RUNNER="$PWD/scripts/sd-cli-flux-runner"
```

Keep router smoke tests small. Native execution is the performance path; the external fallback is process-per-call and should be used for compatibility checks.

## Usage

```bash
lfw run lightflow.flux_text_to_image_router \
  -i use_flux=false \
  -i prompt='"fast preview"' \
  -i output_path='"out/router-preview.png"'
```

Real backend smoke test:

```bash
export LIGHTFLOW_FLUX_BACKEND=native
lfw run lightflow.flux_text_to_image_router \
  -i use_flux=true \
  -i prompt='"router real FLUX smoke test, blue sphere"' \
  -i width=128 \
  -i height=96 \
  -i seed=13 \
  -i steps=1 \
  -i guidance=1.0 \
  -i output_path='"out/router-real.png"'
```

## Validation

```bash
lfw node test lightflow.flux_text_to_image_router
```
## API Usage

Start `lfw serve`, then call the workflow through the shared HTTP run contract. Adjust `inputs` to match the workflow contract above.

```bash
curl -sS -X POST http://127.0.0.1:5174/workflows/lightflow.flux_text_to_image_router/run \
  -H 'content-type: application/json' \
  -d '{"inputs":{}}'
```
