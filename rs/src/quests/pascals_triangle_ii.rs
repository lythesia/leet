/**
 * [119] Pascal's Triangle II
 *
 * Given an integer rowIndex, return the rowIndexth (0-indexed) row of the Pascal's triangle.
In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:
 
Example 1:
Input: rowIndex = 3
Output: [1,3,3,1]
Example 2:
Input: rowIndex = 0
Output: [1]
Example 3:
Input: rowIndex = 1
Output: [1,1]
 
Constraints:

	0 <= rowIndex <= 33

 
Follow up: Could you optimize your algorithm to use only O(rowIndex) extra space?

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let fac = |n: u128| -> u128 { (1..=n).product() };
        let comb = |m, n| (fac(n) / fac(m) / fac(n - m)) as i32; // C(m,n)
        (0..=row_index as u128).map(|i| comb(i, row_index as u128)).collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![1,3,3,1], Solution::get_row(3));
    }
}
