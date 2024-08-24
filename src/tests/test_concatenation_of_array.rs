use crate::easy;

#[test]
fn test_get_concatenation() {
    let nums = vec![1, 2, 1];
    let expected = vec![1, 2, 1, 1, 2, 1];
    assert_eq!(expected, easy::concatenation_of_array::Solution::get_concatenation(nums));
}