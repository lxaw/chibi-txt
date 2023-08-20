use std::fs::File;
use std::fs;
use std::io::{Read, Result};
use chibiTxt::huffman;


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

fn main() {
    let file_name = "test.txt";
    // this should be actually checking for error
    let str_content =read_file_to_string(file_name).unwrap();
    let file_size_bytes = get_file_size_bytes(file_name).unwrap() as f64;
    println!("file size prior: {}",file_size_bytes);

    // create the hash 
    let tree_root = huffman::get_tree_root(&str_content);
    let hash_code= huffman::get_hash_of_tree(tree_root);

    // encode the file
    let str_encoded = huffman::encode_file(&str_content,&hash_code);
    let new_byte_size:f64 = (str_encoded.len()/8) as f64;
    println!("New file size: {}",new_byte_size);
    println!("Percentage of original file size: {}",(new_byte_size/file_size_bytes)*100.0);

    // println!("{}",encoded_str);

    // decode the string
    println!("{}",huffman::decode_encoded_str(str_encoded,&hash_code));
}
