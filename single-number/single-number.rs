use std::collections::{HashMap};
impl Solution {
    
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut map:HashMap<i32, i32> = HashMap::with_capacity(nums.len() / 2 + 1);

        for num in nums {
            if let std::collections::hash_map::Entry::Vacant(e) = map.entry(num) {
                e.insert(1);
            } else {
                map.remove(&num);
            }
        }

        for (key, value) in map.iter(){
            if *value == 1{
                return *key;
            }
        }
        0
    }
}