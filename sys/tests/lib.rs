extern crate libc;
extern crate ffmpeg_sys;

use std::fs::{create_dir, File, symlink_metadata};
use std::io::{Write, BufRead, BufReader};
use std::mem;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str;

use ffmpeg_sys::AVCodecContext;

fn output() -> PathBuf {
	let mut ret = std::env::current_dir().unwrap();
	ret.push(&Path::new("tmp"));
	if symlink_metadata(&ret).is_err() {
		create_dir(&ret).expect("Failed to create temporary output dir");
	}
	ret
}

// Checks if byte offsets in C vs. Rust match
#[test]
fn check_struct_byte_offsets() {
	let out_dir = output();
	let executable = out_dir.join(if cfg!(windows) { "ffmpeg-structs.exe" } else { "ffmpeg-structs" });

    let curr_dir = std::env::current_dir().unwrap();
	let f = File::open(curr_dir.join("tmp/.build"))
		.expect("Filed to open .build");
	let f = BufReader::new(f);
	let args: Vec<String> = f.lines().nth(0).unwrap().unwrap()
		.split(' ').map(String::from).collect();

	println!("Compiling ffmpeg-structs.c");

	if !Command::new(&args[0])
		.args(&args[1..])
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
	let mut matching = vec!();
	let mut errors = vec!();

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
				errors.push(format!("Struct field position as specified in Rust code ({}) is different from C ({})", expected, actual));
			}
			else {
				matching.push(format!("  - {}", expected));
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

	// p!(AVCodecContext, b_frame_strategy); // TODO: make conditional somehow with cfg!(feature="ff_api_private_opt")

	println!("These struct fields have matching position/size in Rust and C code:\n{}", matching.join("\n"));

	if !errors.is_empty() {
		panic!(errors.join("\n"));
	}
}
