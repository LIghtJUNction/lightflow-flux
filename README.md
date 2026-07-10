# lightflow-flux

LightFlow workflow project for FLUX.2 klein image generation and editing.

## Workflows

- `lightflow.flux_text_to_image`: prompt to image.
- `lightflow.flux_image_edit`: edit an existing image from a prompt.
- `lightflow.flux_inpaint`: masked local repainting.
- `lightflow.flux_preview_text_to_image`: deterministic preview fallback.
- `lightflow.flux_text_to_image_router`: route between real FLUX and preview generation.

Each workflow includes Node Schema v1 metadata for editor palettes and an
agent skill under `.agent/skills/<skill-name>/SKILL.md`.

All workflows recommend `unsloth/FLUX.2-klein-9B-GGUF` for the main FLUX
transformer, with `flux-2-klein-9b-Q4_K_M.gguf` as the default Q4
recommendation.

They also declare the support models needed by a LightFlow FLUX runner:

- `llm_model`: `unsloth/Qwen3-8B-GGUF` / `Qwen3-8B-Q4_K_M.gguf`
- `vae_model`: `black-forest-labs/FLUX.2-dev` / `vae/diffusion_pytorch_model.safetensors`

## Runtime Setup

The FLUX workflows load synced model paths from `lfw.lock`. A LightFlow binary
built with `--features flux-native` runs them through the native Rust backend.
For text-to-image, that native backend keeps a loaded FLUX/Qwen/VAE session in
the LightFlow process and reuses it for later images with the same model paths.
When `count > 1`, LightFlow sends text-to-image outputs to the native backend
as one batch request and then writes each generated image to the requested
output path.
Use a long-lived LightFlow process, such as `lfw serve`, when you want
ComfyUI-style model residency across requests.

Builds without `flux-native` can still delegate sampling to an external runner
by setting `LIGHTFLOW_FLUX_RUNNER` to an executable that accepts these
arguments:

```text
--task <text-to-image|image-edit|inpaint>
--prompt <text>
--negative <text>
--width <pixels>
--height <pixels>
--seed <integer>
--steps <integer>
--guidance <number>
--cfg-scale <number>
--strength <number>
--image <source-png>
--mask <mask-png>
--output <png-path>
--flux-model <path>
--llm-model <path>
--vae-model <path>
```

`--image` is required for `image-edit` and `inpaint`; `--mask` is required for
`inpaint`. The runner must write a PNG to `--output`. This keeps LightFlow's
workflow, lockfile, batch, and pipeline layers independent from a specific GPU
backend, so the runner can be stable-diffusion.cpp, a diffusion-rs worker, a
Python backend, or a future in-process runtime.

Keep the runtime path zero-copy at the LightFlow layer. Model requirements
resolve to Hugging Face cache paths in `lfw.lock`; workflows pass image, mask,
and output paths instead of embedding image bytes; the native FLUX backend uses
mmap for GGUF weights. Do not copy model files into this project.

This project includes a stable-diffusion.cpp adapter for fallback and
compatibility:

```bash
export LIGHTFLOW_SD_CLI=/path/to/stable-diffusion.cpp/build/bin/sd-cli
export LIGHTFLOW_FLUX_RUNNER="$PWD/scripts/sd-cli-flux-runner"
```

The adapter maps LightFlow's `--task`, `--image`, `--mask`, and `--strength`
arguments to stable-diffusion.cpp `sd-cli` options. It also checks that
`sd-cli`, the synced model files, and any source image or mask exist before
starting the expensive model load.

Run a small real-backend smoke test after syncing models:

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

For fast wiring checks that do not load FLUX models, use the preview branch:

```bash
lfw run lightflow.flux_text_to_image_router \
  -i use_flux=false \
  -i prompt='"router preview smoke test"' \
  -i width=128 \
  -i height=96 \
  -i output_path='"/tmp/lightflow-flux-test/router-preview.png"'
```

For router checks against the real backend, set `use_flux=true` and keep
`steps=1` for smoke tests. Native text-to-image reuses its loaded model session
inside the process and batches multi-image requests. The external fallback
runner is still process-per-call and should be treated as a compatibility path,
not the performance path.

## One-Step Model Setup

Use hardware-aware selection and download:

```bash
lfw sync lightflow.flux_text_to_image --auto-model --apply
lfw sync lightflow.flux_image_edit --auto-model --apply
lfw sync lightflow.flux_inpaint --auto-model --apply
```

On a typical memory-constrained machine, `--auto-model` selects the Q3/Q4 main
model plus the Qwen3 LLM and FLUX.2 VAE. Larger GPUs may select a higher
main-model quantization level; explicit choices always win:

```bash
lfw sync lightflow.flux_inpaint --model flux_model=flux2-klein-q4-k-m --apply
```

You can override individual support models too:

```bash
lfw sync lightflow.flux_inpaint \
  --model flux_model=flux2-klein-q4-k-m \
  --model llm_model=qwen3-8b-q4-k-m \
  --apply
```

## Mask Contract

`lightflow.flux_inpaint` expects `mask_path` to point at a PNG mask in the same
coordinate space as `image_path`:

- white pixels are repainted
- black pixels are preserved
- gray values are soft mask weights
- `invert_mask=true` flips the mask
- `feather_px` and `dilate_px` are runtime preprocessing hints

The workflow stores mask paths as runtime inputs, not as source-controlled
workflow data.

## Pipeline Example

Quote or escape the pipe token so your shell passes it to `lfw`:

```bash
lfw run lightflow.flux_text_to_image \
  -i prompt='"a small cat photo"' \
  -i width=768 \
  -i height=768 \
  -i seed=42 \
  -i output_path='"out/cat.png"' \
  '|' lightflow.image_invert \
  -i output_path='"out/cat-inverted.png"'
```

`lightflow.image_invert` is provided by the LightFlow standard workflow
collection, not by this FLUX workflow project.

## Batch Editing

For many images, write a JSONL queue:

```json
{"id":"image-001","workflow_id":"lightflow.flux_inpaint","inputs":{"image_path":"input/001.png","mask_path":"masks/001.png","prompt":"replace the scratched area","output_path":"out/001.png"}}
{"id":"image-002","workflow_id":"lightflow.flux_inpaint","inputs":{"image_path":"input/002.png","mask_path":"masks/002.png","prompt":"replace the scratched area","output_path":"out/002.png"}}
```

Then run with conservative GPU concurrency:

```bash
lfw batch run jobs.jsonl --max-gpu-jobs 1 --max-cpu-jobs auto --batch-size auto
```

Resume interrupted work:

```bash
lfw batch resume <run_id>
```

## Node Conformance

Validate workflow contracts before publishing changes:

```bash
lfw node test lightflow.flux_text_to_image
lfw node test lightflow.flux_image_edit
lfw node test lightflow.flux_inpaint
lfw node test lightflow.flux_preview_text_to_image
lfw node test lightflow.flux_text_to_image_router
```
