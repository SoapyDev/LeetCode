use std::collections::HashMap;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        
        //create a hashmap with a capacity equal to nums1
        let mut map = HashMap::with_capacity(nums1.len());
        let mut result:Vec<i32> = Vec::new();

        //fill the hashmap and add 1 for every duplicate
        nums1.into_iter().for_each(|val| *map.entry(val).or_insert(0) +=1);

        // go through nums2 and if num is in map and the value is greater than 0, push to result and
        // decrement value by 1;
        for num in nums2 {
            if map.get(&num) > Some(&0) {
                map.entry(num).and_modify(|v| { *v -= 1 });
                result.push(num);
            }
        }

        result
    }
}