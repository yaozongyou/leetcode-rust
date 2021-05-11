// https://leetcode.com/problems/sliding-window-maximum/

struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if nums.is_empty() || k <= 1 {
            return nums;
        }

        let mut result = Vec::new();
        let mut max: Option<i32> = None;

        for i in 0..nums.len() - k as usize + 1 {
            if max.is_none() {
                max = Some(*nums[i..i + k as usize].iter().max().unwrap());
            } else {
                if nums[i + k as usize - 1] > max.unwrap() {
                    max = Some(nums[i + k as usize - 1]);
                } else {
                    if nums[i - 1] >= max.unwrap() {
                        max = Some(*nums[i..i + k as usize].iter().max().unwrap());
                    }
                }
            }
            result.push(max.unwrap());
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
        vec![3, 3, 5, 5, 6, 7]
    );
    assert_eq!(Solution::max_sliding_window(vec![4, -2], 2), vec![4]);
}
