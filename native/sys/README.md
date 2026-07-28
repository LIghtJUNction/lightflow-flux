# LightFlow FLUX native sys

This crate is the private C FFI build dependency for
`lightflow-flux-native`. It vendors the source needed for CPU and optional
accelerated FLUX execution so a published LightFlow workflow does not depend
on an untracked system checkout.

The implementation is based on MIT-licensed
[`stable-diffusion.cpp`](https://github.com/leejet/stable-diffusion.cpp) and
its MIT-licensed `ggml` dependency. See the packaged upstream license files
and [`VENDORED_PATCHES.md`](VENDORED_PATCHES.md) for the reduced-build policy.
WebP and WebM support are disabled and their sources are not distributed by
this crate.
