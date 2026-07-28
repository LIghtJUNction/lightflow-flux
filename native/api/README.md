# LightFlow FLUX native

`lightflow-flux-native` is the Rust wrapper used by the package-owned
LightFlow FLUX runtime. It is not the upstream `diffusion-rs` crate and does
not claim upstream release or platform support.

The implementation is derived from the MIT-licensed `diffusion-rs` API and
wraps the vendored `stable-diffusion.cpp` FFI from
`lightflow-flux-native-sys`. LightFlow carries a reduced FLUX.2 Klein build
and releases generated image arrays through the upstream `free_sd_images`
ownership API.

CPU is the default native path. Optional crate features expose supported
accelerators when their platform toolchains are installed. See the top-level
README and the sys crate's `VENDORED_PATCHES.md` for the workflow contract and
vendored-source policy.
