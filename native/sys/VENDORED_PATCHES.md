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
