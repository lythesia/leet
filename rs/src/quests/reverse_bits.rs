/**
 * [190] Reverse Bits
 *
 * Reverse bits of a given 32 bits unsigned integer.
Note:

	Note that in some languages, such as Java, there is no unsigned integer type. In this case, both input and output will be given as a signed integer type. They should not affect your implementation, as the integer's internal binary representation is the same, whether it is signed or unsigned.
	In Java, the compiler represents the signed integers using 2's complement notation. Therefore, in Example 2 above, the input represents the signed integer -3 and the output represents the signed integer -1073741825.

 
Example 1:
Input: n = 00000010100101000001111010011100
Output:    964176192 (00111001011110000010100101000000)
Explanation: The input binary string 00000010100101000001111010011100 represents the unsigned integer 43261596, so return 964176192 which its binary representation is 00111001011110000010100101000000.

Example 2:
Input: n = 11111111111111111111111111111101
Output:   3221225471 (10111111111111111111111111111111)
Explanation: The input binary string 11111111111111111111111111111101 represents the unsigned integer 4294967293, so return 3221225471 which its binary representation is 10111111111111111111111111111111.

 
Constraints:

	The input must be a binary string of length 32

 
Follow up: If this function is called many times, how would you optimize it?

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    // using lowbit is complicated..
    pub fn reverse_bits(x: u32) -> u32 {
        let low_bit = |x: u64| (x as i64 & -(x as i64)) as u64;

        let mut x = x as u64;
        let mut hi = 1;
        let mut ans = 1;

        while x > 0 {
            let low = low_bit(x);
            hi *= low<<1;
            ans = ans*(low<<1) + 1;
            x /= (low<<1)
        }

        let mul = (1 << 32) / hi;
        (ans * mul) as u32
    }

    // divide & conquer
    // pub fn reverse_bits(x: u32) -> u32 {
    //     let mut n = x << 16 | x >> 16; // swap 16-bit group
    //     n = (n & 0x00ff00ff) << 8 | (n & 0xff00ff00) >> 8; // swap 8-bit group
    //     n = (n & 0x0f0f0f0f) << 4 | (n & 0xf0f0f0f0) >> 4; // and so on..
    //     n = (n & 0x33333333) << 2 | (n & 0xcccccccc) >> 2;
    //     n = (n & 0x55555555) << 1 | (n & 0xaaaaaaaa) >> 1;
    //     n
    // }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(964176192, Solution::reverse_bits(0b00000010100101000001111010011100));
        assert_eq!(3221225471, Solution::reverse_bits(0b11111111111111111111111111111101));
        assert_eq!(0, Solution::reverse_bits(0));
        assert_eq!(1, Solution::reverse_bits(0b10000000000000000000000000000000));
    }
}
