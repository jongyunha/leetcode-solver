use crate::easy::score_of_a_string::Solution;

#[test]
fn test_score_of_a_string() {
    let s = "hello".to_string();
    let output = 13;
    assert_eq!(output, Solution::score_of_string(s));

    let s = "zaz".to_string();
    let output = 50;
    assert_eq!(output, Solution::score_of_string(s));
}