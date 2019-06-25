#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <iostream>
#include <algorithm>

using namespace std;

class Solution {
public:
  int climbStairs(int n) {
    int res = 0, n_1 = 2, n_2 = 1;
    if(n == 1) return 1;
    else if(n == 2) return 2;
    for(int i=3; i<=n; i++) {
      res = n_1 + n_2;
      n_2 = n_1;
      n_1 = res;
    }
    return res;
  }

  /*
   * another
   */
  int climbStairs2(int n)
  {
    vector<int> res(3);
    res[0] = 1;
    res[1] = 1;
    for (int i = 2; i <= n; i++)
    {
      res[i%3] = res[(i-1)%3] + res[(i-2)%3];
    }
    return res[n%3];
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  cout << so.climbStairs(atoi(argv[1])) << endl;
  return 0;
}
