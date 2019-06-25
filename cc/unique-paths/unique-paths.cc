#include <cstdio>
#include <iostream>

using namespace std;

class Solution {
public:
  int uniquePaths(int m, int n) {
    return nCr(m+n-2, n-1);
  }

  int nCr(int m, int n) {
    long double res = 1;
    if(n > m - n) n = m - n;
    for(int i=m,cnt=1; cnt<=n; i--,cnt++) {
      res *= i;
    }
    for(int i=1; i<=n; i++) {
      res /= i;
    }
    return static_cast<int>(res);
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  cout << so.uniquePaths(3, 7) << endl;
  return 0;
}
