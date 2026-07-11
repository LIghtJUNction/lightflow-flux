---
name: LightFlow FLUX Text To Image
description: Use this skill when working with the lightflow.flux_text_to_image workflow, syncing FLUX models, or generating images from prompts.
version: 0.1.0
---

# LightFlow FLUX Text To Image

Use `lightflow.flux_text_to_image` to generate images from text prompts through the LightFlow FLUX runtime.

## Workflow

- Workflow id: `lightflow.flux_text_to_image`
- Inputs: `prompt`, `negative`, `width`, `height`, `seed`, `count`, `steps`, `guidance`, `output_path`, `output_template`, `model`.
- Outputs: `image`, `image_path`, `images`, `image_paths`.
- Runtime capability: `lightflow.image.generate`.
- Model requirements: `flux_model`, `llm_model`, and `vae_model`.
- Node Schema: image outputs are `image` artifacts; `model` is bound to `flux_model`; width, height, count, steps, and guidance include editor ranges.

## Runtime

Run `lfw sync lightflow.flux_text_to_image --auto-model --apply` before a real generation run. The preferred runtime is LightFlow built with `--features flux-native`. Native text-to-image keeps a loaded FLUX/Qwen/VAE session in the LightFlow process, reuses it for later images with the same locked model paths, and sends `count > 1` requests as one native batch call.

For ComfyUI-style residency across requests, use a long-lived LightFlow process such as `lfw serve`; one-shot `lfw run` commands release the native session when the process exits.

This project also includes a stable-diffusion.cpp adapter for builds without `flux-native`. Configure it only when using the external fallback:

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
cargo run --manifest-path ../LightFlow/Cargo.toml --features flux-native --bin lfw -- \
  run lightflow.flux_text_to_image \
  -i prompt='"real FLUX smoke test, small red cube"' \
  -i width=128 \
  -i height=96 \
  -i seed=11 \
  -i steps=1 \
  -i guidance=1.0 \
  -i output_path='"/tmp/lightflow-flux-test/runner-real.png"'
```

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
