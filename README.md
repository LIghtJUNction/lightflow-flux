# lightflow-flux

LightFlow workflow project for FLUX.2 klein image generation and editing.

## Workflows

- `lightflow.flux.text_to_image`: prompt to image.
- `lightflow.flux.image_edit`: edit an existing image from a prompt.
- `lightflow.flux.inpaint`: masked local repainting.

All workflows recommend `unsloth/FLUX.2-klein-9B-GGUF` for the main FLUX
transformer, with `flux-2-klein-9b-Q4_K_M.gguf` as the default Q4
recommendation.

They also declare the small support models needed by typical FLUX runtimes:

- `ae_model`: `black-forest-labs/FLUX.1-dev` / `ae.safetensors`
- `clip_model`: `comfyanonymous/flux_text_encoders` / `clip_l.safetensors`
- `t5_model`: `comfyanonymous/flux_text_encoders` / `t5xxl_fp8_e4m3fn.safetensors`

## One-Step Model Setup

Use hardware-aware selection and download:

```bash
lfw sync lightflow.flux.text_to_image --auto-model --apply
lfw sync lightflow.flux.image_edit --auto-model --apply
lfw sync lightflow.flux.inpaint --auto-model --apply
```

On a typical memory-constrained machine, `--auto-model` selects the Q4_K_M
main model plus AE, CLIP-L, and the FP8 T5 encoder. Larger GPUs may select a
higher main-model quantization level; explicit choices always win:

```bash
lfw sync lightflow.flux.inpaint --model flux_model=flux2-klein-q4-k-m --apply
```

You can override individual support models too:

```bash
lfw sync lightflow.flux.inpaint \
  --model flux_model=flux2-klein-q4-k-m \
  --model t5_model=t5xxl-fp16 \
  --apply
```

## Mask Contract

`lightflow.flux.inpaint` expects `mask_path` to point at a PNG mask in the same
coordinate space as `image_path`:

- white pixels are repainted
- black pixels are preserved
- gray values are soft mask weights
- `invert_mask=true` flips the mask
- `feather_px` and `dilate_px` are runtime preprocessing hints

The workflow stores mask paths as runtime inputs, not as source-controlled
workflow data.

## Batch Editing

For many images, write a JSONL queue:

```json
{"id":"image-001","workflow_id":"lightflow.flux.inpaint","inputs":{"image_path":"input/001.png","mask_path":"masks/001.png","prompt":"replace the scratched area","output_path":"out/001.png"}}
{"id":"image-002","workflow_id":"lightflow.flux.inpaint","inputs":{"image_path":"input/002.png","mask_path":"masks/002.png","prompt":"replace the scratched area","output_path":"out/002.png"}}
```

Then run with conservative GPU concurrency:

```bash
lfw batch run jobs.jsonl --max-gpu-jobs 1 --max-cpu-jobs auto --batch-size auto
```

Resume interrupted work:

```bash
lfw batch resume <run_id>
```
