use std::collections::{VecDeque, BTreeMap};
use crate::my_tree_map::MyTreeMap;

use super::node::Node;

const SPECIAL_CHAR: char = '\0';
const NEW_LINE: char = '%';

fn build_huff_tree(nodes : &mut Vec<Node>) -> Node{
    // alg:
    // 1) Order the nodes based on freq
    // 2) Merge the two nodes with smallest freqs
    // 3) repeat until one node left
    while nodes.len() > 1{
        let new_freq = nodes[0].freq + nodes[1].freq;
        let left_link_node = Node::new_param(nodes[0].data,nodes[0].freq,nodes[0].l.clone(),nodes[0].r.clone(),String::new());
        let left_link = Some(Box::new(left_link_node));
        
        let right_link_node = Node::new_param(nodes[1].data,nodes[1].freq,nodes[1].l.clone(),nodes[1].r.clone(),String::new());
        let right_link = Some(Box::new(right_link_node));

        // this is the top node that is used to be sorted (see the wikipedia image of the tree)
        // the new node adds the two frequencies
        let new_node = Node::new_param(SPECIAL_CHAR,new_freq, left_link, right_link,String::new());

        // remove first two elements of vector
        // (ie, the left and right nodes)
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
            // since we always pad with 0's when there not an even byte amount of binary,
            // we want to ensure codes always start with 1 to avoid mistranslations
            inside.code.insert(0,'1');
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

// create a hash map (ie, BTreeMap) of the Huffman tree
pub fn get_hash_of_tree(root: Option<Box<Node>>) -> MyTreeMap{
    let mut ret_hash: MyTreeMap =MyTreeMap::new_default();

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
            ret_hash.data_map.insert(node.data,node.code);
        }

        // Move to the right subtree
        current = node.r;
    }
    ret_hash 
}

pub fn encode_file(msg: &String,map: &MyTreeMap) -> String{

    let encoded_str = convert_to_code_str(msg,&map);

    encoded_str
}
pub fn get_tree_root(msg: &String) -> Option<Box<Node>>{
    let hm = get_hash_char_freq(msg.clone());
    let mut nodes = get_nodes(hm);
    // sort in desc order
    nodes.sort();
    let tree_head = build_huff_tree(&mut nodes);
    let mut tree_head_ref = Some(Box::new(tree_head));
    mark_tree(&mut tree_head_ref,&mut String::new());
    // mark_tree_iterative(&mut tree_head_ref,&mut Vec::new());

    tree_head_ref
}

pub fn decode_encoded_str(encoded_msg: &mut String, map: &MyTreeMap) -> String {
    let mut ret = String::new();
    let mut index = 0;

    while index < encoded_msg.len() {
        let mut found = false;

        for (key, value) in map.data_map.iter() {
            if value.len() <= encoded_msg.len() && encoded_msg[index..].starts_with(value) {
                if *key == NEW_LINE {
                    ret.push('\n');
                } else {
                    ret.push(*key);
                }

                index += value.len();
                found = true;
                break;
            }
        }

        if !found {
            // If no match is found, just copy the character
            ret.push(encoded_msg.chars().nth(index).unwrap());
            index += 1;
        }
    }

    ret
}



fn convert_to_code_str(original_msg: &String,map: &MyTreeMap) -> String{
    // converts original message to encoded one
    let mut ret = String::new();

    for c in original_msg.chars(){
        if c == '\n'{
            ret.extend(map.data_map.get(&NEW_LINE).unwrap().chars());
        }else{
            ret.extend(map.data_map.get(&c).unwrap().chars());
        }
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
    // very important
    // FIX THIS SO THAT WE CAN WORK WITH SPACES AND NEW LINES AND TABS
    let mut ret_hash: BTreeMap<char,usize> = BTreeMap::new();
    
    for (_i,c) in msg.chars().enumerate(){
        if c == '\n'{
            if ret_hash.contains_key(&NEW_LINE){
                // if contains, just add to the freq
                ret_hash.insert(NEW_LINE,1+ret_hash[&NEW_LINE]);
            }else{
                // first entry
                ret_hash.insert(NEW_LINE,1);
            }
        }else{
            if ret_hash.contains_key(&c){
                // if contains, just add to the freq
                ret_hash.insert(c,1+ret_hash[&c]);
            }else{
                // first entry
                ret_hash.insert(c,1);
            }
        }
    }

    ret_hash
}

fn get_nodes(btm: BTreeMap<char,usize>) -> Vec<Node>{
    let mut vec : Vec<Node> = Vec::new();

    for (c,f) in btm.iter(){
        let node : Node = Node::new_default(*c,*f);
        vec.push(node);
    }

    vec.sort();

    vec
}