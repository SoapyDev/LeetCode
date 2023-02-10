impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        

    let mut squared_nums = Vec::with_capacity(nums.len());
        nums.into_iter().for_each(|num| squared_nums.push(num * num));
    
    squared_nums.sort();
    squared_nums
        
    }
}