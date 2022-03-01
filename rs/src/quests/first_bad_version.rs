/**
 * [278] First Bad Version
 *
 * You are a product manager and currently leading a team to develop a new product. Unfortunately, the latest version of your product fails the quality check. Since each version is developed based on the previous version, all the versions after a bad version are also bad.
Suppose you have n versions [1, 2, ..., n] and you want to find out the first bad one, which causes all the following ones to be bad.
You are given an API bool isBadVersion(version) which returns whether version is bad. Implement a function to find the first bad version. You should minimize the number of calls to the API.
 
Example 1:
Input: n = 5, bad = 4
Output: 4
Explanation:
call isBadVersion(3) -> false
call isBadVersion(5) -> true
call isBadVersion(4) -> true
Then 4 is the first bad version.

Example 2:
Input: n = 1, bad = 1
Output: 1

 
Constraints:

	1 <= bad <= n <= 231 - 1

 */
pub struct Solution {
    is_bad: fn(i32) -> bool,
}
trait IsBad {
    fn isBadVersion(&self, x: i32) -> bool;
}
impl IsBad for Solution {
    fn isBadVersion(&self, x: i32) -> bool {
        (self.is_bad)(x)
    }
}

// submission codes start here

// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    fn new(f: fn(i32) -> bool) -> Self {
        Self { is_bad: f, }
    }

    pub fn first_bad_version(&self, n: i32) -> i32 {
        let (mut s, mut e) = (1, n);
        while s <= e {
            let m = s + (e-s)/2;
            if self.isBadVersion(m) { e = m - 1; }
            else { s = m + 1; }
        }
        s
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, Solution::new(|x| x>=1).first_bad_version(3));
    }
}
