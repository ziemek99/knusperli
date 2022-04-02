fn main() {
	std::process::Command::new("git")
		.args([
			"submodule",
			"update",
			"--init",
			"--depth",
			"1",
			"--recommend-shallow",
		])
		.status()
		.expect("Failed to fetch git submodules!");

	cc::Build::new()
		.cpp(true)
		.file("src/dct_double.cc")
		.file("src/decode.cc")
		.file("src/gamma_correct.cc")
		.file("src/idct.cc")
		.file("src/jpeg_data_decoder.cc")
		.file("src/jpeg_data_reader.cc")
		.file("src/jpeg_data.cc")
		.file("src/jpeg_huffman_decode.cc")
		.file("src/output_image.cc")
		.file("src/preprocess_downsample.cc")
		.file("src/quantize.cc")
		.file("src/lodepng/lodepng.cpp")
		.include("src/lodepng")
		.compile("knusperli");
}
