impl Solution {


    pub fn move_zeroes(mut nums: &mut Vec<i32>) {

         let mut pointer = 0;
            let mut temp;
            for index in 0..nums.len() {
                if nums[pointer] == 0 {
                    temp = nums[index];
                    nums[index] = nums[pointer];
                    nums[pointer] = temp;
                }
                if nums[pointer] != 0 {
                    pointer += 1;
                }
            }

    }
}