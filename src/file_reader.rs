use std::io::{Read, Result,Write, BufReader, BufRead};
use std::fs::File;
use std::fs;

use super::my_tree_map::MyTreeMap;

/*
Read a BTreeMap from the key file.
 */
pub fn get_hash_from_txt(file_name: &str) -> Result<MyTreeMap> {
    // Create a BTreeMap to store the data
    let mut my_tree_map= MyTreeMap::new_default();

    if let Ok(file) = File::open(file_name) {
        let reader = BufReader::new(file);

        for (_line_number, line) in reader.lines().enumerate() {
            if let Ok(line_text) = line {

                // Check if the line has the expected format
                if let (Some(first_char), Some(rest)) = (line_text.chars().next(),line_text.get(1..)) {
                    // Trim white space from the "rest" string
                    let trimmed_rest = rest.trim();
                    // Insert the data into the BTreeMap

                    my_tree_map.data_map.insert(first_char, trimmed_rest.to_string());
                }
            }
        }
    }

    Ok(my_tree_map)
}

pub fn read_file_to_string(filename: &str) -> Result<String> {
    // Open the file in read-only mode
    let mut file = File::open(filename)?;

    // Read the entire content of the file into a String
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}


pub fn get_file_size_bytes(filename: &str) -> Result<u64>{
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

pub fn write_str_to_file(filename: &str, data: &String) -> std::io::Result<()>{
    let mut file = File::create(filename)?;

    let _byte_buffer: u8 = 0;
    let _bit_position = data.len();
    
        // Create a buffer to store bits
    let mut bit_buffer: u8 = 0;
    let mut bit_position = 0; // Start from the least significant bit

    // read the bits
    for bit in data.chars() {
        if bit == '1'{
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

    let _ = file.flush();

    Ok(())
}

pub fn print_hash_to_file(map: &MyTreeMap, filename: &str) -> std::io::Result<()>{
    let mut file = File::create(filename)?;

    for (key,value) in map.data_map.clone(){
        file.write(&[key.to_owned() as u8])?;
        file.write(&[' ' as u8])?;
        for c in value.chars(){
            if c == '1'{
                let _ = file.write(&[c as u8]);
            }else{
                let _ = file.write(&[c as u8]);
            }
        }
        file.write(&['\n' as u8])?;
    }
    file.flush()?;

    Ok(())
}