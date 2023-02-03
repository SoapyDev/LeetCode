use std::collections::{HashMap};
impl Solution {
    
    pub fn single_number(nums: Vec<i32>) -> i32 {
        //use the fold operator to make a bitwise xor operation on the accumulator (acc)
        nums.iter().fold(0,|acc, x| acc^x)
    }
}