extern crate num_cpus;

use std::env;
use std::fs::{self, File};
use std::io::{self, Write, BufRead, BufReader};
use std::path::PathBuf;
use std::process::Command;
use std::str;

fn version() -> String {
	let major: u8 = env::var("CARGO_PKG_VERSION_MAJOR").unwrap().parse().unwrap();
	let minor: u8 = env::var("CARGO_PKG_VERSION_MINOR").unwrap().parse().unwrap();

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
	let status = try!(Command::new("git")
		.current_dir(&output())
		.arg("clone")
		.arg("-b")
		.arg(format!("release/{}", version()))
		.arg("https://github.com/FFmpeg/FFmpeg")
		.arg(format!("ffmpeg-{}", version()))
		.status());

	if status.success() {
		Ok(())
	}
	else {
		Err(io::Error::new(io::ErrorKind::Other, "fetch failed"))
	}
}

fn build() -> io::Result<()> {
	let mut configure = Command::new("./configure");
	configure.current_dir(&source());
	configure.arg(format!("--prefix={}", search().to_string_lossy()));

	if env::var("TARGET").unwrap() != env::var("HOST").unwrap() {
		configure.arg(format!("--cross-prefix={}-", env::var("TARGET").unwrap()));
	}

	// control debug build
	if env::var("DEBUG").is_ok() {
		configure.arg("--enable-debug");
		configure.arg("--disable-stripping");
	}
	else {
		configure.arg("--disable-debug");
		configure.arg("--enable-stripping");
	}

	// make it static
	configure.arg("--enable-static");
	configure.arg("--disable-shared");

	// do not build programs since we don't need them
	configure.arg("--disable-programs");

	macro_rules! switch {
		($conf:expr, $feat:expr, $name:expr) => (
			if env::var(concat!("CARGO_FEATURE_", $feat)).is_ok() {
				$conf.arg(concat!("--enable-", $name));
			}
			else {
				$conf.arg(concat!("--disable-", $name));
			}
		)
	}

	macro_rules! enable {
		($conf:expr, $feat:expr, $name:expr) => (
			if env::var(concat!("CARGO_FEATURE_", $feat)).is_ok() {
				$conf.arg(concat!("--enable-", $name));
			}
		)
	}

	macro_rules! disable {
		($conf:expr, $feat:expr, $name:expr) => (
			if env::var(concat!("CARGO_FEATURE_", $feat)).is_err() {
				$conf.arg(concat!("--disable-", $name));
			}
		)
	}

	// the binary using ffmpeg-sys must comply with GPL
	switch!(configure, "BUILD_LICENSE_GPL", "gpl");

	// the binary using ffmpeg-sys must comply with (L)GPLv3
	switch!(configure, "BUILD_LICENSE_VERSION3", "version3");

	// the binary using ffmpeg-sys cannot be redistributed
	switch!(configure, "BUILD_LICENSE_NONFREE", "nonfree");

	// configure building libraries based on features
	switch!(configure, "AVCODEC",    "avcodec");
	switch!(configure, "AVDEVICE",   "avdevice");
	switch!(configure, "AVFILTER",   "avfilter");
	switch!(configure, "AVFORMAT",   "avformat");
	switch!(configure, "AVRESAMPLE", "avresample");
	switch!(configure, "POSTPROC",   "postproc");
	switch!(configure, "SWRESAMPLE", "swresample");
	switch!(configure, "SWSCALE",    "swscale");

	// configure external SSL libraries
	enable!(configure, "BUILD_LIB_GNUTLS",  "gnutls");
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
	enable!(configure, "BUILD_LIB_OPENCORE_AMRWB", "libopencore-amrwrb");
	enable!(configure, "BUILD_LIB_OPENH264", "libopenh264");
	enable!(configure, "BUILD_LIB_OPENH265", "libopenh265");
	enable!(configure, "BUILD_LIB_OPENJPEG", "libopenjpeg");
	enable!(configure, "BUILD_LIB_OPUS", "libopus");
	enable!(configure, "BUILD_LIB_SCHROEDINGER", "libschroedinger");
	enable!(configure, "BUILD_LIB_SHINE", "libshine");
	enable!(configure, "BUILD_LIB_SNAPPY", "libsnappy");
	enable!(configure, "BUILD_LIB_SPEEX", "libspeex");
	enable!(configure, "BUILD_LIB_STAGEFRIGHT_H264", "libstagefright-h264");
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

	// other external libraries
	enable!(configure, "BUILD_NVENC", "nvenc");

	// configure external protocols
	enable!(configure, "BUILD_LIB_SMBCLIENT", "libsmbclient");
	enable!(configure, "BUILD_LIB_SSH", "libssh");

	// configure misc build options
	enable!(configure, "BUILD_PIC", "pic");

	// run ./configure
	if !try!(configure.status()).success() {
		return Err(io::Error::new(io::ErrorKind::Other, "configure failed"));
	}

	// run make
	if !try!(Command::new("make")
		.arg("-j").arg(num_cpus::get().to_string())
		.current_dir(&source()).status()).success() {
		return Err(io::Error::new(io::ErrorKind::Other, "make failed"));
	}

	// run make install
	if !try!(Command::new("make").current_dir(&source()).arg("install").status()).success() {
		return Err(io::Error::new(io::ErrorKind::Other, "make install failed"));
	}

	Ok(())
}

fn check_features(infos: &Vec<(&'static str, Option<&'static str>, &'static str)>) {
	let mut includes_code = String::new();
	let mut main_code = String::new();

	for &(header, feature, var) in infos {
		if let Some(feature) = feature {
			if env::var(format!("CARGO_FEATURE_{}", feature.to_uppercase())).is_err() {
				continue
			}
		}

		let include = format!("#include <{}>", header);
		if includes_code.find(&include).is_none() {
			includes_code.push_str(&include);
			includes_code.push_str(&"\n");
		}
		includes_code.push_str(&format!(r#"
			#ifndef {var}
			#define {var} 0
			#define {var}_is_defined 0
			#else
			#define {var}_is_defined 1
			#endif
		"#, var=var));

		main_code.push_str(&format!(r#"printf("[{var}]%d%d\n", {var}, {var}_is_defined);"#, var=var));
	}

	let version_check_info = [("avcodec", 56, 60, 0, 80)];
	for &(lib, begin_version_major, end_version_major, begin_version_minor, end_version_minor) in version_check_info.iter() {
		for version_major in begin_version_major..end_version_major {
			for version_minor in begin_version_minor..end_version_minor {
				main_code.push_str(&format!(r#"printf("[{lib}_version_greater_than_{version_major}_{version_minor}]%d\n", LIB{lib_uppercase}_VERSION_MAJOR > {version_major} || (LIB{lib_uppercase}_VERSION_MAJOR == {version_major} && LIB{lib_uppercase}_VERSION_MINOR > {version_minor}));"#,
				                            lib=lib,
				                            lib_uppercase=lib.to_uppercase(),
				                            version_major=version_major,
				                            version_minor=version_minor));

			}
		}
	}

	let out_dir = output();

	write!(File::create(out_dir.join("check.c")).expect("Failed to create file"), r#"
		#include <stdio.h>
		{includes_code}

		int main()
		{{
			{main_code}
			return 0;
		}}
	"#, includes_code=includes_code, main_code=main_code).expect("Write failed");

	let executable = out_dir.join(if cfg!(windows) { "check.exe" } else { "check" });
	let compiler =
		if cfg!(windows) || env::var("MSYSTEM").unwrap_or("".to_string()).starts_with("MINGW32") {
			"gcc"
		}
		else {
			"cc"
		};

	if !Command::new(compiler).current_dir(&out_dir)
		.arg("-I").arg(search().join("include").to_string_lossy().into_owned())
		.arg("-o").arg(&executable)
		.arg("check.c")
		.status().expect("Command failed").success() {
		panic!("Compile failed");
	}

	let stdout_raw = Command::new(out_dir.join(&executable)).current_dir(&out_dir).output().expect("Check failed").stdout;
	let stdout = str::from_utf8(stdout_raw.as_slice()).unwrap();

	println!("stdout={}",stdout);

	for &(_, feature, var) in infos {
		if let Some(feature) = feature {
			if env::var(format!("CARGO_FEATURE_{}", feature.to_uppercase())).is_err() {
				continue
			}
		}

		let var_str = format!("[{var}]", var=var);
		let pos = stdout.find(&var_str).expect("Variable not found in output") + var_str.len();
		if &stdout[pos..pos+1] == "1" {
			println!(r#"cargo:rustc-cfg=feature="{}""#, var.to_lowercase());
			println!(r#"cargo:{}=true"#, var.to_lowercase());
		}

		// Also find out if defined or not (useful for cases where only the definition of a macro
		// can be used as distinction)
		if &stdout[pos+1..pos+2] == "1" {
			println!(r#"cargo:rustc-cfg=feature="{}_is_defined""#, var.to_lowercase());
			println!(r#"cargo:{}_is_defined=true"#, var.to_lowercase());
		}
	}

	for &(lib, begin_version_major, end_version_major, begin_version_minor, end_version_minor) in version_check_info.iter() {
		for version_major in begin_version_major..end_version_major {
			for version_minor in begin_version_minor..end_version_minor {
				let search_str = format!("[{lib}_version_greater_than_{version_major}_{version_minor}]",
				                         version_major=version_major,
				                         version_minor=version_minor,
				                         lib=lib);
				let pos = stdout.find(&search_str).expect("Variable not found in output") + search_str.len();
 				if &stdout[pos..pos+1] == "1" {
 					println!(r#"cargo:rustc-cfg=feature="{}""#, &search_str[1..(search_str.len() - 1)]);
 				}
			}
		}
	}
}

fn main() {
	if env::var("CARGO_FEATURE_BUILD").is_ok() {
		println!("cargo:rustc-link-search=native={}", search().join("lib").to_string_lossy());

		if env::var("CARGO_FEATURE_BUILD_ZLIB").is_ok() && cfg!(target_os = "linux") {
			println!("cargo:rustc-link-lib=z");
		}

		if fs::metadata(&search().join("lib").join("libavutil.a")).is_err() {
			fs::create_dir_all(&output()).ok().expect("failed to create build directory");
			fetch().unwrap();
			build().unwrap();
		}

		// Check additional required libraries.
		{
			let config_mak = source().join("config.mak");
			let file = File::open(config_mak).unwrap();
			let reader = BufReader::new(file);
			let extra_libs = reader.lines()
				.find(|ref line| line.as_ref().unwrap().starts_with("EXTRALIBS"))
				.map(|line| line.unwrap()).unwrap();

			let linker_args = extra_libs.split('=').last().unwrap().split(' ');
			let include_libs = linker_args.filter(|v| v.starts_with("-l")).map(|flag| &flag[2..]);

			for lib in include_libs {
				println!("cargo:rustc-link-lib={}", lib);
			}
		}
	}

	check_features(&vec![
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
		("libavutil/avutil.h", None, "FF_API_GET_CHANNEL_LAYOUT_COMPAT"),
		("libavutil/avutil.h", None, "FF_API_XVMC"),
		("libavutil/avutil.h", None, "FF_API_OPT_TYPE_METADATA"),
		("libavutil/avutil.h", None, "FF_API_DLOG"),
		("libavutil/avutil.h", None, "FF_API_HMAC"),
		("libavutil/avutil.h", None, "FF_API_VAAPI"),

		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_VIMA_DECODER"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_REQUEST_CHANNELS"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_OLD_DECODE_AUDIO"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_OLD_ENCODE_AUDIO"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_OLD_ENCODE_VIDEO"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_CODEC_ID"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_AUDIO_CONVERT"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_AVCODEC_RESAMPLE"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_DEINTERLACE"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_DESTRUCT_PACKET"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_GET_BUFFER"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_MISSING_SAMPLE"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_LOWRES"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_CAP_VDPAU"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_BUFS_VDPAU"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_VOXWARE"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_SET_DIMENSIONS"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_DEBUG_MV"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_AC_VLC"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_OLD_MSMPEG4"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_ASPECT_EXTENDED"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_THREAD_OPAQUE"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_CODEC_PKT"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_ARCH_ALPHA"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_XVMC"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_ERROR_RATE"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_QSCALE_TYPE"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_MB_TYPE"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_MAX_BFRAMES"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_NEG_LINESIZES"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_EMU_EDGE"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_ARCH_SH4"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_ARCH_SPARC"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_UNUSED_MEMBERS"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_IDCT_XVIDMMX"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_INPUT_PRESERVED"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_NORMALIZE_AQP"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_GMC"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_MV0"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_CODEC_NAME"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_AFD"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_VISMV"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_DV_FRAME_PROFILE"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_AUDIOENC_DELAY"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_VAAPI_CONTEXT"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_AVCTX_TIMEBASE"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_MPV_OPT"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_STREAM_CODEC_TAG"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_QUANT_BIAS"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_RC_STRATEGY"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_CODED_FRAME"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_MOTION_EST"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_WITHOUT_PREFIX"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_CONVERGENCE_DURATION"),
		("libavcodec/avcodec.h", Some("avcodec"), "FF_API_PRIVATE_OPT"),

		("libavformat/avformat.h", Some("avformat"), "FF_API_LAVF_BITEXACT"),
		("libavformat/avformat.h", Some("avformat"), "FF_API_LAVF_FRAC"),
		("libavformat/avformat.h", Some("avformat"), "FF_API_URL_FEOF"),
		("libavformat/avformat.h", Some("avformat"), "FF_API_PROBESIZE_32"),

		("libavfilter/avfilter.h", Some("avfilter"), "FF_API_AVFILTERPAD_PUBLIC"),
		("libavfilter/avfilter.h", Some("avfilter"), "FF_API_FOO_COUNT"),
		("libavfilter/avfilter.h", Some("avfilter"), "FF_API_AVFILTERBUFFER"),
		("libavfilter/avfilter.h", Some("avfilter"), "FF_API_OLD_FILTER_OPTS"),
		("libavfilter/avfilter.h", Some("avfilter"), "FF_API_OLD_FILTER_OPTS_ERROR"),
		("libavfilter/avfilter.h", Some("avfilter"), "FF_API_AVFILTER_OPEN"),
		("libavfilter/avfilter.h", Some("avfilter"), "FF_API_OLD_FILTER_REGISTER"),
		("libavfilter/avfilter.h", Some("avfilter"), "FF_API_OLD_GRAPH_PARSE"),
		("libavfilter/avfilter.h", Some("avfilter"), "FF_API_NOCONST_GET_NAME"),

		("libavresample/avresample.h", Some("avresample"), "FF_API_RESAMPLE_CLOSE_OPEN"),

		("libswscale/swscale.h", Some("swscale"), "FF_API_SWS_CPU_CAPS"),
		("libswscale/swscale.h", Some("swscale"), "FF_API_ARCH_BFIN"),
	]);
}
