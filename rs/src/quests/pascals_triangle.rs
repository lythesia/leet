/**
 * [118] Pascal's Triangle
 *
 * Given an integer numRows, return the first numRows of Pascal's triangle.
In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:
 
Example 1:
Input: numRows = 5
Output: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
Example 2:
Input: numRows = 1
Output: [[1]]
 
Constraints:

	1 <= numRows <= 30

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let fac = |n: u128| -> u128 { (1..=n).product() };
        let comb = |m, n| (fac(n) / fac(m) / fac(n - m)) as i32;
        (0..(num_rows as u128)).map(
            |i| (0..=i).map(|j| comb(j, i)).collect()
        ).collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec_vec![[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]], Solution::generate(25));
    }
}
