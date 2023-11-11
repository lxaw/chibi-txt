/*
Nov 11, Lex Whalen

chibi-txt

A small file compressor. Small is the name of the game!
*/

use chibiTxt::huffman;
use chibiTxt::file_reader;
use std::env;
use std::fs::File;
use std::io::{Read,Write};
use std::process::exit;

// number of arguments expected
const ARG_COUNT:usize = 5;

fn print_usage(){
    println!("usage: cargo run -- -e [input_file.txt] [output_bin_file_name.bin] [key_file_name.txt]");
    println!("or");
    println!("usage: cargo run -- -d [input_file.bin] [key_file_name.txt] [output_decoded_file_name.txt]");
}

fn check_arg_count(args: &Vec<String>) -> bool{
    args.len() == ARG_COUNT
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // ensure the user enters the correct number of arguments
    if !check_arg_count(&args){
        print_usage();
        exit(-1);
    }

    // get the first argument, which is the flag.
    // the flag must either be "-e" (encode) or "-d" (decode)
    let flag = &args[1];

    match flag.as_str() {
        "-e" => {
            let input_file_name= &args[2];
            let output_file_name= &args[3];
            // Handle the case for the "-e" argument

            // this should be actually checking for error
            let str_content = file_reader::read_file_to_string(input_file_name).unwrap();

            // create the hash 
            let tree_root = huffman::get_tree_root(&str_content);
            let hash_code= huffman::get_hash_of_tree(tree_root);

            // encode the file
            let str_encoded = huffman::encode_file(&str_content,&hash_code);


            // file size before
            // should thread this
            let prior_file_size = file_reader::get_file_size_bytes(&input_file_name).unwrap() as f64;

            let _ = file_reader::write_str_to_file(&output_file_name, &str_encoded);
            let after_file_size = file_reader::get_file_size_bytes(&output_file_name).unwrap() as f64;

            let percentage = (prior_file_size / after_file_size) * 100.0;

            println!("File compression percentage: {percentage}");
            println!("Old file size: {prior_file_size} (in bytes)");
            println!("After file size: {after_file_size} (in bytes)");

            let _ = file_reader::print_hash_to_file(&hash_code,"out.txt");
        }
        "-d" => {
            let input_bin_name= &args[2];
            let input_dict_name= &args[3];
            let output_file_name = &args[4];

            let mut file = File::open(input_bin_name).unwrap();

            let file_size_bits = (file.metadata().unwrap().len() * 8) as usize;

            // Create a buffer to read data into
            let mut buffer = [0; 1]; // Read one byte (8 bits) at a time

            let mut char_bit_vec: Vec<char>= vec![' ';file_size_bits];

            // Read and process the file
            let mut index = 0;

            while let Ok(bytes_read) = file.read(&mut buffer) {
                if bytes_read == 0 {
                    break; // End of file
                }

                // Process each bit in the byte
                for i in 0..8 {
                    let bit = (buffer[0] & (1 << i)) >> i;
                    match bit{
                        0=>char_bit_vec[index] = '0',
                        1=>char_bit_vec[index] = '1',
                        _=>panic!("Invalid u8 value")
                    }
                    index += 1;
                }
            }
            let mut str_bits = char_bit_vec.into_iter().collect::<String>();

            let map = file_reader::get_hash_from_txt(&input_dict_name).unwrap();
            let file_decrypted = huffman::decode_encoded_str(&mut str_bits, &map);
            let mut file = File::create(output_file_name).unwrap();
            write!(file,"{}",file_decrypted).unwrap();
        }
        _ => {
            println!("Unknown argument: {}", flag);
            println!("Usage: {} <-e/-d> <input> <output>", args[0]);
        }
    }
}