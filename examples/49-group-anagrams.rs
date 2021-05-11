// https://leetcode.com/problems/group-anagrams/

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups = HashMap::new();

        strs.into_iter().for_each(|str| {
            let fingerprint = &mut [0; 26];
            str.chars().for_each(|char| fingerprint[char as usize - 'a' as usize] += 1);
            let anagrams = groups.entry(*fingerprint).or_insert(Vec::new());
            anagrams.push(str);
        });

        groups.values().cloned().collect::<Vec<_>>()
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::group_anagrams(vec![
            "eat".to_owned(),
            "tea".to_owned(),
            "tan".to_owned(),
            "ate".to_owned(),
            "nat".to_owned(),
            "bat".to_owned()
        ])
    );
    println!("{:?}", Solution::group_anagrams(vec!["".to_owned()]));
    println!("{:?}", Solution::group_anagrams(vec!["a".to_owned()]));
}
