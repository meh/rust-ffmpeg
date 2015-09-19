use std::env;
use std::fs;
use std::io;
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

fn main() {
	if env::var("CARGO_FEATURE_BUILD").is_err() {
		return;
	}

	println!("cargo:rustc-link-search=native={}", search().join("lib").to_string_lossy());

	if fs::metadata(&search().join("lib").join("libavutil.a")).is_ok() {
		return;
	}

	fs::create_dir_all(&output()).ok().expect("failed to create build directory");
	fetch().unwrap();
	extract().unwrap();
	build().unwrap();
}
