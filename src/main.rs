use std::env::args;
use std::fs::File;
use std::io::Write;
use std::process::exit;

//include!("bindings.rs");

fn main() -> Result<(), std::io::Error> {
	if args().len() != 3 {
		println!("Usage: knusperli <input.jpg> <output.png>");
		exit(1);
	}

	let args: Vec<String> = args().collect();

	let _in_file = File::open(&args[1])?;
	let mut out_file = File::create(&args[2])?;
	out_file.write_all(b"Test.\nThe quick brown fox jumps over a lazy dog.")?;
	Ok(())
}
