use std::{collections::{BTreeMap,VecDeque}, hash};
use std::fs;
use std::fs::File;
use std::io::{Read, Result};

const SPECIAL_CHAR: char = '\0';

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


type Link = Option<Box<Node>>;

/***************
NODE begin
 */
#[derive(Debug,Clone)]
struct Node{
    data: char,
    freq: usize,
    l: Link,
    r: Link,
    code:String,
}
impl PartialEq for Node{
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}
impl PartialOrd for Node{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.freq.partial_cmp(&other.freq)
    }
}
impl Eq for Node{

}
impl Ord for Node{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.freq.cmp(&other.freq)
    }
}
impl Node{
    fn new(data: char, freq: usize)->Node{
        Node {data:data,freq:freq,l:None,r:None,code:"".to_string()}
    }
}

/***************
NODE END
 */


fn build_huff_tree(nodes : &mut Vec<Node>) -> Node{
    // alg:
    // 1) Order the nodes based on freq
    // 2) Merge the two nodes with smallest freqs
    // 3) repeat until one node left
    while(nodes.len() > 1){
        let new_freq = nodes[0].freq + nodes[1].freq;
        let left_link = Some(Box::new(Node{
            data:nodes[0].data,freq:nodes[0].freq,
            l:nodes[0].l.clone(),r:nodes[0].r.clone(),code:"".to_string()
        }));
        let right_link = Some(Box::new(Node{
            data:nodes[1].data,freq:nodes[1].freq,
            l:nodes[1].l.clone(),r:nodes[1].r.clone(),code:"".to_string()
        }));
        // $ is special character
        let new_node = Node{data:SPECIAL_CHAR,freq:new_freq,
            l:left_link,r:right_link,code:"".to_string()
        };

        // remove first two elements of vector
        nodes.drain(0..2);

        // push new node in 
        nodes.push(new_node);
        // sort the tree
        nodes.sort();
    }

    nodes[0].clone()
}

// this preorder search should be iterative
fn mark_tree(root: &mut Option<Box<Node>>,marker:&mut String){
    match root{
        Some(inside) => {
            marker.push('0');
            mark_tree(&mut inside.l,marker);
            inside.code = marker.to_string();
            marker.push('1');
            mark_tree(&mut inside.r,marker);
            // need to pop
            marker.pop();
        }
        None => {
            // reached leaf     
            marker.pop();
        }
    }
}
fn get_hash_of_tree(root: Option<Box<Node>>) -> BTreeMap<char,String>{
    let mut ret_hash: BTreeMap<char,String> = BTreeMap::new();

    let mut stack: VecDeque<Box<Node>> = VecDeque::new();
    let mut current = root;

    loop {
        // Traverse to the leftmost node while pushing nodes onto the stack
        while let Some(node) = current {
            stack.push_back(node.clone());
            current = node.l;
        }

        // If the stack is empty, traversal is complete
        if stack.is_empty() {
            break;
        }

        // Process the current node (top of the stack)
        let node = stack.pop_back().unwrap();

        if node.data != SPECIAL_CHAR{
            ret_hash.insert(node.data,node.code);
        }

        // Move to the right subtree
        current = node.r;
    }
    ret_hash 
}

fn encode_file(msg: &String,map: &BTreeMap<char,String>) -> String{

    let encoded_str = convert_to_code_str(msg,&map);

    encoded_str
}
fn get_tree_root(msg: &String) -> Option<Box<Node>>{
    let hm = get_hash_char_freq(msg.clone());
    let mut nodes = get_nodes(hm);
    // sort in desc order
    nodes.sort();
    let mut tree_head = build_huff_tree(&mut nodes);
    let mut tree_head_ref = Some(Box::new(tree_head));
    mark_tree(&mut tree_head_ref,&mut "".to_string());

    tree_head_ref
}

fn main() {
    let file_name = "my_txt.txt";
    // this should be actually checking for error
    let str_content =read_file_to_string(file_name).unwrap();
    let file_size_bytes = get_file_size_bytes(file_name).unwrap() as f64;
    println!("file size prior: {}",file_size_bytes);

    // create the hash 
    let tree_root = get_tree_root(&str_content);
    let hash_code= get_hash_of_tree(tree_root);

    // encode the file
    let str_encoded = encode_file(&str_content,&hash_code);
    let new_byte_size:f64 = (str_encoded.len()/8) as f64;
    println!("New file size: {}",new_byte_size);
    println!("Percentage of original file size: {}",(new_byte_size/file_size_bytes)*100.0)

    // println!("{}",encoded_str);

    // decode the string
    // println!("{}",decode_encoded_str(str_encoded,&hash_code));
}

fn decode_encoded_str(encoded_msg: String, map: &BTreeMap<char,String>) -> String{
    // decode string
    let mut ret = String::new();
    let mut msg_copy = encoded_msg.clone();

    for _ in 0..encoded_msg.len(){
        for (key,value) in map.iter(){
            if value.len() <= msg_copy.len() && msg_copy.starts_with(value) {
                // check if this key matches the current substring
                ret.push(*key);
                // remove len chars
                msg_copy.drain(..value.len());
            }
        }
    }

    ret
}


fn convert_to_code_str(original_msg: &String,map: &BTreeMap<char,String>) -> String{
    // converts original message to encoded one
    let mut ret = String::new();

    for c in original_msg.chars(){
        ret.push_str(map.get(&c).unwrap());
    }

    ret
}

fn get_hash_char_freq(msg:String) -> BTreeMap<char,usize> {
    /*
    Return a hashmap of characters and their respective frequencies.
    ie:
    {
        'a': 2,
        'b':35,
        ...
    }
    */
    let mut ret_hash: BTreeMap<char,usize> = BTreeMap::new();
    
    for (_i,c) in msg.chars().enumerate(){
        if ret_hash.contains_key(&c){
            // if contains, just add to the freq
            ret_hash.insert(c,1+ret_hash[&c]);
        }else{
            // first entry
            ret_hash.insert(c,1);
        }
    }

    ret_hash
}

fn get_nodes(btm: BTreeMap<char,usize>) -> Vec<Node>{
    let mut vec : Vec<Node> = Vec::new();

    for (c,f) in btm.iter(){
        let node : Node = Node::new(*c,*f);
        vec.push(node);
    }

    vec.sort();

    vec
}