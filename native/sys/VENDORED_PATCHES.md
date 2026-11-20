# Vendored source policy

LightFlow packages a FLUX-only build of `stable-diffusion.cpp`.

- CPU support is always available; CUDA, HIP, Metal, SYCL, and Vulkan remain
  Cargo features.
- Examples, server, WebP, and WebM are disabled.
- Embedded Gemma, Gemma2, GPT-OSS, and Mistral vocabularies are removed because
  LightFlow does not expose the model families that consume them. Those
  constructors fail explicitly unless a vocabulary is supplied.
- CLIP, T5, UMT5, and Qwen vocabularies remain. LightFlow's FLUX.2 workflows
  expose FLUX.2 Klein, whose text encoder uses Qwen; the full Mistral-backed
  FLUX.2 family is not part of this package contract.

The upstream `stable-diffusion.cpp/LICENSE`, `ggml/LICENSE`, and
`thirdparty/LICENSE.darts_clone.txt` files are part of the crate.

## C header bindings

Bindgen parses the C headers with `-fno-builtin` so the declared `size_t`
parameters and return values keep their `usize` representation. Clang builtin
prototypes otherwise become fixed-width integers for `memcpy`, `memmove`,
`memset`, `memcmp`, `strlen`, and `bcmp`, which trigger
the `suspicious_runtime_symbol_definitions` lint with `-D warnings`. The bundled fallback bindings
use the same `usize` signatures. This option applies only to header parsing;
the native CMake build keeps its existing compiler optimization settings.
