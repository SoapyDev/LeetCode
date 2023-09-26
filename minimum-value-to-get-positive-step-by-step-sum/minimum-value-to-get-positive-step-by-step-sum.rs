impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut min = 1;
        let mut value = 1;
        
        for num in nums {
            value += num;
            if value < 1{
                min += 1 - value;
                value = 1;
            }
        }
        min
    }
}