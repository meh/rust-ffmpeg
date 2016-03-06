extern crate libc;
extern crate ffmpeg_sys;

use std::env;
use std::fs::{create_dir, File};
use std::io::Write;
use std::mem;
use std::path::PathBuf;
use std::process::Command;
use std::str;

use libc::c_void;

use ffmpeg_sys::AVCodecContext;

fn output() -> PathBuf {
	let ret = PathBuf::from("tmp");
	create_dir(&ret).ok();
	ret
}

// Checks if byte offsets in C vs. Rust match
#[test]
fn check_struct_byte_offsets() {
	let out_dir = output();
	let executable = out_dir.join(if cfg!(windows) { "ffmpeg-structs.exe" } else { "ffmpeg-structs" });
	let compiler =
		if cfg!(windows) || env::var("MSYSTEM").unwrap_or("".to_string()).starts_with("MINGW32") {
			"gcc"
		}
		else {
			"cc"
		};

	println!("Compiling ffmpeg-structs.c");

	if !Command::new(compiler)
		.arg("-o").arg(&executable)
		.arg("tests/data/ffmpeg-structs.c")
		.status()
		.expect("ffmpeg-structs compile failed")
		.success() {
		panic!("ffmpeg-structs compile failed");
	}

	println!("Running ffmpeg-structs ({:?})", executable);
	let stdout_raw = Command::new(&executable).current_dir(&out_dir).output().expect("ffmpeg-structs failed").stdout;

	println!("Writing ffmpeg-structs output file");
	File::create(out_dir.join("ffmpeg-structs.out")).expect("ffmpeg-structs.out 1").write_all(&stdout_raw[..]).expect("ffmpeg-structs.out 2");

	let stdout = str::from_utf8(stdout_raw.as_slice()).unwrap();

	// This should check same fields as ffmpeg-structs.c
	macro_rules! p {
		($the_struct:ident, $the_field:ident) => {{
			println!("Test field size for {}::{}", stringify!($the_struct), stringify!($the_field));
			let ptr : &$the_struct = unsafe { mem::uninitialized() };
			let expected = format!("[{}::{} @ {}-{}]", stringify!($the_struct), stringify!($the_field), (&ptr.$the_field) as *const _ as usize - ptr as *const _ as usize, mem::size_of_val(&ptr.$the_field));

			if stdout.find(&expected).is_none() {
				let actual_prefix = format!("[{}::{} @ ", stringify!($the_struct), stringify!($the_field));
				let mut actual = String::from("not found");
				for line in stdout.lines() {
					if line.find(&actual_prefix).is_some() {
						actual = line.to_string();
						break
					}
				}
				panic!("Struct field position as specified in Rust code ({}) is different from C ({})", expected, actual);
			}
		}};
	}

	p!(AVCodecContext, av_class);
	p!(AVCodecContext, codec_id);
	p!(AVCodecContext, bit_rate);
	p!(AVCodecContext, bit_rate_tolerance);
	p!(AVCodecContext, width);
	p!(AVCodecContext, height);
	p!(AVCodecContext, coded_width);
	p!(AVCodecContext, pix_fmt);
	p!(AVCodecContext, sample_rate);
	p!(AVCodecContext, channels);
	p!(AVCodecContext, sample_fmt);
	p!(AVCodecContext, frame_size);
	p!(AVCodecContext, frame_number);
	p!(AVCodecContext, block_align);
}
