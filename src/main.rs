use std::{collections::{BTreeMap,VecDeque}, hash};

// https://www.geeksforgeeks.org/huffman-coding-greedy-algo-3/

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
        let new_node = Node{data:'$',freq:new_freq,
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

        if node.data != '$'{
            ret_hash.insert(node.data,node.code);
        }

        // Move to the right subtree
        current = node.r;
    }
    ret_hash 
}

fn main() {
    let msg = "This is my secret message. Please do not let anyone see!".to_string();

    let hm = get_hash_char_freq(msg.clone());

    let mut nodes = get_nodes(hm);
    // sort in desc order
    nodes.sort();

    let mut tree_head = build_huff_tree(&mut nodes);
    let mut tree_head_ref = Some(Box::new(tree_head));
    mark_tree(&mut tree_head_ref,&mut "".to_string());
    let hash_code= get_hash_of_tree(tree_head_ref);

    let encoded_str = convert_to_code_str(msg,&hash_code);
    // println!("{}",encoded_str);

    // decode the string
    println!("{}",decode_encoded_str(encoded_str,&hash_code));
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


fn convert_to_code_str(original_msg: String,map: &BTreeMap<char,String>) -> String{
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