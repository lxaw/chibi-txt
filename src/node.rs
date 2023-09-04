type Link = Option<Box<Node>>;

/***************
NODE begin
 */
#[derive(Debug,Clone)]
pub struct Node{
    pub data: char,
    pub freq: usize,
    pub l: Link,
    pub r: Link,
    pub code:Vec<bool>,
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
    pub fn new(data: char, freq: usize)->Node{
        Node {data:data,freq:freq,l:None,r:None,code:Vec::new()}
    }
}

/***************
NODE END
 */
