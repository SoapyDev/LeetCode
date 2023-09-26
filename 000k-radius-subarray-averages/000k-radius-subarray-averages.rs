impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let window = k * 2 + 1;
    let len = nums.len();
    let mut averages = vec![-1; len];
    let temp_nums = nums.iter().map(|&x| x as usize).collect::<Vec<usize>>();

    for i in 0..len {
        if i < k || i >= len - k {
            continue;
        }
        let sum: usize = temp_nums[i - k..=i + k].iter().sum();
        averages[i] = (sum / window) as i32;
    }

    averages
    }
}