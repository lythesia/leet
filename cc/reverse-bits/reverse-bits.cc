#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
  uint32_t reverseBits(uint32_t n) {
    uint32_t ans = 0;
    for(int i=0; i<32; i++) ans = (ans << 1) + (n >> i & 1);
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  uint32_t n = 43261596;
  Solution so;
  printf("%d\n", so.reverseBits(n));
  return 0;
}
