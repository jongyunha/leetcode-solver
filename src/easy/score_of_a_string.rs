use std::ops::Sub;

pub struct Solution;
impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let bytes = s.as_bytes();
        (0..s.len() - 1)
            .map(|i| (bytes[i] as i32 - bytes[i + 1] as i32).abs())
            .sum()
    }
}