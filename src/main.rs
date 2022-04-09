use std::env::args;
use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::process::exit;

include!("bindings.rs");

fn main() {
	if args().len() != 3 {
		println!("Usage: knusperli <input.jpg> <output.png>");
		exit(1);
	}

	// Shamelessly stolen from: https://stackoverflow.com/questions/34379641

	// create a vector of zero terminated strings
	let args = args()
		.map(|arg| CString::new(arg).unwrap())
		.collect::<Vec<CString>>();
	// convert the strings to raw pointers
	let c_args = args
		.iter()
		.map(|arg| arg.as_ptr())
		.collect::<Vec<*const c_char>>();

	unsafe {
		// pass the pointer of the vector's internal buffer to a C function
		exit(main_cc(c_args.len() as c_int, c_args.as_ptr()) as i32);
	}
}
