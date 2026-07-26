use super::native::NativeFluxBatchRequest;
use super::{FluxError, NativeResult};
use diffusion_rs_sys::{
    free_sd_images, generate_image, lora_apply_mode_t, sd_cache_params_init, sd_ctx_params_init,
    sd_get_default_sample_method, sd_get_default_scheduler, sd_guidance_params_t,
    sd_hires_params_init, sd_image_t, sd_img_gen_params_init, sd_pm_params_t,
    sd_sample_params_init, sd_set_progress_callback, sd_slg_params_t, sd_tiling_params_t,
    sd_vae_format_t,
};
use std::ffi::CString;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::ptr::null_mut;
use std::slice;
use std::sync::{Mutex, OnceLock};

static SESSION: OnceLock<Mutex<Option<NativeFluxSession>>> = OnceLock::new();

struct GeneratedImages {
    pointer: *mut sd_image_t,
    count: i32,
}

impl GeneratedImages {
    unsafe fn from_raw(pointer: *mut sd_image_t, count: i32) -> NativeResult<Self> {
        if pointer.is_null() {
            return Err(FluxError::Backend(
                "native FLUX text-to-image generation returned no image".to_owned(),
            ));
        }
        Ok(Self { pointer, count })
    }

    unsafe fn as_slice(&self) -> &[sd_image_t] {
        unsafe { slice::from_raw_parts(self.pointer, self.count as usize) }
    }
}

impl Drop for GeneratedImages {
    fn drop(&mut self) {
        unsafe {
            free_sd_images(self.pointer, self.count);
        }
    }
}

#[cfg(feature = "native-test-seam")]
pub(super) fn generate_test_image_with_ffi_ownership(
    output_path: &Path,
    width: i32,
    height: i32,
    seed: i64,
) -> NativeResult<()> {
    let width = u32::try_from(width)
        .map_err(|_| FluxError::Backend("native test width must be positive".to_owned()))?;
    let height = u32::try_from(height)
        .map_err(|_| FluxError::Backend("native test height must be positive".to_owned()))?;
    let len = width
        .checked_mul(height)
        .and_then(|pixels| pixels.checked_mul(3))
        .ok_or_else(|| FluxError::Backend("native test image dimensions overflowed".to_owned()))?
        as usize;

    unsafe {
        let data = libc::malloc(len).cast::<u8>();
        if data.is_null() {
            return Err(FluxError::Backend(
                "native test image allocation failed".to_owned(),
            ));
        }
        for (index, value) in slice::from_raw_parts_mut(data, len).iter_mut().enumerate() {
            *value = (index as u8).wrapping_add(seed as u8);
        }

        let images = libc::malloc(std::mem::size_of::<sd_image_t>()).cast::<sd_image_t>();
        if images.is_null() {
            libc::free(data.cast());
            return Err(FluxError::Backend(
                "native test image array allocation failed".to_owned(),
            ));
        }
        images.write(sd_image_t {
            width,
            height,
            channel: 3,
            data,
        });

        // GeneratedImages owns the C allocations and releases both through the
        // production free_sd_images FFI function after PNG encoding.
        let generated = GeneratedImages::from_raw(images, 1)?;
        write_native_png(generated.as_slice()[0], output_path)?;
    }
    Ok(())
}

pub(super) fn generate_text_to_image_with_cached_session(
    request: NativeFluxBatchRequest<'_>,
) -> NativeResult<()> {
    let mut session = SESSION
        .get_or_init(|| Mutex::new(None))
        .lock()
        .map_err(|_| FluxError::Backend("native FLUX session lock was poisoned".to_owned()))?;
    if request.output_paths.is_empty() {
        return Err(FluxError::Backend(
            "native FLUX batch generation requires at least one output path".to_owned(),
        ));
    }

    let key = NativeFluxSessionKey::from_request(&request);
    let reload = session
        .as_ref()
        .map(|session| session.key != key)
        .unwrap_or(true);
    if reload {
        *session = Some(NativeFluxSession::load(key.clone())?);
    }
    let session = session
        .as_mut()
        .ok_or_else(|| FluxError::Backend("native FLUX session was not loaded".to_owned()))?;

    let prompt = cstring("prompt", request.prompt)?;
    let negative = cstring("negative prompt", request.negative)?;
    let mut layers: Vec<i32> = Vec::new();

    unsafe {
        sd_set_progress_callback(None, null_mut());

        let sample_method = sd_get_default_sample_method(session.ctx);
        let scheduler = sd_get_default_scheduler(session.ctx, sample_method);
        let mut sample_params = std::mem::zeroed();
        sd_sample_params_init(&mut sample_params);
        sample_params.guidance = sd_guidance_params_t {
            txt_cfg: request.cfg_scale,
            img_cfg: request.cfg_scale,
            distilled_guidance: request.guidance,
            slg: sd_slg_params_t {
                layers: layers.as_mut_ptr(),
                layer_count: layers.len(),
                layer_start: 0.01,
                layer_end: 0.2,
                scale: 0.0,
            },
        };
        sample_params.sample_method = sample_method;
        sample_params.scheduler = scheduler;
        sample_params.sample_steps = request.steps;

        let mut cache = std::mem::zeroed();
        sd_cache_params_init(&mut cache);

        let mut hires = std::mem::zeroed();
        sd_hires_params_init(&mut hires);

        let mut params = std::mem::zeroed();
        sd_img_gen_params_init(&mut params);
        params.prompt = prompt.as_ptr();
        params.negative_prompt = negative.as_ptr();
        params.width = request.width;
        params.height = request.height;
        params.seed = request.seed;
        params.batch_count = i32::try_from(request.output_paths.len())
            .map_err(|_| FluxError::Backend("native FLUX batch count overflowed i32".to_owned()))?;
        params.sample_params = sample_params;
        params.strength = request.strength;
        params.init_image = sd_image_t {
            width: 0,
            height: 0,
            channel: 3,
            data: null_mut(),
        };
        params.mask_image = sd_image_t {
            width: request.width as u32,
            height: request.height as u32,
            channel: 1,
            data: null_mut(),
        };
        params.control_image = sd_image_t {
            width: 0,
            height: 0,
            channel: 3,
            data: null_mut(),
        };
        params.vae_tiling_params = sd_tiling_params_t {
            enabled: true,
            temporal_tiling: false,
            tile_size_x: 32,
            tile_size_y: 32,
            target_overlap: 0.5,
            rel_size_x: 0.0,
            rel_size_y: 0.0,
            extra_tiling_args: null_mut(),
        };
        params.cache = cache;
        params.hires = hires;
        params.pm_params = sd_pm_params_t {
            id_images: null_mut(),
            id_images_count: 0,
            id_embed_path: null_mut(),
            style_strength: 20.0,
        };

        let images =
            GeneratedImages::from_raw(generate_image(session.ctx, &params), params.batch_count)?;
        images
            .as_slice()
            .iter()
            .zip(request.output_paths)
            .try_for_each(|(image, output_path)| write_native_png(*image, output_path))?;
    }

    for output_path in request.output_paths {
        let bytes = std::fs::read(output_path).map_err(FluxError::from)?;
        if !bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
            return Err(FluxError::Backend(format!(
                "native FLUX generation completed but did not write a PNG: {}",
                output_path.display()
            )));
        }
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NativeFluxSessionKey {
    diffusion_model: PathBuf,
    llm_model: PathBuf,
    vae_model: PathBuf,
}

impl NativeFluxSessionKey {
    fn from_request(request: &NativeFluxBatchRequest<'_>) -> Self {
        Self {
            diffusion_model: request.diffusion_model.to_path_buf(),
            llm_model: request.llm_model.to_path_buf(),
            vae_model: request.vae_model.to_path_buf(),
        }
    }
}

struct NativeFluxSession {
    key: NativeFluxSessionKey,
    ctx: *mut diffusion_rs_sys::sd_ctx_t,
    _diffusion_model: CString,
    _llm_model: CString,
    _vae_model: CString,
}

unsafe impl Send for NativeFluxSession {}

impl NativeFluxSession {
    fn load(key: NativeFluxSessionKey) -> NativeResult<Self> {
        let diffusion_model = cstring_path("FLUX model", &key.diffusion_model)?;
        let llm_model = cstring_path("LLM model", &key.llm_model)?;
        let vae_model = cstring_path("VAE model", &key.vae_model)?;

        unsafe {
            let mut params = std::mem::zeroed();
            sd_ctx_params_init(&mut params);
            params.diffusion_model_path = diffusion_model.as_ptr();
            params.llm_path = llm_model.as_ptr();
            params.vae_path = vae_model.as_ptr();
            params.vae_format = sd_vae_format_t::SD_VAE_FORMAT_FLUX2;
            params.enable_mmap = true;
            params.flash_attn = true;
            params.lora_apply_mode = lora_apply_mode_t::LORA_APPLY_AUTO;

            let ctx = diffusion_rs_sys::new_sd_ctx(&params);
            if ctx.is_null() {
                return Err(FluxError::Backend(format!(
                    "failed to load native FLUX session for {}",
                    key.diffusion_model.display()
                )));
            }

            if !diffusion_rs_sys::sd_ctx_supports_image_generation(ctx) {
                diffusion_rs_sys::free_sd_ctx(ctx);
                return Err(FluxError::Backend(format!(
                    "native FLUX session does not support image generation: {}",
                    key.diffusion_model.display()
                )));
            }

            Ok(Self {
                key,
                ctx,
                _diffusion_model: diffusion_model,
                _llm_model: llm_model,
                _vae_model: vae_model,
            })
        }
    }
}

impl Drop for NativeFluxSession {
    fn drop(&mut self) {
        unsafe {
            diffusion_rs_sys::free_sd_ctx(self.ctx);
        }
    }
}

fn cstring(label: &str, value: &str) -> NativeResult<CString> {
    CString::new(value)
        .map_err(|_| FluxError::Backend(format!("{label} contains an interior NUL byte")))
}

fn cstring_path(label: &str, path: &Path) -> NativeResult<CString> {
    cstring(label, &path.display().to_string())
}

fn write_native_png(image: diffusion_rs_sys::sd_image_t, path: &Path) -> NativeResult<()> {
    if image.data.is_null() {
        return Err(FluxError::Backend(
            "native FLUX text-to-image generation returned null image data".to_owned(),
        ));
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(FluxError::from)?;
    }

    let len = image
        .width
        .checked_mul(image.height)
        .and_then(|pixels| pixels.checked_mul(image.channel))
        .ok_or_else(|| FluxError::Backend("native FLUX image dimensions overflowed".to_owned()))?
        as usize;
    let data = unsafe { slice::from_raw_parts(image.data, len) };
    let file = File::create(path).map_err(FluxError::from)?;
    let writer = BufWriter::new(file);
    let mut encoder = png::Encoder::new(writer, image.width, image.height);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_color(match image.channel {
        1 => png::ColorType::Grayscale,
        3 => png::ColorType::Rgb,
        4 => png::ColorType::Rgba,
        channel => {
            return Err(FluxError::Backend(format!(
                "native FLUX returned unsupported PNG channel count: {channel}"
            )));
        }
    });
    let mut png = encoder.write_header().map_err(|error| {
        FluxError::Backend(format!(
            "failed to write native FLUX PNG header for {}: {error}",
            path.display()
        ))
    })?;
    png.write_image_data(data).map_err(|error| {
        FluxError::Backend(format!(
            "failed to write native FLUX PNG data for {}: {error}",
            path.display()
        ))
    })
}
