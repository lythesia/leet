#include <bits/stdc++.h>
using namespace std;

typedef vector<int> vi;
typedef vector<vi> vvi;
class Solution {
public:
  int minPathSum(vvi &grid) {
    if(grid.empty()) return 0;
    int m = grid.size(), n = grid[0].size();
    for(int i=0; i<m; i++) {
      for(int j=0; j<n; j++) {
        // (0,0)
        if(!i && !j) grid[i][j] = grid[i][j];
        // 1st row
        else if(!i && j) grid[i][j] = grid[i][j] + grid[i][j-1];
        // 1st col
        else if(i && !j) grid[i][j] = grid[i][j] + grid[i-1][j];
        // in between
        else grid[i][j] = min(grid[i][j-1], grid[i-1][j]) + grid[i][j];
      }
    }
    return grid[m-1][n-1];
  }
};

void pvvec(const vvi &a) {
  for(auto &v : a) {
    for(auto i : v) cout << i << ' '; cout << endl;
  }
  cout << endl;
}

int main(int argc, const char *argv[])
{
  vvi a;
  int m = 3, n = 4, sz = m * n;
  for(int i=0; i<m; i++) {
    vi _a;
    for(int j=0; j<n; j++) _a.push_back(rand() % sz);
    a.push_back(_a);
  }

  pvvec(a);
  Solution so;
  cout << so.minPathSum(a) << endl;
  return 0;
}
