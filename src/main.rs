use std::fs::File;
use std::fs;
use std::io::{Read, Result,Write};
use chibiTxt::huffman;


// for command line args
use clap::{Arg,Command};


fn read_file_to_string(filename: &str) -> Result<String> {
    // Open the file in read-only mode
    let mut file = File::open(filename)?;

    // Read the entire content of the file into a String
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}

fn get_file_size_bytes(filename: &str) -> Result<u64>{
    match fs::metadata(filename) {
        Ok(metadata) => {
            let file_size = metadata.len();
            return Ok(file_size);
        }
        Err(err) => {
            return Err(err);
        }
    }
}

fn write_str_to_file(filename: &str, data: &Vec<bool>) -> std::io::Result<()>{
    let mut file = File::create(filename)?;

    let _byte_buffer: u8 = 0;
    let _bit_position = data.len();
    
        // Create a buffer to store bits
    let mut bit_buffer: u8 = 0;
    let mut bit_position = 0; // Start from the least significant bit

    for bit in data.iter() {
        if *bit {
            bit_buffer |= 1 << bit_position;
        }

        bit_position += 1;

        // If we've filled a byte, write it to the file
        if bit_position == 8 {
            file.write(&[bit_buffer])?;
            bit_buffer = 0;
            bit_position = 0;
        }
    }

    // If there are any remaining bits in the buffer, write them and pad with '0's
    if bit_position != 0 {
        file.write(&[bit_buffer])?;
    }

    Ok(())
}

fn main() {

    // clap
    let matches = Command::new("chibiTxt")
        .version("0.true")
        .author("Lex W. <https://github.com/lxaw/chibiTxt>")
        .about("A simple file encoder.")
        .arg(Arg::new("input")
            .value_name("INPUT_FILE")
            .help("Sets the input text file name.")
            .required(true)
            .index(1)
        )
        .arg(Arg::new("output")
            .value_name("OUTPUT_FILE")
            .help("Sets the output binary file name.")
            .required(true)
            .index(2)
        )
        .get_matches();

    let input_file_name = matches.get_one::<String>("input").unwrap();
    let output_file_name = matches.get_one::<String>("output").unwrap();

    // this should be actually checking for error
    let str_content =read_file_to_string(input_file_name).unwrap();
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
    let prior_file_size = get_file_size_bytes(&input_file_name).unwrap() as f64;

    // println!("{}",huffman::decode_encoded_str(str_encoded,&hash_code));

    write_str_to_file(&output_file_name, &my_vec);
    let after_file_size = get_file_size_bytes(&output_file_name).unwrap() as f64;

    let percentage = (prior_file_size / after_file_size) * 100.0;

    println!("File compression percentage: {percentage}");
    println!("Old file size: {prior_file_size} (in bytes)");
    println!("After file size: {after_file_size} (in bytes)");

}
