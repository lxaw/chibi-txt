use std::{collections::{BTreeMap,VecDeque}};
use super::node::{Node};

const SPECIAL_CHAR: char = '\0';
const NEW_LINE: char = '%';

fn build_huff_tree(nodes : &mut Vec<Node>) -> Node{
    // alg:
    // 1) Order the nodes based on freq
    // 2) Merge the two nodes with smallest freqs
    // 3) repeat until one node left
    while(nodes.len() > 1){
        let new_freq = nodes[0].freq + nodes[1].freq;
        let left_link = Some(Box::new(Node{
            data:nodes[0].data,freq:nodes[0].freq,
            l:nodes[0].l.clone(),r:nodes[0].r.clone(),code:Vec::new()
        }));
        let right_link = Some(Box::new(Node{
            data:nodes[1].data,freq:nodes[1].freq,
            l:nodes[1].l.clone(),r:nodes[1].r.clone(),code:Vec::new()
        }));
        // $ is special character
        let new_node = Node{data:SPECIAL_CHAR,freq:new_freq,
            l:left_link,r:right_link,code:Vec::new()
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
fn mark_tree(root: &mut Option<Box<Node>>,marker:&mut Vec<bool>){
    match root{
        Some(inside) => {
            marker.push(false);
            mark_tree(&mut inside.l,marker);
            inside.code = marker.to_vec();
            marker.push(true);
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
pub fn get_hash_of_tree(root: Option<Box<Node>>) -> BTreeMap<char,Vec<bool>>{
    let mut ret_hash: BTreeMap<char,Vec<bool>> = BTreeMap::new();

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

pub fn encode_file(msg: &String,map: &BTreeMap<char,Vec<bool>>) -> Vec<bool>{

    let encoded_str = convert_to_code_str(msg,&map);

    encoded_str
}
pub fn get_tree_root(msg: &String) -> Option<Box<Node>>{
    let hm = get_hash_char_freq(msg.clone());
    let mut nodes = get_nodes(hm);
    // sort in desc order
    nodes.sort();
    let mut tree_head = build_huff_tree(&mut nodes);
    let mut tree_head_ref = Some(Box::new(tree_head));
    mark_tree(&mut tree_head_ref,&mut Vec::new());

    tree_head_ref
}

pub fn decode_encoded_str(encoded_msg: &String, map: &BTreeMap<char,String>) -> String{
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


fn convert_to_code_str(original_msg: &String,map: &BTreeMap<char,Vec<bool>>) ->Vec<bool>{
    // converts original message to encoded one
    let mut ret = Vec::new();

    for c in original_msg.chars(){
        if c == '\n'{
            ret.extend(map.get(&NEW_LINE).unwrap());
        }else{
            ret.extend(map.get(&c).unwrap());
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
        let node : Node = Node::new(*c,*f);
        vec.push(node);
    }

    vec.sort();

    vec
}