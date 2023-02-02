use std::collections::hash_map;
impl Solution {

    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        
        let mut map = hash_map::HashMap::new();
        for i in nums{
            if map.insert(i, ()).is_some() { return true }
        }
        false
    
    }
}