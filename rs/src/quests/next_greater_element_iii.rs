/**
 * [556] Next Greater Element III
 *
 * Given a positive integer n, find the smallest integer which has exactly the same digits existing in the integer n and is greater in value than n. If no such positive integer exists, return -1.
Note that the returned integer should fit in 32-bit integer, if there is a valid answer but it does not fit in 32-bit integer, return -1.
 
Example 1:
Input: n = 12
Output: 21
Example 2:
Input: n = 21
Output: -1
 
Constraints:

	1 <= n <= 231 - 1

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut v = vec![];
        let mut x = n;
        while x > 0 {
            v.push(x%10);
            x /= 10;
        }
        v.reverse();
        // next permutation
        let mut v = v.as_mut_slice();
        let mut i = v.len() - 1;
        while i > 0 && v[i-1] >= v[i] {
            i -= 1;
        }
        if i > 0 {
            let mut j = v.len() - 1;
            while j > i - 1 && v[j] <= v[i-1] {
                j -= 1;
            }
            v.swap(j, i - 1);

            v[i..].reverse();
            v.into_iter().try_fold(0i32, |acc, d|
                acc.checked_mul(10).and_then(|x| x.checked_add(*d))
            ).unwrap_or(-1)
        } else {
            -1
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(21, Solution::next_greater_element(12));
        assert_eq!(-1, Solution::next_greater_element(21));
        assert_eq!(-1, Solution::next_greater_element(2147483486));
    }
}
