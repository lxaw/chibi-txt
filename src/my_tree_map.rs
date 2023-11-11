use std::collections::BTreeMap;

#[derive(Clone)]
pub struct MyTreeMap{
    pub data_map: BTreeMap<char,String>
}

impl MyTreeMap {
    pub fn new_default() -> MyTreeMap{
        let tree: BTreeMap<char, String> = BTreeMap::new();
        MyTreeMap { data_map: tree }
    }
}