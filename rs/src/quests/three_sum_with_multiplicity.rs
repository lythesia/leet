/**
 * [923] 3Sum With Multiplicity
 *
 * Given an integer array arr, and an integer target, return the number of tuples i, j, k such that i < j < k and arr[i] + arr[j] + arr[k] == target.
As the answer can be very large, return it modulo 109 + 7.
 
Example 1:
Input: arr = [1,1,2,2,3,3,4,4,5,5], target = 8
Output: 20
Explanation: 
Enumerating by the values (arr[i], arr[j], arr[k]):
(1, 2, 5) occurs 8 times;
(1, 3, 4) occurs 8 times;
(2, 2, 4) occurs 2 times;
(2, 3, 3) occurs 2 times.

Example 2:
Input: arr = [1,1,2,2,2,2], target = 5
Output: 12
Explanation: 
arr[i] = 1, arr[j] = arr[k] = 2 occurs 12 times:
We choose one 1 from [1,1] in 2 ways,
and two 2s from [2,2,2,2] in 6 ways.

 
Constraints:

	3 <= arr.length <= 3000
	0 <= arr[i] <= 100
	0 <= target <= 300

 */
pub struct Solution {}

// submission codes start here

const MOD: i64 = 1_000_000_007;
impl Solution {
    pub fn three_sum_multi(mut arr: Vec<i32>, target: i32) -> i32 {
        let mut ans = 0i64; // avoid overflow
        let mut counter = [0; 101];
        arr.iter().for_each(|&x| counter[x as usize] += 1);
        for i in 0..=100 {
            for j in i..=100 {
                let k = target - i - j;
                if k >= j && k <= 100 {
                    let (a, b, c) = (counter[i as usize], counter[j as usize], counter[k as usize]);
                    ans = (ans + match (i == j, j == k) {
                        (false, false) => a*b*c,
                        (true, false) => a*(a-1)/2*c,
                        (false, true) => a*b*(b-1)/2,
                        _ => a*(a-1)*(a-2)/6,
                    })%MOD;
                }
            }
        }
        ans as _
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(20, Solution::three_sum_multi(vec![1,1,2,2,3,3,4,4,5,5], 8));
        assert_eq!(12, Solution::three_sum_multi(vec![1,1,2,2,2,2], 5));
    }
}
