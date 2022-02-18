/**
 * [371] Sum of Two Integers
 *
 * Given two integers a and b, return the sum of the two integers without using the operators + and -.
 
Example 1:
Input: a = 1, b = 2
Output: 3
Example 2:
Input: a = 2, b = 3
Output: 5
 
Constraints:

	-1000 <= a, b <= 1000

 */
pub struct Solution {}

// submission codes start here
/*
0110
1110
& = 0110 carry of each bit, it should be add to next higher bit, which is 0110<<1 = 1100
^ = 1000 sum of each bit
so sum is (& << 1 + ^), do it recursively until no carry

say negative 4-bit numbers, -1 and -2
1111
1110
+ = 11101 == -3

so it does not matter if positive or negative, just add bits up
 */
impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let (mut a, mut b) = (a, b);
        while b != 0 {
            let c: u32 = (a & b) as u32;
            a = a ^ b;
            b = (c << 1) as i32;
        }
        a
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, Solution::get_sum(1, 2));
        assert_eq!(1, Solution::get_sum(-1, 2));
        assert_eq!(-1, Solution::get_sum(1, -2));
    }
}
