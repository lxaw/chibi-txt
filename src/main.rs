use chibiTxt::huffman;
use chibiTxt::file_reader;
use std::env;
use std::fs::File;
use std::io::{Read,Write};
use std::process::exit;

fn print_usage(){
    println!("usage: cargo run -- -e [input_file.txt] [output_bin_file_name.bin] [key_file_name.txt]");
    println!("or");
    println!("usage: cargo run -- -d [input_file.bin] [key_file_name.txt] [output_decoded_file_name.txt]");
}
fn check_arg_count(args: &Vec<String>) -> bool{
    args.len() == 5
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if !check_arg_count(&args){
        print_usage();
        exit(-1);
    }

    let flag = &args[1];

    match flag.as_str() {
        "-e" => {
            let input_file_name= &args[2];
            let output_file_name= &args[3];
            // Handle the case for the "-e" argument

            // this should be actually checking for error
            let str_content = file_reader::read_file_to_string(input_file_name).unwrap();
            // let file_size_bytes = get_file_size_bytes(input_file_name).unwrap() as f64;
            // println!("file size prior: {}",file_size_bytes);

            // create the hash 
            let tree_root = huffman::get_tree_root(&str_content);
            let hash_code= huffman::get_hash_of_tree(tree_root);

            // encode the file
            let str_encoded = huffman::encode_file(&str_content,&hash_code);

            let my_vec =str_encoded.to_vec();

            // file size before
            // should thread this
            let prior_file_size = file_reader::get_file_size_bytes(&input_file_name).unwrap() as f64;

            let _ = file_reader::write_str_to_file(&output_file_name, &my_vec);
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

            // input is a binary file
            let mut file_contents = String::new();

            let mut file = File::open(input_bin_name).unwrap();
            // Create a buffer to read data into
            let mut buffer = [0; 1]; // Read one byte (8 bits) at a time

            // Read and process the file
            while let Ok(bytes_read) = file.read(&mut buffer) {
                if bytes_read == 0 {
                    break; // End of file
                }

                // Process each bit in the byte
                for i in 0..8 {
                    let bit = (buffer[0] & (1 << i)) >> i;
                    println!("{}",bit);
                    file_contents.push_str(&bit.to_string());
                }
            }

            let map = file_reader::get_hash_from_txt(&input_dict_name).unwrap();
            let file_decrypted = huffman::decode_encoded_str(&file_contents, &map);
            let mut file = File::create(output_file_name).unwrap();
            write!(file,"{}",file_decrypted);
        }
        _ => {
            println!("Unknown argument: {}", flag);
            println!("Usage: {} <-e/-d> <input> <output>", args[0]);
        }
    }
}