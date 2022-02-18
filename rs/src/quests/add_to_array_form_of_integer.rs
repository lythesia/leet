/**
 * [989] Add to Array-Form of Integer
 *
 * The array-form of an integer num is an array representing its digits in left to right order.

	For example, for num = 1321, the array form is [1,3,2,1].

Given num, the array-form of an integer, and an integer k, return the array-form of the integer num + k.
 
Example 1:
Input: num = [1,2,0,0], k = 34
Output: [1,2,3,4]
Explanation: 1200 + 34 = 1234

Example 2:
Input: num = [2,7,4], k = 181
Output: [4,5,5]
Explanation: 274 + 181 = 455

Example 3:
Input: num = [2,1,5], k = 806
Output: [1,0,2,1]
Explanation: 215 + 806 = 1021

Example 4:
Input: num = [9,9,9,9,9,9,9,9,9,9], k = 1
Output: [1,0,0,0,0,0,0,0,0,0,0]
Explanation: 9999999999 + 1 = 10000000000

 
Constraints:

	1 <= num.length <= 104
	0 <= num[i] <= 9
	num does not contain any leading zeros except for the zero itself.
	1 <= k <= 104

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
        let mut ret = vec![];
        let mut c = 0;
        let mut k = k;
        for x in num.iter().rev() {
            let s = x + k % 10 + c;
            ret.push(s % 10);
            c = s / 10;
            k /= 10;
        }
        while k > 0 {
            let s = k % 10 + c;
            ret.push(s % 10);
            c = s / 10;
            k /= 10;
        }
        if c > 0 { ret.push(c); }
        ret.reverse();
        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![1,2,3,4], Solution::add_to_array_form(vec![1,2,0,0], 34));
        assert_eq!(vec![4,5,5], Solution::add_to_array_form(vec![2,7,4], 181));
        assert_eq!(vec![1,0,2,1], Solution::add_to_array_form(vec![2,1,5], 806));
        assert_eq!(vec![1,0,0,0,0,0,0,0,0,0,0], Solution::add_to_array_form(vec![9,9,9,9,9,9,9,9,9,9], 1));
    }
}
