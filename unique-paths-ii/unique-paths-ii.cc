#include <bits/stdc++.h>
using namespace std;

typedef vector<int> vi;
typedef vector<vi> vvi;
class Solution {
public:
  int uniquePathsWithObstacles(vvi &a) {
    if(a[0][0]) return 0;
    int m = a.size(), n = a[0].size();
    a[0][0] = 1;
    for(int i=0; i<m; i++) for(int j=0; j<n; j++)
      if(!i && !j) continue; else a[i][j] = a[i][j] ? 0:(i ? a[i-1][j]:0) + (j ? a[i][j-1]:0);
    return a[m-1][n-1];
  }
};

int main(int argc, const char *argv[])
{
  vvi a = {
    {0,0},
    {1,1},
    {0,0}
  };
  Solution so;
  cout << so.uniquePathsWithObstacles(a) << endl;
  return 0;
}
