use std::collections::BTreeMap;
use std::fs::File;
use std::fs;
use std::io::{Read, Result,Write, BufReader, BufRead};
use chibiTxt::huffman;
use std::env;


fn get_hash_from_txt(file_name: &str) -> Result<BTreeMap<char, String> > {
    // Create a BTreeMap to store the data
    let mut data_map: BTreeMap<char, String> = BTreeMap::new();

    if let Ok(file) = File::open(file_name) {
        let reader = BufReader::new(file);

        for (line_number, line) in reader.lines().enumerate() {
            if let Ok(line_text) = line {

                // Check if the line has the expected format
                if let (Some(first_char), Some(rest)) = (line_text.chars().next(),line_text.get(1..)) {
                    // Trim white space from the "rest" string
                    let trimmed_rest = rest.trim();
                    // Insert the data into the BTreeMap
                    data_map.insert(first_char, trimmed_rest.to_string());
                }
            }
        }
    }

    Ok(data_map)
}


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

    file.flush();

    Ok(())
}

fn print_hash_to_file(map: &BTreeMap<char,Vec<bool>>, filename: &str) -> std::io::Result<()>{
    let mut file = File::create(filename)?;

    for (key,value) in map{
        file.write(&[key.to_owned() as u8])?;
        file.write(&[' ' as u8])?;
        for bool_val in value{
            if *bool_val{
                file.write(&['1' as u8])?;
            }else{
                file.write(&['0' as u8])?;
            }
        }
        file.write(&['\n' as u8])?;
    }
    file.flush()?;

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("Usage: {} <-e/-d> <input> <output>", args[0]);
        return;
    }

    let flag = &args[1];

    match flag.as_str() {
        "-e" => {

            let input_file_name= &args[2];
            let output_file_name= &args[3];
            // Handle the case for the "-e" argument

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

            write_str_to_file(&output_file_name, &my_vec);
            let after_file_size = get_file_size_bytes(&output_file_name).unwrap() as f64;

            let percentage = (prior_file_size / after_file_size) * 100.0;

            println!("File compression percentage: {percentage}");
            println!("Old file size: {prior_file_size} (in bytes)");
            println!("After file size: {after_file_size} (in bytes)");

            print_hash_to_file(&hash_code,"out.txt");
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
                    file_contents.push_str(&bit.to_string());
                }
            }

            let map = get_hash_from_txt(&input_dict_name).unwrap();
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
