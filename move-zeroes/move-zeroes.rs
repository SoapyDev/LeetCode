impl Solution {


    pub fn move_zeroes(mut nums: &mut Vec<i32>) {

        let mut end = if nums.len() > 1 { 1 } else { return; };
        let mut start = 0;

     while end < nums.len(){
            while  end < nums.len() && nums[end] == 0 {
                end += 1;
            }
            while start < nums.len() &&  nums[start] != 0 {
                start += 1;
                if start == end {
                    end += 1;
                }
            }

            if end < nums.len() && nums[end] != 0 && nums[start] == 0 {
                nums[start] = nums[end];
                nums[end] = 0;
            }
        }

    }
}