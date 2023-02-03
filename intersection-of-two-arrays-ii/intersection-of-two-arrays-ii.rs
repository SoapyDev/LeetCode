use std::collections::HashMap;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        
        let capacity = if nums1.len() >= nums2.len()  { nums1.len() }else { nums2.len() };
        let mut map = HashMap::with_capacity(capacity);

        for &num in &nums1{
            *map.entry(num).or_insert(0) += 1;
        }

        let mut result:Vec<i32> = Vec::new();

        for num in nums2 {
            if map.get(&num) > Some(&0) {
                map.entry(num).and_modify(|e| { *e -= 1 });
                result.push(num);
            }
        }

        result
    }
}