#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
  int gao(int m, int n, int t) {
    if(!m) return 0;
    if(t <= m) return t | gao(m-t, n-t, t>>1);
    else if(n < t) return gao(m, n, t>>1);
    else return 0;
  }

  int rangeBitwiseAnd(int m, int n) {
    return gao(m, n, 1<<30);
  }
};

int main(int argc, const char *argv[])
{
  // int m = 0, n = 2; // 0
  // int m = 1, n = 2; // 0
  // int m = 1, n = 3; // 0
  // int m = 2, n = 5; // 0
  // int m = 4, n = 7; // 4
  int m = 4, n = 4; // 4
  Solution so;
  cout << so.rangeBitwiseAnd(m, n) << endl;
  return 0;
}
