// https://leetcode.com/problems/valid-anagram/

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let left = &mut [0; 26];
        let right = &mut [0; 26];
        s.chars().for_each(|char| left[char as usize - 'a' as usize] += 1);
        t.chars().for_each(|char| right[char as usize - 'a' as usize] += 1);

        left == right
    }
}

fn main() {
    assert!(Solution::is_anagram("anagram".to_owned(), "nagaram".to_owned()));
    assert!(!Solution::is_anagram("rat".to_owned(), "car".to_owned()));
}
