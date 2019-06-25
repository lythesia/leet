#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
  /*
   * for binaries:
   *   1. when n minus 1, then the right-most bit 1(say k-th bit) of n becomes 0, and 0's on right all become 1
   *      (xxxx100 -> xxxx011)
   *   2. since we only count the right-most bit 1, if we do n & (n-1), then from k-th on all become 0
   *      (xxxx100 & xxxx011 -> xxxx000)
   *   3. do 1-2 until n becomes all 0
   */
  int hammingWeight(uint32_t n) {
    int c;
    for(c=0; n; ++c) n &= (n-1); // set right-most bit 1 -> 0
    return c;
  }

  /*
   * for oct:
   *   1. abc(2) = 4a + 2b + c, and # of bit 1 = a + b + c
   *   2. a + b + c = (2a + b) - (a) = (4a + 2b + c) >> 1 + (..) >> 2
   *   3. t + (t >> 3): adds up adj 2 3-bit
   *   4. & 030707070707 to remove duplicate computing in 3
   *   5. % 63 since: for 3-bit: 64^k*xk + .. 64^2*x2 + 64*x1 + x0 == xk + .. x0 mod 63
   */
  int hakman(uint32_t n) {
    unsigned int tmp = n - ((n >> 1) & 033333333333) - ((n >> 2) & 011111111111);
    return ((tmp + (tmp >> 3)) & 030707070707) % 63;
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  printf("%d\n", so.hammingWeight(11));
  printf("%d\n", so.hakman(11));
  return 0;
}
