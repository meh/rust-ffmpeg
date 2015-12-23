use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

fn version() -> String {
	let major: u8 = env::var("CARGO_PKG_VERSION_MAJOR").unwrap().parse().unwrap();
	let minor: u8 = env::var("CARGO_PKG_VERSION_MINOR").unwrap().parse().unwrap();
	let patch: u8 = env::var("CARGO_PKG_VERSION_PATCH").unwrap().parse().unwrap();

	if patch == 0 {
		format!("{}.{}", major, minor)
	}
	else {
		format!("{}.{}.{}", major, minor, patch)
	}
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
	let url    = format!("http://ffmpeg.org/releases/ffmpeg-{}.tar.bz2", version());
	let status = try!(if cfg!(target_os = "linux") {
		Command::new("wget")
			.current_dir(&output())
			.arg(url)
			.arg("-c")
			.arg("-O")
			.arg("ffmpeg.tar.bz2")
			.status()
	}
	else {
		unimplemented!();
	});

	if status.success() {
		Ok(())
	}
	else {
		Err(io::Error::new(io::ErrorKind::Other, "fetch failed"))
	}
}

fn extract() -> io::Result<()> {
	let status = try!(if cfg!(target_os = "linux") {
		Command::new("tar")
			.current_dir(&output())
			.arg("xf")
			.arg("ffmpeg.tar.bz2")
			.status()
	}
	else {
		unimplemented!();
	});

	if status.success() {
		Ok(())
	}
	else {
		Err(io::Error::new(io::ErrorKind::Other, "extract failed"))
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

	// configure external protocols
	enable!(configure, "BUILD_LIB_SMBCLIENT", "libsmbclient");
	enable!(configure, "BUILD_LIB_SSH", "libssh");

	// run ./configure
	if !try!(configure.status()).success() {
		return Err(io::Error::new(io::ErrorKind::Other, "configure failed"));
	}

	// run make
	if !try!(Command::new("make").current_dir(&source()).status()).success() {
		return Err(io::Error::new(io::ErrorKind::Other, "make failed"));
	}

	// run make install
	if !try!(Command::new("make").current_dir(&source()).arg("install").status()).success() {
		return Err(io::Error::new(io::ErrorKind::Other, "make install failed"));
	}

	Ok(())
}

fn feature(header: &str, feature: Option<&str>, var: &str) -> io::Result<()> {
	if let Some(feature) = feature {
		if env::var(format!("CARGO_FEATURE_{}", feature.to_uppercase())).is_err() {
			return Ok(());
		}
	}

	try!(write!(try!(File::create(output().join("check.c"))), r#"
		#include <stdio.h>
		#include <{header}>

		#ifndef {var}
		#define {var} 0
		#endif

		int
		main (int argc, char* argv[])
		{{
			printf("%d\n", {var});
			return 0;
		}}
	"#, header=header, var=var));

	if Command::new("cc").current_dir(&output())
		.arg("-I").arg(search().join("dist").join("include").to_string_lossy().into_owned())
		.arg("-o").arg("check")
		.arg("check.c")
		.status().is_err()
	{
		return Ok(());
	}

	if try!(Command::new("./check").current_dir(&output()).output()).stdout[0] == b'1' {
		println!(r#"cargo:rustc-cfg=feature="{}""#, var.to_lowercase());
		println!(r#"cargo:{}=true"#, var.to_lowercase());
	}

	Ok(())
}

fn main() {
	if env::var("CARGO_FEATURE_BUILD").is_ok() {
		println!("cargo:rustc-link-search=native={}", search().join("lib").to_string_lossy());

		if fs::metadata(&search().join("lib").join("libavutil.a")).is_ok() {
			return;
		}

		fs::create_dir_all(&output()).ok().expect("failed to create build directory");
		fetch().unwrap();
		extract().unwrap();
		build().unwrap();
	}

	feature("libavutil/avutil.h", None, "FF_API_OLD_AVOPTIONS").unwrap();
	feature("libavutil/avutil.h", None, "FF_API_PIX_FMT").unwrap();
	feature("libavutil/avutil.h", None, "FF_API_CONTEXT_SIZE").unwrap();
	feature("libavutil/avutil.h", None, "FF_API_PIX_FMT_DESC").unwrap();
	feature("libavutil/avutil.h", None, "FF_API_AV_REVERSE").unwrap();
	feature("libavutil/avutil.h", None, "FF_API_AUDIOCONVERT").unwrap();
	feature("libavutil/avutil.h", None, "FF_API_CPU_FLAG_MMX2").unwrap();
	feature("libavutil/avutil.h", None, "FF_API_LLS_PRIVATE").unwrap();
	feature("libavutil/avutil.h", None, "FF_API_AVFRAME_LAVC").unwrap();
	feature("libavutil/avutil.h", None, "FF_API_VDPAU").unwrap();
	feature("libavutil/avutil.h", None, "FF_API_GET_CHANNEL_LAYOUT_COMPAT").unwrap();
	feature("libavutil/avutil.h", None, "FF_API_XVMC").unwrap();
	feature("libavutil/avutil.h", None, "FF_API_OPT_TYPE_METADATA").unwrap();
	feature("libavutil/avutil.h", None, "FF_API_DLOG").unwrap();
	feature("libavutil/avutil.h", None, "FF_API_HMAC").unwrap();
	feature("libavutil/avutil.h", None, "FF_API_VAAPI").unwrap();

	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_VIMA_DECODER").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_REQUEST_CHANNELS").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_OLD_DECODE_AUDIO").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_OLD_ENCODE_AUDIO").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_OLD_ENCODE_VIDEO").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_CODEC_ID").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_AUDIO_CONVERT").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_AVCODEC_RESAMPLE").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_DEINTERLACE").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_DESTRUCT_PACKET").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_GET_BUFFER").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_MISSING_SAMPLE").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_LOWRES").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_CAP_VDPAU").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_BUFS_VDPAU").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_VOXWARE").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_SET_DIMENSIONS").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_DEBUG_MV").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_AC_VLC").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_OLD_MSMPEG4").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_ASPECT_EXTENDED").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_THREAD_OPAQUE").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_CODEC_PKT").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_ARCH_ALPHA").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_XVMC").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_ERROR_RATE").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_QSCALE_TYPE").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_MB_TYPE").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_MAX_BFRAMES").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_NEG_LINESIZES").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_EMU_EDGE").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_ARCH_SH4").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_ARCH_SPARC").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_UNUSED_MEMBERS").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_IDCT_XVIDMMX").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_INPUT_PRESERVED").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_NORMALIZE_AQP").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_GMC").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_MV0").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_CODEC_NAME").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_AFD").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_VISMV").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_DV_FRAME_PROFILE").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_AUDIOENC_DELAY").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_VAAPI_CONTEXT").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_AVCTX_TIMEBASE").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_MPV_OPT").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_STREAM_CODEC_TAG").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_QUANT_BIAS").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_RC_STRATEGY").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_CODED_FRAME").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_MOTION_EST").unwrap();
	feature("libavcodec/avcodec.h", Some("avcodec"), "FF_API_WITHOUT_PREFIX").unwrap();

	feature("libavformat/avformat.h", Some("avformat"), "FF_API_LAVF_BITEXACT").unwrap();
	feature("libavformat/avformat.h", Some("avformat"), "FF_API_LAVF_FRAC").unwrap();
	feature("libavformat/avformat.h", Some("avformat"), "FF_API_URL_FEOF").unwrap();
	feature("libavformat/avformat.h", Some("avformat"), "FF_API_PROBESIZE_32").unwrap();

	feature("libavfilter/avfilter.h", Some("avfilter"), "FF_API_AVFILTERPAD_PUBLIC").unwrap();
	feature("libavfilter/avfilter.h", Some("avfilter"), "FF_API_FOO_COUNT").unwrap();
	feature("libavfilter/avfilter.h", Some("avfilter"), "FF_API_AVFILTERBUFFER").unwrap();
	feature("libavfilter/avfilter.h", Some("avfilter"), "FF_API_OLD_FILTER_OPTS").unwrap();
	feature("libavfilter/avfilter.h", Some("avfilter"), "FF_API_OLD_FILTER_OPTS_ERROR").unwrap();
	feature("libavfilter/avfilter.h", Some("avfilter"), "FF_API_AVFILTER_OPEN").unwrap();
	feature("libavfilter/avfilter.h", Some("avfilter"), "FF_API_OLD_FILTER_REGISTER").unwrap();
	feature("libavfilter/avfilter.h", Some("avfilter"), "FF_API_OLD_GRAPH_PARSE").unwrap();
	feature("libavfilter/avfilter.h", Some("avfilter"), "FF_API_NOCONST_GET_NAME").unwrap();

	feature("libavresample/avresample.h", Some("avresample"), "FF_API_RESAMPLE_CLOSE_OPEN").unwrap();

	feature("libswscale/swscale.h", Some("swscale"), "FF_API_SWS_CPU_CAPS").unwrap();
	feature("libswscale/swscale.h", Some("swscale"), "FF_API_ARCH_BFIN").unwrap();
}
