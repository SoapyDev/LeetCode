impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut averages = Vec::with_capacity(nums.len());

    for i in 0..nums.len(){
        if i < k || i >= nums.len() - k {
            averages.push(-1);
            continue;
        }
        let sum: usize = nums[i - k..=i + k].iter().map(|&x| x as usize).sum();
        averages.push((sum / (2 * k + 1)) as i32);
    }

    averages

    }
}