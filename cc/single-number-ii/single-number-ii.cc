#include <bits/stdc++.h>

using namespace std;

/*
 * A[0]: 0 1 ... 1 .. [0] ... 1  |
 * A[0]: 1 0 ... 1 .. [1] ... 1  |
 * A[0]: 0 0 ... 0 .. [1] ... 1  |
 * ...                           v
 *
 * every number represented in binary foramt, we add numbers in downward direction, then:
 * 1. (digit sum) % 3 on each bit(see column in []) can only be 0 or 1, because those 3-time number adds upto 3k,
 *    where 3k % 3 = 0, only leaves the 1-time number
 * 2. use these vars to record when iterate to k-th number:
 *    bit_one_time: those bits that 1 only appears one time are 1, other 0
 *    bit_two_times: ... two times
 *    bit_three_times: ... three times
 * 3. note that once we get bit_three_times, we must reset these bit to 0 both on bit_one_time and bit_two_times
 * 4. the bit_one_time is the leave along answer
 */
class Solution {
public:
  int singleNumber(int A[], int n) {
    int bit_one_time = 0, bit_two_times = 0, bit_three_times = 0;
    for(int i=0; i<n; i++) {
      int inc_two_times = bit_one_time & A[i];
      bit_two_times |= inc_two_times; // or bit_two_times ^= inc_two_times; since bit_one_time and bit_two_times are not overlapped!
      bit_one_time ^= A[i]; // new bit_one_time when A[i] added
      bit_three_times = bit_one_time & bit_two_times;
      bit_one_time &= ~bit_three_times;   // reset since reach 3 times
      bit_two_times &= ~bit_three_times;  // same
    }
    return bit_one_time;
  }
};

int main(int argc, const char *argv[])
{
  int A[] = { 2, 2, 3, 2 };
  Solution so;
  printf("%d\n", so.singleNumber(A, sizeof(A) / sizeof(int)));
  return 0;
}
