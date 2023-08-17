use core::fmt;
use std::{collections::BTreeMap};

// https://www.geeksforgeeks.org/huffman-coding-greedy-algo-3/

#[derive(Clone, Copy)]
struct HeapNode {
    data: char,
    freq: usize,
}

impl fmt::Debug for HeapNode{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HeapNode [{}:{}]", self.data, self.freq)
    }
}

impl PartialEq for HeapNode{
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}
impl PartialOrd for HeapNode{
    fn ge(&self, other: &Self) -> bool {
        self.freq >= other.freq
    }
    fn gt(&self, other: &Self) -> bool {
        self.freq > other.freq
    }
    fn le(&self, other: &Self) -> bool {
        self.freq <= other.freq 
    }
    fn lt(&self, other: &Self) -> bool {
        self.freq < other.freq 
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.data.partial_cmp(&other.data) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.freq.partial_cmp(&other.freq)
    }

}

struct Heap{
    nodes : Vec<HeapNode>    
}
impl fmt::Debug for Heap{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{:?}",self.nodes)
    }
}
impl Heap{
    // build the heap
    fn build_heap(&mut self){
        // index of last non-leaf node
        let start_index = self.nodes.len()/2;

        for i in (0..start_index).rev(){
            self.heapify(i);
        }
    }
    // heapify
    fn heapify(&mut self,i:usize){
        let mut largest = i;
        let l = Heap::left_index(i);
        let r = Heap::right_index(i);

        // if left child larger than root
        if l < self.nodes.len()-1 && self.nodes[l] > self.nodes[largest]{
            largest = l;
        }
        // if right child larger than largest so far
        if r < self.nodes.len()-1 && self.nodes[r] > self.nodes[largest]{
            largest = r;
        }
        // if largest is not root
        if largest != i{
            self.nodes.swap(i,largest);
            self.heapify(largest);
        }
    }
    fn is_leaf(&self, pos:usize) -> bool{
        pos*2 > self.nodes.len()-1
    }
    fn min_heapify(&mut self, pos:usize){
        if !self.is_leaf(pos){
            if self.nodes[pos] > self.nodes[Heap::left_index(pos)] ||
                self.nodes[pos] > self.nodes[Heap::right_index(pos)]
                {
                    if self.nodes[Heap::left_index(pos)] < self.nodes[Heap::right_index(pos)]{
                        self.nodes.swap(pos, Heap::left_index(pos));
                        self.min_heapify(Heap::left_index(pos));
                    }
                    else{
                        self.nodes.swap(pos,Heap::right_index(pos));
                        self.min_heapify(Heap::right_index(pos));
                    }
                }
        }
    }
    fn min_heap(&mut self){
        for i in (0..(self.nodes.len())/2).rev(){
            self.min_heapify(i);
        }
    }
    fn left_index(index: usize) -> usize{
        2*index
    }
    fn right_index(index: usize) -> usize{
        (2*index) + 1
    }
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

fn get_heap_nodes(btm: BTreeMap<char,usize>) -> Vec<HeapNode>{
    let mut vec : Vec<HeapNode> = Vec::new();

    for (c,f) in btm.iter(){
        let node : HeapNode = HeapNode { data: (*c), freq: (*f) };
        vec.push(node);
    }
    vec
}

fn main() {
    let msg = "Hello world this is my message.".to_string();

    let hm = get_hash_char_freq(msg);
    let _vec = get_heap_nodes(hm);

    let test_arr = [
        HeapNode{data:'c',freq:20},
        HeapNode{data:'c',freq:25},
        HeapNode{data:'c',freq:10},
        HeapNode{data:'c',freq:99},
        HeapNode{data:'c',freq:60},
        HeapNode{data:'c',freq:70},
        HeapNode{data:'c',freq:82},
        HeapNode{data:'c',freq:30},
        HeapNode{data:'c',freq:50},
        HeapNode{data:'c',freq:90},
        HeapNode{data:'c',freq:58},
        HeapNode{data:'c',freq:85},
        HeapNode{data:'c',freq:71},
        // need extra space here! TODO
    ];
    // 10, 20, 25, 60, 30, 58, 71, 99, 70, 82, 50, 90, 85

    let nodes : Vec<HeapNode> = Vec::from(test_arr);

    let mut heap : Heap = Heap { nodes: (nodes) };
    heap.build_heap();
    heap.min_heap();

    println!("{:?}",heap);
}