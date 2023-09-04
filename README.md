# chibiTxt
A simple file compressor using Rust.

# :grey_question: What is it?
Compresses files by using [Huffman encoding](https://en.wikipedia.org/wiki/Huffman_coding).

# :file_folder: What file types?
Right now only txt. However, the code is simple enough to use with pretty much any file type, just requires a little bit of fine-tuning. The idea is to show how to write a simple encoding/decoding program.

# How to use:
cargo run [input_file.txt] [output_file.bin]

# Example:
`user@computer:~/ cargo run example_txt/anna_karenina.txt "out.bin"`

`File compression percentage: 182.88868810627798`

`Old file size: 2026200 (in bytes)`

`After file size:1107887 (in bytes)`

# Notes:
This can surely be optimized. It was one of my first projects in Rust, so I am sure I will look back in agony at how poorly it was done. However, that is a problem for future me.
