# lightflow-flux

Self-contained LightFlow workflow packages for FLUX.2 Klein image generation
and editing.

## Workflows

- `lightflow.flux_text_to_image`: prompt to image.
- `lightflow.flux_image_edit`: prompt-guided image editing.
- `lightflow.flux_inpaint`: masked repainting.
- `lightflow.flux_preview_text_to_image`: deterministic wiring preview.
- `lightflow.flux_text_to_image_router`: route between real and preview leaves.

Each workflow includes Node Schema metadata and an agent skill. Production
workflows declare these lockable model requirements:

- `flux_model`: `unsloth/FLUX.2-klein-9B-GGUF`
- `llm_model`: `unsloth/Qwen3-8B-GGUF`
- `vae_model`: `black-forest-labs/FLUX.2-dev`

The current package contract is FLUX.2 Klein with Qwen and the FLUX.2 VAE. It
does not include the full Mistral-backed FLUX.2 family.

## Runtime Contract

The LightFlow host verifies every required file against the current project's
`lfw.lock`, including actual size and SHA-256, then sends generic resolved
model bindings in the versioned `runner.v1` request. The FLUX package maps the
`flux_model`, `llm_model`, and `vae_model` requirement ids to its backend.
The optional `model` input must match the locked `flux_model` variant; it
cannot override the lock during a run.

Production workflow crates declare their own validated Cargo `native` runner
feature. The root `lfw` binary contains no FLUX implementation and needs no
FLUX feature. Each invocation is a separate Cargo runner process. A native
session may be reused inside one invocation, but `lfw serve` does not
currently guarantee cross-request model residency.

All output paths and expanded templates must be unique project-relative paths.
Every backend writes to same-directory staging files. The package validates
the complete PNG batch before an all-or-none commit and preserves existing
outputs if generation or validation fails.

## Native Backend

Sync models, select the native backend, and run:

```bash
lfw sync lightflow.flux_text_to_image --auto-model --apply
export LIGHTFLOW_FLUX_BACKEND=native
lfw run lightflow.flux_text_to_image \
  -i prompt='"a small red cube"' \
  -i width=128 \
  -i height=96 \
  -i steps=1 \
  -i output_path='"out/native-smoke.png"'
```

The first invocation may spend significant time compiling the vendored native
runtime. Native builds require CMake, a C/C++17 toolchain, and the development
tools for the selected platform backend. Clang/libclang regenerates bindings;
a bundled binding snapshot supports docs and environments without libclang.
CPU is the production workflow default. CUDA and Vulkan remain explicit
runtime crate features for controlled builds.

## External Backend

When native prerequisites are unavailable:

```bash
export LIGHTFLOW_FLUX_BACKEND=external
export LIGHTFLOW_FLUX_RUNNER=/path/to/flux-runner
```

The executable receives verified lock paths and these arguments:

```text
--task <text-to-image|image-edit|inpaint>
--prompt <text>
--negative <text>
--width <pixels>
--height <pixels>
--seed <integer>
--steps <integer>
--guidance <number>
--strength <number>
--image <source-png>
--mask <mask-png>
--output <png-path>
--flux-model <path>
--llm-model <path>
--vae-model <path>
```

`--image` is required for edit and inpaint; `--mask` is required for inpaint.
The runner must write a PNG to `--output`. Stdout and stderr are bounded, the
process group has a deadline, and failures are fail-closed.

This repository includes a stable-diffusion.cpp compatibility adapter:

```bash
export LIGHTFLOW_SD_CLI=/path/to/stable-diffusion.cpp/build/bin/sd-cli
export LIGHTFLOW_FLUX_RUNNER="$PWD/scripts/sd-cli-flux-runner"
```

## Model Setup

```bash
lfw sync lightflow.flux_text_to_image --auto-model --apply
lfw sync lightflow.flux_image_edit --auto-model --apply
lfw sync lightflow.flux_inpaint --auto-model --apply
```

Choose the main variant explicitly when needed:

```bash
lfw sync lightflow.flux_inpaint \
  --model flux_model=flux2-klein-q4-k-m \
  --model llm_model=qwen3-8b-q4-k-m \
  --apply
```

## Preview

Preview does not compile the native feature or require model locks:

```bash
lfw run lightflow.flux_text_to_image_router \
  -i use_flux=false \
  -i prompt='"router preview smoke test"' \
  -i width=128 \
  -i height=96 \
  -i output_path='"out/router-preview.png"'
```

## Inpaint Mask

The mask must share the source image coordinate space:

- white pixels are repainted;
- black pixels are preserved;
- gray pixels are soft weights;
- `invert_mask`, `feather_px`, and `dilate_px` adjust preprocessing.

## Verification

```bash
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo check -p lightflow-flux-runtime --features native
cargo package -p lightflow-flux-native-sys --allow-dirty
```
