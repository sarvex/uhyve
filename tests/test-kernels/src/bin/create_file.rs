use std::{fs::File, io::prelude::*};

#[cfg(target_os = "hermit")]
use hermit_sys as _;

fn main() {
	let mut file = File::create("foo.txt").unwrap();
	file.write_all(b"Hello, world!").unwrap();
}
