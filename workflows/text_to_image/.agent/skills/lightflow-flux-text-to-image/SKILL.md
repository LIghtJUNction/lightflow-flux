---
name: LightFlow FLUX Text To Image
description: Use this skill when working with the lightflow.flux_text_to_image workflow, syncing FLUX models, or generating images from prompts.
version: 0.1.0
---

# LightFlow FLUX Text To Image

Use `lightflow.flux_text_to_image` to generate images from text prompts through the LightFlow FLUX runtime.

The source definition now uses the canonical
`workflow! { name, description, ... }` form. This source-only DSL migration
does not change execution behavior or the input/output contract below.

## Workflow

- Workflow id: `lightflow.flux_text_to_image`
- Inputs: `prompt`, `negative`, `width`, `height`, `seed`, `count`, `steps`, `guidance`, `output_path`, `output_template`, `model`.
- Outputs: `image`, `image_path`, `images`, `image_paths`.
- Runtime capability: `lightflow.image.generate`.
- Runtime engine: `runner.v1`.
- Model requirements: `flux_model`, `llm_model`, and `vae_model`.
- Node Schema: image outputs are `image` artifacts; `model` is bound to `flux_model`; width, height, count, steps, and guidance include editor ranges.

## Runtime

Run `lfw sync lightflow.flux_text_to_image --auto-model --apply` before a real
generation run. Execution belongs to the published workflow package and its
`lightflow-flux-runtime` dependency; the LightFlow host has no FLUX business
implementation. Set `LIGHTFLOW_FLUX_BACKEND=native` for the package-owned CPU
backend. The workflow crate enables its validated Cargo `native` feature; the
root `lfw` binary needs no FLUX feature. Models come from verified
`runner.v1` bindings resolved from `lfw.lock`. Each call is a separate runner
process, so cross-request residency is not guaranteed.

This project also includes a stable-diffusion.cpp adapter. Configure it with
`LIGHTFLOW_FLUX_BACKEND=external`:

```bash
export LIGHTFLOW_SD_CLI=/path/to/stable-diffusion.cpp/build/bin/sd-cli
export LIGHTFLOW_FLUX_RUNNER="$PWD/scripts/sd-cli-flux-runner"
```

The adapter validates `sd-cli`, model files, and source assets before loading models. The external fallback is process-per-call; use native execution for performance work.

## Usage

```bash
lfw sync lightflow.flux_text_to_image --auto-model --apply
lfw run lightflow.flux_text_to_image \
  -i prompt='"a quiet lake at sunrise"' \
  -i width=768 \
  -i height=768 \
  -i output_path='"out/lake.png"'
```

Fast real-backend smoke test:

```bash
lfw run lightflow.flux_text_to_image \
  -i prompt='"real FLUX smoke test, small red cube"' \
  -i width=128 \
  -i height=96 \
  -i seed=11 \
  -i steps=1 \
  -i guidance=1.0 \
  -i output_path='"out/runner-real.png"'
```

Outputs and expanded templates must be unique project-relative paths.

## Validation

```bash
lfw node test lightflow.flux_text_to_image
```
## API Usage

Start `lfw serve`, then call the workflow through the shared HTTP run contract. Adjust `inputs` to match the workflow contract above.

```bash
curl -sS -X POST http://127.0.0.1:5174/workflows/lightflow.flux_text_to_image/run \
  -H 'content-type: application/json' \
  -d '{"inputs":{}}'
```
