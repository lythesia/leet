#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
  int trailingZeroes(int n) {
    int cnt = 0;
    // avoid n = INT_MAX
    for(long long i=5; i<=n; i*=5) cnt += (n / i);
    return cnt;
  }
};

int main(int argc, const char *argv[])
{
  // int n = 0;
  // int n = 1808548329;
  int n = 2147483647;
  Solution so;
  cout << so.trailingZeroes(n) << endl;
  return 0;
}
