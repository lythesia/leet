/**
 * [260] Single Number III
 *
 * Given an integer array nums, in which exactly two elements appear only once and all the other elements appear exactly twice. Find the two elements that appear only once. You can return the answer in any order.
You must write an algorithm that runs in linear runtime complexity and uses only constant extra space.
 
Example 1:
Input: nums = [1,2,1,3,2,5]
Output: [3,5]
Explanation:  [5, 3] is also a valid answer.

Example 2:
Input: nums = [-1,0]
Output: [-1,0]

Example 3:
Input: nums = [0,1]
Output: [1,0]

 
Constraints:

	2 <= nums.length <= 3 * 104
	-231 <= nums[i] <= 231 - 1
	Each integer in nums will appear twice, only two integers will appear once.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    // xor_all一定是a xor b的结果
    // low_bit是xor_all的最低位1bit, 那么相当于把数组分成两类, 一类A这个bit为1, 一类B这个bit不为1
    // 理论上数组应该是: k*2个数字, 每个a[1]..a[k]出现两次, 再加上x 和 y
    // 那么 x = x ^ all 2k个xor, y = y ^ 2k个xor, 显然可以假设x的对应low_bit为1, y的为0
    // 那么k个数中, m个为1, n个为0, m + n == k, 那么在2k中就有2m + 2n, 显然这2m xor 和 2n xor起来是0, 不影响x xor y的结果
    // 那么x = xor_all{2m个, x}, y = xor_all{2n个, y}
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let xor_all = nums.iter().fold(0, |acc, &i| acc ^ i);
        let low_bit = xor_all & -xor_all;
        let (mut x, mut y) = (0 ,0);
        for i in nums {
            if i & low_bit == 0 {
                x ^= i;
            } else {
                y ^= i;
            }
        }
        vec![x, y]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
    }
}
