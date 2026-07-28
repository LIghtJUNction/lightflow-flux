use std::{
    env,
    fs::{self, read_dir},
    path::{Path, PathBuf},
};

use cmake::Config;
mod build_support;

// Inspired by https://github.com/tazz4843/whisper-rs/blob/master/sys/build.rs

fn main() {
    // Link C++ standard library
    let target = env::var("TARGET").unwrap();
    if let Some(cpp_stdlib) = get_cpp_link_stdlib(&target) {
        println!("cargo:rustc-link-lib=dylib={cpp_stdlib}");
    }

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=src/bindings.rs");
    println!("cargo:rerun-if-env-changed=DOCS_RS");
    for path in build_support::source_files(Path::new("stable-diffusion.cpp"))
        .expect("enumerate vendored native sources")
    {
        println!("cargo:rerun-if-changed={}", path.display());
    }

    // Copy stable-diffusion code into the build script directory
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let diffusion_root = out.join("stable-diffusion.cpp/");

    build_support::replace_tree(Path::new("stable-diffusion.cpp"), &diffusion_root).unwrap_or_else(
        |error| {
            panic!(
                "failed to refresh stable-diffusion sources in {}: {error}",
                diffusion_root.display()
            )
        },
    );

    let bindings_path = out.join("bindings.rs");
    if env::var_os("DOCS_RS").is_some() {
        fs::copy("src/bindings.rs", &bindings_path).expect("copy bundled docs.rs bindings");
        return;
    }
    let generated = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        bindgen::Builder::default()
            .header("wrapper.h")
            .clang_arg("-I./stable-diffusion.cpp")
            .clang_arg("-I./stable-diffusion.cpp/ggml/include")
            .layout_tests(false)
            .rustified_non_exhaustive_enum(".*")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .generate()
    }));
    match generated {
        Ok(Ok(bindings)) => bindings
            .write_to_file(&bindings_path)
            .expect("write generated bindings"),
        Ok(Err(error)) => {
            println!("cargo:warning=bindgen unavailable ({error}); using bundled bindings");
            fs::copy("src/bindings.rs", &bindings_path).expect("copy bundled bindings");
        }
        Err(_) => {
            println!("cargo:warning=libclang unavailable; using bundled bindings");
            fs::copy("src/bindings.rs", &bindings_path).expect("copy bundled bindings");
        }
    }

    // Configure cmake for building
    let mut config = Config::new(&diffusion_root);

    if target.contains("msvc") {
        config.generator("Ninja");
        config.define("CMAKE_BUILD_TYPE", "Release");
        config.define("CMAKE_C_COMPILER", "cl.exe");
        config.define("CMAKE_CXX_COMPILER", "cl.exe");
        config.define("CMAKE_CXX_FLAGS", "'/bigobj'");
    }
    config
        .profile("Release")
        .define("SD_BUILD_SHARED_LIBS", "OFF")
        .define("SD_BUILD_EXAMPLES", "OFF")
        .define("SD_BUILD_SERVER", "OFF")
        .define("GGML_OPENMP", "OFF")
        .define("SD_WEBP", "OFF")
        .define("SD_WEBM", "OFF")
        .very_verbose(true)
        .pic(true);

    let use_vulkan = cfg!(feature = "vulkan");
    let mut use_metal = cfg!(feature = "metal") && !use_vulkan;

    if target.contains("apple") && !use_vulkan {
        use_metal = true;
    }

    //Enable cmake feature flags
    #[cfg(feature = "cuda")]
    {
        println!("cargo:rerun-if-env-changed=CUDA_PATH");
        println!("cargo:rustc-link-lib=cublas");
        println!("cargo:rustc-link-lib=cudart");
        println!("cargo:rustc-link-lib=cublasLt");
        println!("cargo:rustc-link-lib=cuda");

        if target.contains("msvc") {
            let cuda_path = PathBuf::from(env::var("CUDA_PATH").unwrap()).join("lib/x64");
            println!("cargo:rustc-link-search={}", cuda_path.display());
        } else {
            println!("cargo:rustc-link-lib=culibos");
            println!("cargo:rustc-link-search=/usr/local/cuda/lib64");
            println!("cargo:rustc-link-search=/usr/local/cuda/lib64/stubs");
            println!("cargo:rustc-link-search=/opt/cuda/lib64");
            println!("cargo:rustc-link-search=/opt/cuda/lib64/stubs");
        }

        config.define("SD_CUDA", "ON");
        if let Ok(target) = env::var("CUDA_COMPUTE_CAP") {
            config.define("CUDA_COMPUTE_CAP", target);
        }
    }

    #[cfg(feature = "hipblas")]
    {
        println!("cargo:rerun-if-env-changed=HIP_PATH");
        println!("cargo:rustc-link-lib=hipblas");
        println!("cargo:rustc-link-lib=rocblas");
        println!("cargo:rustc-link-lib=amdhip64");

        config.generator("Ninja");
        config.define("CMAKE_C_COMPILER", "clang");
        config.define("CMAKE_CXX_COMPILER", "clang++");
        config.define("CMAKE_BUILD_WITH_INSTALL_RPATH", "ON");
        config.define("CMAKE_POSITION_INDEPENDENT_CODE", "ON");
        let hip_lib_path = if target.contains("msvc") {
            let hip_path = env::var("HIP_PATH").expect("Missing HIP_PATH env variable");
            PathBuf::from(hip_path).join("lib")
        } else {
            let hip_path = match env::var("HIP_PATH") {
                Ok(path) => PathBuf::from(path),
                Err(_) => PathBuf::from("/opt/rocm"),
            };
            hip_path.join("lib")
        };
        println!("cargo:rustc-link-search={}", hip_lib_path.display());

        config.define("SD_HIPBLAS", "ON");
        if let Ok(target) = env::var("GFX_NAME") {
            config.define("AMDGPU_TARGETS", &target);
            config.define("GPU_TARGETS", target);
        }
    }

    if target.contains("apple") {
        println!("cargo:rustc-link-lib=framework=Accelerate");
        println!("cargo:rustc-link-lib=framework=Foundation");
    }

    if use_metal {
        config.define("SD_METAL", "ON");
        println!("cargo:rustc-link-lib=framework=Metal");
        println!("cargo:rustc-link-lib=framework=MetalKit");
    }

    if use_vulkan {
        let vulkan_path = env::var("VULKAN_SDK").map(PathBuf::from);
        if target.contains("msvc") {
            println!("cargo:rerun-if-env-changed=VULKAN_SDK");
            println!("cargo:rustc-link-lib=vulkan-1");

            let vulkan_lib_path = vulkan_path
                .expect("Please install Vulkan SDK and ensure that VULKAN_SDK env variable is set")
                .join("Lib");
            println!("cargo:rustc-link-search={}", vulkan_lib_path.display());
        } else {
            if let Ok(vulkan_path) = vulkan_path {
                let vulkan_lib_path = vulkan_path.join("lib");
                println!("cargo:rustc-link-search={}", vulkan_lib_path.display());
            }
            if target.contains("darwin") {
                println!("cargo:rustc-link-search=/usr/local/lib");
            }
            println!("cargo:rustc-link-lib=vulkan");
        }
        config.define("SD_VULKAN", "ON");
    }

    #[cfg(feature = "sycl")]
    {
        env::var("ONEAPI_ROOT").expect("Please load the oneAPi environment before building. See https://github.com/ggerganov/llama.cpp/blob/master/docs/backend/SYCL.md");
        let sycl_lib_path = PathBuf::from(env::var("ONEAPI_ROOT").unwrap()).join("mkl/latest/lib");
        println!("cargo:rustc-link-search={}", sycl_lib_path.display());

        println!("cargo:rustc-link-lib=static=mkl_sycl");
        println!("cargo:rustc-link-lib=static=mkl_core");
        println!("cargo:rustc-link-lib=static=mkl_scalapack_ilp64");
        println!("cargo:rustc-link-lib=static=mkl_intel_ilp64");
        println!("cargo:rustc-link-lib=static=mkl_blacs_intelmpi_ilp64");
        println!("cargo:rustc-link-lib=static=mkl_tbb_thread");

        println!("cargo:rustc-link-lib=tbb");
        println!("cargo:rustc-link-lib=OpenCL");
        println!("cargo:rustc-link-lib=svml");
        println!("cargo:rustc-link-lib=imf");
        println!("cargo:rustc-link-lib=intlc");
        println!("cargo:rustc-link-lib=ur_loader");
        println!("cargo:rustc-link-lib=m");
        println!("cargo-rustc-link-lib=dl");
        println!("cargo:rustc-link-lib=sycl");
        println!("cargo:rustc-link-lib=dnnl");

        if target.contains("msvc") {
            config.generator("Ninja");
            config.define("CMAKE_C_COMPILER", "cl");
            config.define("CMAKE_CXX_COMPILER", "icx");
        } else {
            config.define("CMAKE_C_COMPILER", "icx");
            config.define("CMAKE_CXX_COMPILER", "icpx");
        }
        config.define("SD_SYCL", "ON");
    }

    // Build stable-diffusion
    let destination = config.build();

    add_link_search_path(&out.join("lib")).unwrap();
    add_link_search_path(&out.join("build")).unwrap();
    add_link_search_path(&out).unwrap();

    println!("cargo:rustc-link-search=native={}", destination.display());
    println!("cargo:rustc-link-lib=static=stable-diffusion");
    println!("cargo:rustc-link-lib=static=ggml-base");
    println!("cargo:rustc-link-lib=static=ggml-cpu");
    println!("cargo:rustc-link-lib=static=ggml");

    #[cfg(feature = "cuda")]
    println!("cargo:rustc-link-lib=static=ggml-cuda");

    #[cfg(feature = "hipblas")]
    println!("cargo:rustc-link-lib=static=ggml-hip");

    if use_metal {
        println!("cargo:rustc-link-lib=static=ggml-blas");
        println!("cargo:rustc-link-lib=static=ggml-metal");
    }

    if use_vulkan {
        println!("cargo:rustc-link-lib=static=ggml-vulkan");
    }

    #[cfg(feature = "sycl")]
    println!("cargo:rustc-link-lib=static=ggml-sycl");
}

fn add_link_search_path(dir: &Path) -> std::io::Result<()> {
    if dir.is_dir() {
        println!("cargo:rustc-link-search={}", dir.display());
        for entry in read_dir(dir)? {
            add_link_search_path(&entry?.path())?;
        }
    }
    Ok(())
}

// From https://github.com/alexcrichton/cc-rs/blob/fba7feded71ee4f63cfe885673ead6d7b4f2f454/src/lib.rs#L2462
fn get_cpp_link_stdlib(target: &str) -> Option<&'static str> {
    if target.contains("msvc") {
        None
    } else if target.contains("apple") || target.contains("freebsd") || target.contains("openbsd") {
        Some("c++")
    } else if target.contains("android") {
        Some("c++_shared")
    } else {
        Some("stdc++")
    }
}
