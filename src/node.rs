/*
Node

 The nodes are the nodes of the Huffman tree.
 See: https://en.wikipedia.org/wiki/Huffman_coding
*/
type Link = Option<Box<Node>>;

#[derive(Debug,Clone)]
pub struct Node{
    // character of the text
    pub data: char,
    // frequency of character within text
    pub freq: usize,
    // link to left child
    pub l: Link,
    // link to right child
    pub r: Link,
    // code for the character
    // ie, 'A' could be represented as 0010
    pub code:String,
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
    pub fn new_default(data: char, freq: usize)->Node{
        Node {data,freq,l:None,r:None,code:String::new()}
    }
    pub fn new_param(data: char, freq: usize,l:Link,r:Link,code:String)->Node{
        Node {data,freq,l,r,code}
    }
}

/***************
NODE END
 */
