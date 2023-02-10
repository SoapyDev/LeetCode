impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        
        let mut current = 0;
        let mut answer = 0;
        let mut left = 0;

        for right in 0..nums.len() {

            if nums[right] == 0 {
                current += 1;
            }

            while current > k{
                if nums[left] == 0 {
                    current -=1;
                }
                left += 1;
            }

            answer = answer.max(right - left + 1);

        }


        answer as i32
        
        
    }
}