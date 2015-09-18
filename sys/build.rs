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

	// the binary using ffmpeg-sys must comply with GPL
	if env::var("CARGO_FEATURE_BUILD_GPL").is_ok() {
		configure.arg("--enable-gpl");
	}
	
	// the binary using ffmpeg-sys must comply with (L)GPLv3
	if env::var("CARGO_FEATURE_BUILD_VERSION3").is_ok() {
		configure.arg("--enable-version3");
	}

	// the binary using ffmpeg-sys cannot be redistributed
	if env::var("CARGO_FEATURE_BUILD_NONFREE").is_ok() {
		configure.arg("--enable-nonfree");
	}

	// do not build programs since we don't need them
	configure.arg("--disable-programs");

	// disable/enable building libraries based on features
	if env::var("CARGO_FEATURE_AVCODEC").is_ok() {
		configure.arg("--enable-avcodec");
	}
	else {
		configure.arg("--disable-avcodec");
	}

	if env::var("CARGO_FEATURE_AVDEVICE").is_ok() {
		configure.arg("--enable-avdevice");
	}
	else {
		configure.arg("--disable-avdevice");
	}

	if env::var("CARGO_FEATURE_AVFILTER").is_ok() {
		configure.arg("--enable-avfilter");
	}
	else {
		configure.arg("--disable-avfilter");
	}

	if env::var("CARGO_FEATURE_AVFORMAT").is_ok() {
		configure.arg("--enable-avformat");
	}
	else {
		configure.arg("--disable-avformat");
	}

	if env::var("CARGO_FEATURE_AVRESAMPLE").is_ok() {
		configure.arg("--enable-avresample");
	}
	else {
		configure.arg("--disable-avresample");
	}

	if env::var("CARGO_FEATURE_POSTPROC").is_ok() {
		if env::var("CARGO_FEATURE_BUILD_GPL").is_err() {
			return Err(io::Error::new(io::ErrorKind::Other, "postproc is GPL"));
		}

		configure.arg("--enable-postproc");
	}
	else {
		configure.arg("--disable-postproc");
	}

	if env::var("CARGO_FEATURE_SWRESAMPLE").is_ok() {
		configure.arg("--enable-swresample");
	}
	else {
		configure.arg("--disable-swresample");
	}

	if env::var("CARGO_FEATURE_SWSCALE").is_ok() {
		configure.arg("--enable-swscale");
	}
	else {
		configure.arg("--disable-swscale");
	}

	// enable external codecs
	configure.arg("--enable-avisynth");
	configure.arg("--enable-avresample");
	configure.arg("--enable-fontconfig");
	configure.arg("--enable-gnutls");
	configure.arg("--enable-ladspa");
	configure.arg("--enable-libass");
	configure.arg("--enable-libbluray");
	configure.arg("--enable-libfreetype");
	configure.arg("--enable-libfribidi");
	configure.arg("--enable-libgsm");
	configure.arg("--enable-libmodplug");
	configure.arg("--enable-libmp3lame");
	configure.arg("--enable-libopenjpeg");
	configure.arg("--enable-libopus");
	configure.arg("--enable-libpulse");
	configure.arg("--enable-libschroedinger");
	configure.arg("--enable-libsoxr");
	configure.arg("--enable-libspeex");
	configure.arg("--enable-libssh");
	configure.arg("--enable-libtheora");
	configure.arg("--enable-libv4l2");
	configure.arg("--enable-libvorbis");
	configure.arg("--enable-libvpx");
	configure.arg("--enable-libwebp");

	if env::var("CARGO_FEATURE_BUILD_GPL").is_ok() {
		configure.arg("--enable-libx264");
		configure.arg("--enable-libx265");
		configure.arg("--enable-libxvid");
	}

	if env::var("CARGO_FEATURE_BUILD_VERSION3").is_ok() {
		configure.arg("--enable-libopencore_amrnb");
		configure.arg("--enable-libopencore_amrwb");
	}

	if !try!(configure.status()).success() {
		return Err(io::Error::new(io::ErrorKind::Other, "configure failed"));
	}

	if !try!(Command::new("make").current_dir(&source()).status()).success() {
		return Err(io::Error::new(io::ErrorKind::Other, "make failed"));
	}

	if !try!(Command::new("make").current_dir(&source()).arg("install").status()).success() {
		return Err(io::Error::new(io::ErrorKind::Other, "install failed"));
	}

	Ok(())
}

fn main() {
	println!("cargo:rustc-link-search=native={}", search().join("lib").to_string_lossy());

	if fs::metadata(&search().join("lib").join("libavutil.a")).is_ok() {
		return;
	}

	fs::create_dir_all(&output()).ok().expect("failed to create build directory");
	fetch().unwrap();
	extract().unwrap();
	build().unwrap();
}
