extern crate bindgen;
extern crate cc;
extern crate num_cpus;
extern crate pkg_config;

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str;

use bindgen::callbacks::{
    EnumVariantCustomBehavior, EnumVariantValue, IntKind, MacroParsingBehavior, ParseCallbacks,
};

#[derive(Debug)]
struct Library {
    name: &'static str,
    is_feature: bool,
}

impl Library {
    fn feature_name(&self) -> Option<String> {
        if self.is_feature {
            Some("CARGO_FEATURE_".to_string() + &self.name.to_uppercase())
        } else {
            None
        }
    }
}

static LIBRARIES: &[Library] = &[
    Library {
        name: "avcodec",
        is_feature: true,
    },
    Library {
        name: "avdevice",
        is_feature: true,
    },
    Library {
        name: "avfilter",
        is_feature: true,
    },
    Library {
        name: "avformat",
        is_feature: true,
    },
    Library {
        name: "avresample",
        is_feature: true,
    },
    Library {
        name: "avutil",
        is_feature: false,
    },
    Library {
        name: "postproc",
        is_feature: true,
    },
    Library {
        name: "swresample",
        is_feature: true,
    },
    Library {
        name: "swscale",
        is_feature: true,
    },
];

#[derive(Debug)]
struct Callbacks;

impl ParseCallbacks for Callbacks {
    fn int_macro(&self, _name: &str, value: i64) -> Option<IntKind> {
        let ch_layout_prefix = "AV_CH_";
        let codec_cap_prefix = "AV_CODEC_CAP_";
        let codec_flag_prefix = "AV_CODEC_FLAG_";
        let error_max_size = "AV_ERROR_MAX_STRING_SIZE";

        if value >= i64::min_value() as i64
            && value <= i64::max_value() as i64
            && _name.starts_with(ch_layout_prefix)
        {
            Some(IntKind::ULongLong)
        } else if value >= i32::min_value() as i64
            && value <= i32::max_value() as i64
            && (_name.starts_with(codec_cap_prefix) || _name.starts_with(codec_flag_prefix))
        {
            Some(IntKind::UInt)
        } else if _name == error_max_size {
            Some(IntKind::Custom {
                name: "usize",
                is_signed: false,
            })
        } else if value >= i32::min_value() as i64 && value <= i32::max_value() as i64 {
            Some(IntKind::Int)
        } else {
            None
        }
    }

    fn enum_variant_behavior(
        &self,
        _enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: EnumVariantValue,
    ) -> Option<EnumVariantCustomBehavior> {
        let dummy_codec_id_prefix = "AV_CODEC_ID_FIRST_";
        if original_variant_name.starts_with(dummy_codec_id_prefix) {
            Some(EnumVariantCustomBehavior::Constify)
        } else {
            None
        }
    }

    // https://github.com/rust-lang/rust-bindgen/issues/687#issuecomment-388277405
    fn will_parse_macro(&self, name: &str) -> MacroParsingBehavior {
        use MacroParsingBehavior::*;

        match name {
            "FP_INFINITE" => Ignore,
            "FP_NAN" => Ignore,
            "FP_NORMAL" => Ignore,
            "FP_SUBNORMAL" => Ignore,
            "FP_ZERO" => Ignore,
            _ => Default,
        }
    }
}

fn version() -> String {
    let major: u8 = env::var("CARGO_PKG_VERSION_MAJOR")
        .unwrap()
        .parse()
        .unwrap();
    let minor: u8 = env::var("CARGO_PKG_VERSION_MINOR")
        .unwrap()
        .parse()
        .unwrap();

    format!("{}.{}", major, minor)
}

fn output() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").unwrap())
}

fn source() -> PathBuf {
    output().join(format!("ffmpeg-{}", version()))
}

fn search() -> PathBuf {
    let mut absolute = env::current_dir().unwrap();
    absolute.push(&output());
    absolute.push("dist");

    absolute
}

fn fetch() -> io::Result<()> {
    let output_base_path = output();
    let clone_dest_dir = format!("ffmpeg-{}", version());
    let _ = std::fs::remove_dir_all(output_base_path.join(&clone_dest_dir));
    let status = Command::new("git")
        .current_dir(&output_base_path)
        .arg("clone")
        .arg("--depth=1")
        .arg("-b")
        .arg(format!("release/{}", version()))
        .arg("https://github.com/FFmpeg/FFmpeg")
        .arg(&clone_dest_dir)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "fetch failed"))
    }
}

fn switch(configure: &mut Command, feature: &str, name: &str) {
    let arg = if env::var("CARGO_FEATURE_".to_string() + feature).is_ok() {
        "--enable-"
    } else {
        "--disable-"
    };
    configure.arg(arg.to_string() + name);
}

fn build() -> io::Result<()> {
    let source_dir = source();

    // Command's path is not relative to command's current_dir
    let configure_path = source_dir.join("configure");
    assert!(configure_path.exists());
    let mut configure = Command::new(&configure_path);
    configure.current_dir(&source_dir);

    configure.arg(format!("--prefix={}", search().to_string_lossy()));

    if env::var("TARGET").unwrap() != env::var("HOST").unwrap() {
        // Rust targets are subtly different than naming scheme for compiler prefixes.
        // The cc crate has the messy logic of guessing a working prefix,
        // and this is a messy way of reusing that logic.
        let cc = cc::Build::new();
        let compiler = cc.get_compiler();
        let compiler = compiler.path().file_stem().unwrap().to_str().unwrap();
        let suffix_pos = compiler.rfind('-').unwrap(); // cut off "-gcc"
        let prefix = compiler[0..suffix_pos].trim_end_matches("-wr"); // "wr-c++" compiler

        configure.arg(format!("--cross-prefix={}-", prefix));
        configure.arg(format!(
            "--arch={}",
            env::var("CARGO_CFG_TARGET_ARCH").unwrap()
        ));
        configure.arg(format!(
            "--target_os={}",
            env::var("CARGO_CFG_TARGET_OS").unwrap()
        ));
    }

    // control debug build
    if env::var("DEBUG").is_ok() {
        configure.arg("--enable-debug");
        configure.arg("--disable-stripping");
        configure.arg("--disable-optimizations");
    } else {
        configure.arg("--disable-debug");
        configure.arg("--enable-stripping");
    }

    // make it static
    configure.arg("--enable-static");
    configure.arg("--disable-shared");

    configure.arg("--enable-pic");

    // stop autodetected libraries enabling themselves, causing linking errors
    configure.arg("--disable-autodetect");

    // do not build programs since we don't need them
    configure.arg("--disable-programs");

    macro_rules! enable {
        ($conf:expr, $feat:expr, $name:expr) => {
            if env::var(concat!("CARGO_FEATURE_", $feat)).is_ok() {
                $conf.arg(concat!("--enable-", $name));
            }
        };
    }

    // macro_rules! disable {
    //     ($conf:expr, $feat:expr, $name:expr) => (
    //         if env::var(concat!("CARGO_FEATURE_", $feat)).is_err() {
    //             $conf.arg(concat!("--disable-", $name));
    //         }
    //     )
    // }

    // the binary using ffmpeg-sys must comply with GPL
    switch(&mut configure, "BUILD_LICENSE_GPL", "gpl");

    // the binary using ffmpeg-sys must comply with (L)GPLv3
    switch(&mut configure, "BUILD_LICENSE_VERSION3", "version3");

    // the binary using ffmpeg-sys cannot be redistributed
    switch(&mut configure, "BUILD_LICENSE_NONFREE", "nonfree");

    // configure building libraries based on features
    for lib in LIBRARIES.iter().filter(|lib| lib.is_feature) {
        switch(&mut configure, &lib.name.to_uppercase(), lib.name);
    }

    // configure external SSL libraries
    enable!(configure, "BUILD_LIB_GNUTLS", "gnutls");
    enable!(configure, "BUILD_LIB_OPENSSL", "openssl");

    // configure external filters
    enable!(configure, "BUILD_LIB_FONTCONFIG", "fontconfig");
    enable!(configure, "BUILD_LIB_FREI0R", "frei0r");
    enable!(configure, "BUILD_LIB_LADSPA", "ladspa");
    enable!(configure, "BUILD_LIB_ASS", "libass");
    enable!(configure, "BUILD_LIB_FREETYPE", "libfreetype");
    enable!(configure, "BUILD_LIB_FRIBIDI", "libfribidi");
    enable!(configure, "BUILD_LIB_OPENCV", "libopencv");

    // configure external encoders/decoders
    enable!(configure, "BUILD_LIB_AACPLUS", "libaacplus");
    enable!(configure, "BUILD_LIB_CELT", "libcelt");
    enable!(configure, "BUILD_LIB_DCADEC", "libdcadec");
    enable!(configure, "BUILD_LIB_FAAC", "libfaac");
    enable!(configure, "BUILD_LIB_FDK_AAC", "libfdk-aac");
    enable!(configure, "BUILD_LIB_GSM", "libgsm");
    enable!(configure, "BUILD_LIB_ILBC", "libilbc");
    enable!(configure, "BUILD_LIB_VAZAAR", "libvazaar");
    enable!(configure, "BUILD_LIB_MP3LAME", "libmp3lame");
    enable!(configure, "BUILD_LIB_OPENCORE_AMRNB", "libopencore-amrnb");
    enable!(configure, "BUILD_LIB_OPENCORE_AMRWB", "libopencore-amrwb");
    enable!(configure, "BUILD_LIB_OPENH264", "libopenh264");
    enable!(configure, "BUILD_LIB_OPENH265", "libopenh265");
    enable!(configure, "BUILD_LIB_OPENJPEG", "libopenjpeg");
    enable!(configure, "BUILD_LIB_OPUS", "libopus");
    enable!(configure, "BUILD_LIB_SCHROEDINGER", "libschroedinger");
    enable!(configure, "BUILD_LIB_SHINE", "libshine");
    enable!(configure, "BUILD_LIB_SNAPPY", "libsnappy");
    enable!(configure, "BUILD_LIB_SPEEX", "libspeex");
    enable!(
        configure,
        "BUILD_LIB_STAGEFRIGHT_H264",
        "libstagefright-h264"
    );
    enable!(configure, "BUILD_LIB_THEORA", "libtheora");
    enable!(configure, "BUILD_LIB_TWOLAME", "libtwolame");
    enable!(configure, "BUILD_LIB_UTVIDEO", "libutvideo");
    enable!(configure, "BUILD_LIB_VO_AACENC", "libvo-aacenc");
    enable!(configure, "BUILD_LIB_VO_AMRWBENC", "libvo-amrwbenc");
    enable!(configure, "BUILD_LIB_VORBIS", "libvorbis");
    enable!(configure, "BUILD_LIB_VPX", "libvpx");
    enable!(configure, "BUILD_LIB_WAVPACK", "libwavpack");
    enable!(configure, "BUILD_LIB_WEBP", "libwebp");
    enable!(configure, "BUILD_LIB_X264", "libx264");
    enable!(configure, "BUILD_LIB_X265", "libx265");
    enable!(configure, "BUILD_LIB_AVS", "libavs");
    enable!(configure, "BUILD_LIB_XVID", "libxvid");
    enable!(configure, "BUILD_LIB_DRM", "libdrm");
    // other external libraries
    enable!(configure, "BUILD_NVENC", "nvenc");

    // configure external protocols
    enable!(configure, "BUILD_LIB_SMBCLIENT", "libsmbclient");
    enable!(configure, "BUILD_LIB_SSH", "libssh");

    // configure misc build options
    enable!(configure, "BUILD_PIC", "pic");

    // run ./configure
    let output = configure
        .output()
        .unwrap_or_else(|_| panic!("{:?} failed", configure));
    if !output.status.success() {
        println!("configure: {}", String::from_utf8_lossy(&output.stdout));

        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "configure failed {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        ));
    }

    // run make
    if !Command::new("make")
        .arg("-j")
        .arg(num_cpus::get().to_string())
        .current_dir(&source())
        .status()?
        .success()
    {
        return Err(io::Error::new(io::ErrorKind::Other, "make failed"));
    }

    // run make install
    if !Command::new("make")
        .current_dir(&source())
        .arg("install")
        .status()?
        .success()
    {
        return Err(io::Error::new(io::ErrorKind::Other, "make install failed"));
    }

    Ok(())
}

#[cfg(not(target_env = "msvc"))]
fn try_vcpkg(_statik: bool) -> Option<Vec<PathBuf>> {
    None
}

#[cfg(target_env = "msvc")]
fn try_vcpkg(statik: bool) -> Option<Vec<PathBuf>> {
    if !statik {
        env::set_var("VCPKGRS_DYNAMIC", "1");
    }

    vcpkg::find_package("ffmpeg")
        .map_err(|e| {
            println!("Could not find ffmpeg with vcpkg: {}", e);
        })
        .map(|library| library.include_paths)
        .ok()
}

fn check_features(
    include_paths: Vec<PathBuf>,
    infos: &[(&'static str, Option<&'static str>, &'static str)],
) {
    let mut includes_code = String::new();
    let mut main_code = String::new();

    for &(header, feature, var) in infos {
        if let Some(feature) = feature {
            if env::var(format!("CARGO_FEATURE_{}", feature.to_uppercase())).is_err() {
                continue;
            }
        }

        let include = format!("#include <{}>", header);
        if includes_code.find(&include).is_none() {
            includes_code.push_str(&include);
            includes_code.push_str(&"\n");
        }
        includes_code.push_str(&format!(
            r#"
            #ifndef {var}_is_defined
            #ifndef {var}
            #define {var} 0
            #define {var}_is_defined 0
            #else
            #define {var}_is_defined 1
            #endif
            #endif
        "#,
            var = var
        ));

        main_code.push_str(&format!(
            r#"printf("[{var}]%d%d\n", {var}, {var}_is_defined);
            "#,
            var = var
        ));
    }

    let version_check_info = [("avcodec", 56, 60, 0, 108)];
    for &(lib, begin_version_major, end_version_major, begin_version_minor, end_version_minor) in
        version_check_info.iter()
    {
        for version_major in begin_version_major..end_version_major {
            for version_minor in begin_version_minor..end_version_minor {
                main_code.push_str(&format!(
                    r#"printf("[{lib}_version_greater_than_{version_major}_{version_minor}]%d\n", LIB{lib_uppercase}_VERSION_MAJOR > {version_major} || (LIB{lib_uppercase}_VERSION_MAJOR == {version_major} && LIB{lib_uppercase}_VERSION_MINOR > {version_minor}));
                    "#, lib = lib,
                    lib_uppercase = lib.to_uppercase(),
                    version_major = version_major,
                    version_minor = version_minor
                ));
            }
        }
    }

    let out_dir = output();

    write!(
        File::create(out_dir.join("check.c")).expect("Failed to create file"),
        r#"
            #include <stdio.h>
            {includes_code}

            int main()
            {{
                {main_code}
                return 0;
            }}
           "#,
        includes_code = includes_code,
        main_code = main_code
    )
    .expect("Write failed");

    let executable = out_dir.join(if cfg!(windows) { "check.exe" } else { "check" });
    let mut compiler = cc::Build::new()
        .target(&env::var("HOST").unwrap()) // don't cross-compile this
        .get_compiler()
        .to_command();

    for dir in include_paths {
        compiler.arg("-I");
        compiler.arg(dir.to_string_lossy().into_owned());
    }
    if !compiler
        .current_dir(&out_dir)
        .arg("-o")
        .arg(&executable)
        .arg("check.c")
        .status()
        .expect("Command failed")
        .success()
    {
        panic!("Compile failed");
    }

    let check_output = Command::new(out_dir.join(&executable))
        .current_dir(&out_dir)
        .output()
        .expect("Check failed");
    if !check_output.status.success() {
        panic!(
            "{} failed: {}\n{}",
            executable.display(),
            String::from_utf8_lossy(&check_output.stdout),
            String::from_utf8_lossy(&check_output.stderr)
        );
    }

    let stdout = str::from_utf8(&check_output.stdout).unwrap();

    println!("stdout of {}={}", executable.display(), stdout);

    for &(_, feature, var) in infos {
        if let Some(feature) = feature {
            if env::var(format!("CARGO_FEATURE_{}", feature.to_uppercase())).is_err() {
                continue;
            }
        }

        let var_str = format!("[{var}]", var = var);
        let pos = var_str.len()
            + stdout
                .find(&var_str)
                .unwrap_or_else(|| panic!("Variable '{}' not found in stdout output", var_str));
        if &stdout[pos..pos + 1] == "1" {
            println!(r#"cargo:rustc-cfg=feature="{}""#, var.to_lowercase());
            println!(r#"cargo:{}=true"#, var.to_lowercase());
        }

        // Also find out if defined or not (useful for cases where only the definition of a macro
        // can be used as distinction)
        if &stdout[pos + 1..pos + 2] == "1" {
            println!(
                r#"cargo:rustc-cfg=feature="{}_is_defined""#,
                var.to_lowercase()
            );
            println!(r#"cargo:{}_is_defined=true"#, var.to_lowercase());
        }
    }

    for &(lib, begin_version_major, end_version_major, begin_version_minor, end_version_minor) in
        version_check_info.iter()
    {
        for version_major in begin_version_major..end_version_major {
            for version_minor in begin_version_minor..end_version_minor {
                let search_str = format!(
                    "[{lib}_version_greater_than_{version_major}_{version_minor}]",
                    version_major = version_major,
                    version_minor = version_minor,
                    lib = lib
                );
                let pos = stdout
                    .find(&search_str)
                    .expect("Variable not found in output")
                    + search_str.len();

                if &stdout[pos..pos + 1] == "1" {
                    println!(
                        r#"cargo:rustc-cfg=feature="{}""#,
                        &search_str[1..(search_str.len() - 1)]
                    );
                }
            }
        }
    }

    let ffmpeg_lavc_versions = [
        ("ffmpeg_3_0", 57, 24),
        ("ffmpeg_3_1", 57, 48),
        ("ffmpeg_3_2", 57, 64),
        ("ffmpeg_3_3", 57, 89),
        ("ffmpeg_3_1", 57, 107),
        ("ffmpeg_4_0", 58, 18),
        ("ffmpeg_4_1", 58, 35),
        ("ffmpeg_4_2", 58, 54),
        ("ffmpeg_4_3", 58, 91),
    ];
    for &(ffmpeg_version_flag, lavc_version_major, lavc_version_minor) in
        ffmpeg_lavc_versions.iter()
    {
        let search_str = format!(
            "[avcodec_version_greater_than_{lavc_version_major}_{lavc_version_minor}]",
            lavc_version_major = lavc_version_major,
            lavc_version_minor = lavc_version_minor - 1
        );
        let pos = stdout
            .find(&search_str)
            .expect("Variable not found in output")
            + search_str.len();
        if &stdout[pos..pos + 1] == "1" {
            println!(r#"cargo:rustc-cfg=feature="{}""#, ffmpeg_version_flag);
            println!(r#"cargo:{}=true"#, ffmpeg_version_flag);
        }
    }
}

fn search_include(include_paths: &[PathBuf], header: &str) -> String {
    for dir in include_paths {
        let include = dir.join(header);
        if fs::metadata(&include).is_ok() {
            return include.as_path().to_str().unwrap().to_string();
        }
    }
    format!("/usr/include/{}", header)
}

fn link_to_libraries(statik: bool) {
    let ffmpeg_ty = if statik { "static" } else { "dylib" };
    for lib in LIBRARIES {
        let feat_is_enabled = lib.feature_name().and_then(|f| env::var(&f).ok()).is_some();
        if !lib.is_feature || feat_is_enabled {
            println!("cargo:rustc-link-lib={}={}", ffmpeg_ty, lib.name);
        }
    }
    if env::var("CARGO_FEATURE_BUILD_ZLIB").is_ok() && cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=z");
    }
}

fn main() {
    // The long chain of `header` method calls for `bindgen::Builder` seems to be overflowing the default stack size on Windows.
    // The main thread appears to have a hardcoded stack size which is unaffected by `RUST_MIN_STACK`. As a workaround, spawn a thread here with a stack size that works expermentally, and allow overriding it with `FFMPEG_SYS_BUILD_STACK_SIZE` just in case.
    let stack_size = std::env::var("FFMPEG_SYS_BUILD_STACK_SIZE")
        .map(|s| s.parse())
        .unwrap_or(Ok(3 * 1024 * 1024));
    eprintln!("Using stack size: {:?}", stack_size);

    std::thread::Builder::new()
        .name("ffmpg-sys-build".into())
        .stack_size(stack_size.unwrap())
        .spawn(thread_main)
        .unwrap()
        .join()
        .unwrap();
}

fn thread_main() {
    let statik = env::var("CARGO_FEATURE_STATIC").is_ok();

    let include_paths: Vec<PathBuf> = if env::var("CARGO_FEATURE_BUILD").is_ok() {
        println!(
            "cargo:rustc-link-search=native={}",
            search().join("lib").to_string_lossy()
        );
        link_to_libraries(statik);
        if fs::metadata(&search().join("lib").join("libavutil.a")).is_err() {
            fs::create_dir_all(&output()).expect("failed to create build directory");
            fetch().unwrap();
            build().unwrap();
        }

        // Check additional required libraries.
        {
            let config_mak = source().join("ffbuild/config.mak");
            let file = File::open(config_mak).unwrap();
            let reader = BufReader::new(file);
            let extra_libs = reader
                .lines()
                .find(|ref line| line.as_ref().unwrap().starts_with("EXTRALIBS"))
                .map(|line| line.unwrap())
                .unwrap();

            let linker_args = extra_libs.split('=').last().unwrap().split(' ');
            let include_libs = linker_args
                .filter(|v| v.starts_with("-l"))
                .map(|flag| &flag[2..]);

            for lib in include_libs {
                println!("cargo:rustc-link-lib={}", lib);
            }
        }

        vec![search().join("include")]
    }
    // Use prebuilt library
    else if let Ok(ffmpeg_dir) = env::var("FFMPEG_DIR") {
        let ffmpeg_dir = PathBuf::from(ffmpeg_dir);
        println!(
            "cargo:rustc-link-search=native={}",
            ffmpeg_dir.join("lib").to_string_lossy()
        );
        link_to_libraries(statik);
        vec![ffmpeg_dir.join("include")]
    } else if let Some(paths) = try_vcpkg(statik) {
        // vcpkg doesn't detect the "system" dependencies
        if statik {
            if cfg!(feature = "avcodec") || cfg!(feature = "avdevice") {
                println!("cargo:rustc-link-lib=ole32");
            }

            if cfg!(feature = "avformat") {
                println!("cargo:rustc-link-lib=secur32");
                println!("cargo:rustc-link-lib=ws2_32");
            }

            // avutil depdendencies
            println!("cargo:rustc-link-lib=bcrypt");
            println!("cargo:rustc-link-lib=user32");
        }

        paths
    }
    // Fallback to pkg-config
    else {
        let mut libavutil = pkg_config::Config::new()
            .cargo_metadata(false)
            .statik(statik)
            .probe("libavutil")
            .unwrap();
        print_pkg_config_libs(statik, &libavutil);

        let libs = vec![
            ("libavformat", "AVFORMAT"),
            ("libavfilter", "AVFILTER"),
            ("libavdevice", "AVDEVICE"),
            ("libavresample", "AVRESAMPLE"),
            ("libswscale", "SWSCALE"),
            ("libswresample", "SWRESAMPLE"),
        ];

        for (lib_name, env_variable_name) in libs.iter() {
            if env::var(format!("CARGO_FEATURE_{}", env_variable_name)).is_ok() {
                print_pkg_config_libs(
                    statik,
                    &pkg_config::Config::new()
                        .cargo_metadata(false)
                        .statik(statik)
                        .probe(lib_name)
                        .unwrap(),
                );
            }
        }

        let libavcodec = pkg_config::Config::new()
            .cargo_metadata(false)
            .statik(statik)
            .probe("libavcodec")
            .unwrap();
        print_pkg_config_libs(statik, &libavcodec);

        let mut paths = libavcodec.include_paths;
        paths.append(&mut libavutil.include_paths);
        paths
    };

    if statik && cfg!(target_os = "macos") {
        let frameworks = vec![
            "AppKit",
            "AudioToolbox",
            "AVFoundation",
            "CoreFoundation",
            "CoreGraphics",
            "CoreMedia",
            "CoreServices",
            "CoreVideo",
            "Foundation",
            "OpenCL",
            "OpenGL",
            "QTKit",
            "QuartzCore",
            "Security",
            "VideoDecodeAcceleration",
            "VideoToolbox",
        ];
        for f in frameworks {
            println!("cargo:rustc-link-lib=framework={}", f);
        }
    }

    check_features(
        include_paths.clone(),
        &[
            ("libavutil/avutil.h", None, "FF_API_OLD_AVOPTIONS"),
            ("libavutil/avutil.h", None, "FF_API_PIX_FMT"),
            ("libavutil/avutil.h", None, "FF_API_CONTEXT_SIZE"),
            ("libavutil/avutil.h", None, "FF_API_PIX_FMT_DESC"),
            ("libavutil/avutil.h", None, "FF_API_AV_REVERSE"),
            ("libavutil/avutil.h", None, "FF_API_AUDIOCONVERT"),
            ("libavutil/avutil.h", None, "FF_API_CPU_FLAG_MMX2"),
            ("libavutil/avutil.h", None, "FF_API_LLS_PRIVATE"),
            ("libavutil/avutil.h", None, "FF_API_AVFRAME_LAVC"),
            ("libavutil/avutil.h", None, "FF_API_VDPAU"),
            (
                "libavutil/avutil.h",
                None,
                "FF_API_GET_CHANNEL_LAYOUT_COMPAT",
            ),
            ("libavutil/avutil.h", None, "FF_API_XVMC"),
            ("libavutil/avutil.h", None, "FF_API_OPT_TYPE_METADATA"),
            ("libavutil/avutil.h", None, "FF_API_DLOG"),
            ("libavutil/avutil.h", None, "FF_API_HMAC"),
            ("libavutil/avutil.h", None, "FF_API_VAAPI"),
            ("libavutil/avutil.h", None, "FF_API_PKT_PTS"),
            ("libavutil/avutil.h", None, "FF_API_ERROR_FRAME"),
            ("libavutil/avutil.h", None, "FF_API_FRAME_QP"),
            (
                "libavcodec/avcodec.h",
                Some("avcodec"),
                "FF_API_VIMA_DECODER",
            ),
            (
                "libavcodec/avcodec.h",
                Some("avcodec"),
                "FF_API_REQUEST_CHANNELS",
            ),
            (
                "libavcodec/avcodec.h",
                Some("avcodec"),
                "FF_API_OLD_DECODE_AUDIO",
            ),
            (
                "libavcodec/avcodec.h",
                Some("avcodec"),
                "FF_API_OLD_ENCODE_AUDIO",
            ),
            (
                "libavcodec/avcodec.h",
                Some("avcodec"),
                "FF_API_OLD_ENCODE_VIDEO",
            ),
            ("libavcodec/avcodec.h", Some("avcodec"), "FF_API_CODEC_ID"),
            (
                "libavcodec/avcodec.h",
                Some("avcodec"),
                "FF_API_AUDIO_CONVERT",
            ),
            (
                "libavcodec/avcodec.h",
                Some("avcodec"),
                "FF_API_AVCODEC_RESAMPLE",
            ),
            (
                "libavcodec/avcodec.h",
                Some("avcodec"),
                "FF_API_DEINTERLACE",
            ),
            (
                "libavcodec/avcodec.h",
                Some("avcodec"),
                "FF_API_DESTRUCT_PACKET",
            ),
            ("libavcodec/avcodec.h", Some("avcodec"), "FF_API_GET_BUFFER"),
            (
                "libavcodec/avcodec.h",
                Some("avcodec"),
                "FF_API_MISSING_SAMPLE",
            ),
            ("libavcodec/avcodec.h", Some("avcodec"), "FF_API_LOWRES"),
            ("libavcodec/avcodec.h", Some("avcodec"), "FF_API_CAP_VDPAU"),
            ("libavcodec/avcodec.h", Some("avcodec"), "FF_API_BUFS_VDPAU"),
            ("libavcodec/avcodec.h", Some("avcodec"), "FF_API_VOXWARE"),
            (
                "libavcodec/avcodec.h",
                Some("avcodec"),
                "FF_API_SET_DIMENSIONS",
            ),
            ("libavcodec/avcodec.h", Some("avcodec"), "FF_API_DEBUG_MV"),
            ("libavcodec/avcodec.h", Some("avcodec"), "FF_API_AC_VLC"),
            (
                "libavcodec/avcodec.h",
                Some("avcodec"),
                "FF_API_OLD_MSMPEG4",
            ),
            (
                "libavcodec/avcodec.h",
                Some("avcodec"),
                "FF_API_ASPECT_EXTENDED",
            ),
            (
                "libavcodec/avcodec.h",
                Some("avcodec"),
                "FF_API_THREAD_OPAQUE",
            ),
            ("libavcodec/avcodec.h", Some("avcodec"), "FF_API_CODEC_PKT"),
            ("libavcodec/avcodec.h", Some("avcodec"), "FF_API_ARCH_ALPHA"),
            ("libavcodec/avcodec.h", Some("avcodec"), "FF_API_ERROR_RATE"),
            (
                "libavcodec/avcodec.h",
                Some("avcodec"),
                "FF_API_QSCALE_TYPE",
            ),
            ("libavcodec/avcodec.h", Some("avcodec"), "FF_API_MB_TYPE"),
            (
                "libavcodec/avcodec.h",
                Some("avcodec"),
                "FF_API_MAX_BFRAMES",
            ),
            (
                "libavcodec/avcodec.h",
                Some("avcodec"),
                "FF_API_NEG_LINESIZES",
            ),
            ("libavcodec/avcodec.h", Some("avcodec"), "FF_API_EMU_EDGE"),
            ("libavcodec/avcodec.h", Some("avcodec"), "FF_API_ARCH_SH4"),
            ("libavcodec/avcodec.h", Some("avcodec"), "FF_API_ARCH_SPARC"),
            (
                "libavcodec/avcodec.h",
                Some("avcodec"),
                "FF_API_UNUSED_MEMBERS",
            ),
            (
                "libavcodec/avcodec.h",
                Some("avcodec"),
                "FF_API_IDCT_XVIDMMX",
            ),
            (
                "libavcodec/avcodec.h",
                Some("avcodec"),
                "FF_API_INPUT_PRESERVED",
            ),
            (
                "libavcodec/avcodec.h",
                Some("avcodec"),
                "FF_API_NORMALIZE_AQP",
            ),
            ("libavcodec/avcodec.h", Some("avcodec"), "FF_API_GMC"),
            ("libavcodec/avcodec.h", Some("avcodec"), "FF_API_MV0"),
            ("libavcodec/avcodec.h", Some("avcodec"), "FF_API_CODEC_NAME"),
            ("libavcodec/avcodec.h", Some("avcodec"), "FF_API_AFD"),
            ("libavcodec/avcodec.h", Some("avcodec"), "FF_API_VISMV"),
            (
                "libavcodec/avcodec.h",
                Some("avcodec"),
                "FF_API_DV_FRAME_PROFILE",
            ),
            (
                "libavcodec/avcodec.h",
                Some("avcodec"),
                "FF_API_AUDIOENC_DELAY",
            ),
            (
                "libavcodec/avcodec.h",
                Some("avcodec"),
                "FF_API_VAAPI_CONTEXT",
            ),
            (
                "libavcodec/avcodec.h",
                Some("avcodec"),
                "FF_API_AVCTX_TIMEBASE",
            ),
            ("libavcodec/avcodec.h", Some("avcodec"), "FF_API_MPV_OPT"),
            (
                "libavcodec/avcodec.h",
                Some("avcodec"),
                "FF_API_STREAM_CODEC_TAG",
            ),
            ("libavcodec/avcodec.h", Some("avcodec"), "FF_API_QUANT_BIAS"),
            (
                "libavcodec/avcodec.h",
                Some("avcodec"),
                "FF_API_RC_STRATEGY",
            ),
            (
                "libavcodec/avcodec.h",
                Some("avcodec"),
                "FF_API_CODED_FRAME",
            ),
            ("libavcodec/avcodec.h", Some("avcodec"), "FF_API_MOTION_EST"),
            (
                "libavcodec/avcodec.h",
                Some("avcodec"),
                "FF_API_WITHOUT_PREFIX",
            ),
            (
                "libavcodec/avcodec.h",
                Some("avcodec"),
                "FF_API_CONVERGENCE_DURATION",
            ),
            (
                "libavcodec/avcodec.h",
                Some("avcodec"),
                "FF_API_PRIVATE_OPT",
            ),
            ("libavcodec/avcodec.h", Some("avcodec"), "FF_API_CODER_TYPE"),
            (
                "libavcodec/avcodec.h",
                Some("avcodec"),
                "FF_API_RTP_CALLBACK",
            ),
            ("libavcodec/avcodec.h", Some("avcodec"), "FF_API_STAT_BITS"),
            ("libavcodec/avcodec.h", Some("avcodec"), "FF_API_VBV_DELAY"),
            (
                "libavcodec/avcodec.h",
                Some("avcodec"),
                "FF_API_SIDEDATA_ONLY_PKT",
            ),
            ("libavcodec/avcodec.h", Some("avcodec"), "FF_API_AVPICTURE"),
            (
                "libavformat/avformat.h",
                Some("avformat"),
                "FF_API_LAVF_BITEXACT",
            ),
            (
                "libavformat/avformat.h",
                Some("avformat"),
                "FF_API_LAVF_FRAC",
            ),
            (
                "libavformat/avformat.h",
                Some("avformat"),
                "FF_API_URL_FEOF",
            ),
            (
                "libavformat/avformat.h",
                Some("avformat"),
                "FF_API_PROBESIZE_32",
            ),
            (
                "libavformat/avformat.h",
                Some("avformat"),
                "FF_API_LAVF_AVCTX",
            ),
            (
                "libavformat/avformat.h",
                Some("avformat"),
                "FF_API_OLD_OPEN_CALLBACKS",
            ),
            (
                "libavfilter/avfilter.h",
                Some("avfilter"),
                "FF_API_AVFILTERPAD_PUBLIC",
            ),
            (
                "libavfilter/avfilter.h",
                Some("avfilter"),
                "FF_API_FOO_COUNT",
            ),
            (
                "libavfilter/avfilter.h",
                Some("avfilter"),
                "FF_API_OLD_FILTER_OPTS",
            ),
            (
                "libavfilter/avfilter.h",
                Some("avfilter"),
                "FF_API_OLD_FILTER_OPTS_ERROR",
            ),
            (
                "libavfilter/avfilter.h",
                Some("avfilter"),
                "FF_API_AVFILTER_OPEN",
            ),
            (
                "libavfilter/avfilter.h",
                Some("avfilter"),
                "FF_API_OLD_FILTER_REGISTER",
            ),
            (
                "libavfilter/avfilter.h",
                Some("avfilter"),
                "FF_API_OLD_GRAPH_PARSE",
            ),
            (
                "libavfilter/avfilter.h",
                Some("avfilter"),
                "FF_API_NOCONST_GET_NAME",
            ),
            (
                "libavresample/avresample.h",
                Some("avresample"),
                "FF_API_RESAMPLE_CLOSE_OPEN",
            ),
            (
                "libswscale/swscale.h",
                Some("swscale"),
                "FF_API_SWS_CPU_CAPS",
            ),
            ("libswscale/swscale.h", Some("swscale"), "FF_API_ARCH_BFIN"),
        ],
    );

    let clang_includes = include_paths
        .iter()
        .map(|include| format!("-I{}", include.to_string_lossy()));

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let mut builder = bindgen::Builder::default()
        .clang_args(clang_includes)
        .ctypes_prefix("libc")
        // https://github.com/rust-lang/rust-bindgen/issues/550
        .blacklist_type("max_align_t")
        // these are never part of ffmpeg API
        .blacklist_function("_.*")
        // Rust doesn't support long double, and bindgen can't skip it
        // https://github.com/rust-lang/rust-bindgen/issues/1549
        .blacklist_function("acoshl")
        .blacklist_function("acosl")
        .blacklist_function("asinhl")
        .blacklist_function("asinl")
        .blacklist_function("atan2l")
        .blacklist_function("atanhl")
        .blacklist_function("atanl")
        .blacklist_function("cbrtl")
        .blacklist_function("ceill")
        .blacklist_function("copysignl")
        .blacklist_function("coshl")
        .blacklist_function("cosl")
        .blacklist_function("dreml")
        .blacklist_function("ecvt_r")
        .blacklist_function("erfcl")
        .blacklist_function("erfl")
        .blacklist_function("exp2l")
        .blacklist_function("expl")
        .blacklist_function("expm1l")
        .blacklist_function("fabsl")
        .blacklist_function("fcvt_r")
        .blacklist_function("fdiml")
        .blacklist_function("finitel")
        .blacklist_function("floorl")
        .blacklist_function("fmal")
        .blacklist_function("fmaxl")
        .blacklist_function("fminl")
        .blacklist_function("fmodl")
        .blacklist_function("frexpl")
        .blacklist_function("gammal")
        .blacklist_function("hypotl")
        .blacklist_function("ilogbl")
        .blacklist_function("isinfl")
        .blacklist_function("isnanl")
        .blacklist_function("j0l")
        .blacklist_function("j1l")
        .blacklist_function("jnl")
        .blacklist_function("ldexpl")
        .blacklist_function("lgammal")
        .blacklist_function("lgammal_r")
        .blacklist_function("llrintl")
        .blacklist_function("llroundl")
        .blacklist_function("log10l")
        .blacklist_function("log1pl")
        .blacklist_function("log2l")
        .blacklist_function("logbl")
        .blacklist_function("logl")
        .blacklist_function("lrintl")
        .blacklist_function("lroundl")
        .blacklist_function("modfl")
        .blacklist_function("nanl")
        .blacklist_function("nearbyintl")
        .blacklist_function("nextafterl")
        .blacklist_function("nexttoward")
        .blacklist_function("nexttowardf")
        .blacklist_function("nexttowardl")
        .blacklist_function("powl")
        .blacklist_function("qecvt")
        .blacklist_function("qecvt_r")
        .blacklist_function("qfcvt")
        .blacklist_function("qfcvt_r")
        .blacklist_function("qgcvt")
        .blacklist_function("remainderl")
        .blacklist_function("remquol")
        .blacklist_function("rintl")
        .blacklist_function("roundl")
        .blacklist_function("scalbl")
        .blacklist_function("scalblnl")
        .blacklist_function("scalbnl")
        .blacklist_function("significandl")
        .blacklist_function("sinhl")
        .blacklist_function("sinl")
        .blacklist_function("sqrtl")
        .blacklist_function("strtold")
        .blacklist_function("tanhl")
        .blacklist_function("tanl")
        .blacklist_function("tgammal")
        .blacklist_function("truncl")
        .blacklist_function("y0l")
        .blacklist_function("y1l")
        .blacklist_function("ynl")
        .rustified_enum("*")
        .prepend_enum_name(false)
        .derive_eq(true)
        .size_t_is_usize(true)
        .parse_callbacks(Box::new(Callbacks));

    // The input headers we would like to generate
    // bindings for.
    if env::var("CARGO_FEATURE_AVCODEC").is_ok() {
        builder = builder
            .header(search_include(&include_paths, "libavcodec/avcodec.h"))
            .header(search_include(&include_paths, "libavcodec/dv_profile.h"))
            .header(search_include(&include_paths, "libavcodec/avfft.h"))
            .header(search_include(&include_paths, "libavcodec/vaapi.h"))
            .header(search_include(&include_paths, "libavcodec/vorbis_parser.h"));
    }

    if env::var("CARGO_FEATURE_AVDEVICE").is_ok() {
        builder = builder.header(search_include(&include_paths, "libavdevice/avdevice.h"));
    }

    if env::var("CARGO_FEATURE_AVFILTER").is_ok() {
        builder = builder
            .header(search_include(&include_paths, "libavfilter/buffersink.h"))
            .header(search_include(&include_paths, "libavfilter/buffersrc.h"))
            .header(search_include(&include_paths, "libavfilter/avfilter.h"));
    }

    if env::var("CARGO_FEATURE_AVFORMAT").is_ok() {
        builder = builder
            .header(search_include(&include_paths, "libavformat/avformat.h"))
            .header(search_include(&include_paths, "libavformat/avio.h"));
    }

    if env::var("CARGO_FEATURE_AVRESAMPLE").is_ok() {
        builder = builder.header(search_include(&include_paths, "libavresample/avresample.h"));
    }

    builder = builder
        .header(search_include(&include_paths, "libavutil/adler32.h"))
        .header(search_include(&include_paths, "libavutil/aes.h"))
        .header(search_include(&include_paths, "libavutil/audio_fifo.h"))
        .header(search_include(&include_paths, "libavutil/base64.h"))
        .header(search_include(&include_paths, "libavutil/blowfish.h"))
        .header(search_include(&include_paths, "libavutil/bprint.h"))
        .header(search_include(&include_paths, "libavutil/buffer.h"))
        .header(search_include(&include_paths, "libavutil/camellia.h"))
        .header(search_include(&include_paths, "libavutil/cast5.h"))
        .header(search_include(&include_paths, "libavutil/channel_layout.h"))
        .header(search_include(&include_paths, "libavutil/cpu.h"))
        .header(search_include(&include_paths, "libavutil/crc.h"))
        .header(search_include(&include_paths, "libavutil/dict.h"))
        .header(search_include(&include_paths, "libavutil/display.h"))
        .header(search_include(&include_paths, "libavutil/downmix_info.h"))
        .header(search_include(&include_paths, "libavutil/error.h"))
        .header(search_include(&include_paths, "libavutil/eval.h"))
        .header(search_include(&include_paths, "libavutil/fifo.h"))
        .header(search_include(&include_paths, "libavutil/file.h"))
        .header(search_include(&include_paths, "libavutil/frame.h"))
        .header(search_include(&include_paths, "libavutil/hash.h"))
        .header(search_include(&include_paths, "libavutil/hmac.h"))
        .header(search_include(&include_paths, "libavutil/imgutils.h"))
        .header(search_include(&include_paths, "libavutil/lfg.h"))
        .header(search_include(&include_paths, "libavutil/log.h"))
        .header(search_include(&include_paths, "libavutil/lzo.h"))
        .header(search_include(&include_paths, "libavutil/macros.h"))
        .header(search_include(&include_paths, "libavutil/mathematics.h"))
        .header(search_include(&include_paths, "libavutil/md5.h"))
        .header(search_include(&include_paths, "libavutil/mem.h"))
        .header(search_include(&include_paths, "libavutil/motion_vector.h"))
        .header(search_include(&include_paths, "libavutil/murmur3.h"))
        .header(search_include(&include_paths, "libavutil/opt.h"))
        .header(search_include(&include_paths, "libavutil/parseutils.h"))
        .header(search_include(&include_paths, "libavutil/pixdesc.h"))
        .header(search_include(&include_paths, "libavutil/pixfmt.h"))
        .header(search_include(&include_paths, "libavutil/random_seed.h"))
        .header(search_include(&include_paths, "libavutil/rational.h"))
        .header(search_include(&include_paths, "libavutil/replaygain.h"))
        .header(search_include(&include_paths, "libavutil/ripemd.h"))
        .header(search_include(&include_paths, "libavutil/samplefmt.h"))
        .header(search_include(&include_paths, "libavutil/sha.h"))
        .header(search_include(&include_paths, "libavutil/sha512.h"))
        .header(search_include(&include_paths, "libavutil/stereo3d.h"))
        .header(search_include(&include_paths, "libavutil/avstring.h"))
        .header(search_include(&include_paths, "libavutil/threadmessage.h"))
        .header(search_include(&include_paths, "libavutil/time.h"))
        .header(search_include(&include_paths, "libavutil/timecode.h"))
        .header(search_include(&include_paths, "libavutil/twofish.h"))
        .header(search_include(&include_paths, "libavutil/avutil.h"))
        .header(search_include(&include_paths, "libavutil/xtea.h"))
        .header(search_include(&include_paths, "libavutil/hwcontext.h"));

    if env::var("CARGO_FEATURE_POSTPROC").is_ok() {
        builder = builder.header(search_include(&include_paths, "libpostproc/postprocess.h"));
    }

    if env::var("CARGO_FEATURE_SWRESAMPLE").is_ok() {
        builder = builder.header(search_include(&include_paths, "libswresample/swresample.h"));
    }

    if env::var("CARGO_FEATURE_SWSCALE").is_ok() {
        builder = builder.header(search_include(&include_paths, "libswscale/swscale.h"));
    }

    if env::var("CARGO_FEATURE_LIB_DRM").is_ok() {
        builder = builder.header(search_include(&include_paths, "libavutil/hwcontext_drm.h"))
    }

    // Finish the builder and generate the bindings.
    let bindings = builder
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(output().join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn print_pkg_config_libs(statik: bool, lib: &pkg_config::Library) {
    let target = env::var("TARGET").unwrap();
    let is_msvc = target.contains("msvc");
    let is_apple = target.contains("apple");

    for val in &lib.link_paths {
        println!("cargo:rustc-link-search=native={}", val.display());
    }
    for val in &lib.framework_paths {
        println!("cargo:rustc-link-search=framework={}", val.display());
    }
    for val in &lib.frameworks {
        println!("cargo:rustc-link=framework={}", val);
    }

    for val in &lib.libs {
        if is_msvc && ["m", "c", "pthread"].contains(&val.as_str()) {
            continue;
        }
        if is_apple && val == "stdc++" {
            println!("cargo:rustc-link-lib=c++");
            continue;
        }

        if statik && is_static_available(val, &lib.include_paths) {
            println!("cargo:rustc-link-lib=static={}", val);
        } else {
            println!("cargo:rustc-link-lib={}", val);
        }
    }
}

fn is_static_available(lib: &str, dirs: &[PathBuf]) -> bool {
    let libname = format!("lib{}.a", lib);
    let has = dirs
        .iter()
        .map(|d| d.as_path())
        .chain([Path::new("/usr/local/lib")].iter().copied())
        .any(|dir| dir.join(&libname).exists());
    if !has {
        println!("cargo:warning=static {} not found", libname);
    }
    has
}
