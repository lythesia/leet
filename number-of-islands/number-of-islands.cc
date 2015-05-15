#include <bits/stdc++.h>
using namespace std;

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<char> vc;
typedef vector<vc> vvc;
const int dir[][2] = {
  {-1,  0 }, // up
  { 1,  0 }, // down
  { 0, -1 }, // left
  { 0,  1 }  // right
};
class Solution {
public:
  int numIslands(vvc &g) {
    if(!g.size() || !g[0].size()) return 0;
    int m = g.size(), n = g[0].size();
    vector<vector<bool>> v(m, vector<bool>(n, false));
    int ans = 0;
    for(int i=0; i<m; i++) {
      for(int j=0; j<n; j++) {
        if(v[i][j] || g[i][j] == '0') continue;
        // bfs
        queue<int> Q;
        v[i][j] = true;
        Q.push(i*n+j);
        while(!Q.empty()) {
          int t = Q.front(), ii = t / n, jj = t % n;
          g[ii][jj] = '0';
          Q.pop();
          for(int k=0; k<4; k++) {
            int ni = ii + dir[k][0], nj = jj + dir[k][1];
            if((ni >=0 && ni <= m-1 && nj >=0 && nj <= n-1) &&  // range
                (!v[ni][nj] && g[ni][nj] == '1')) {
              Q.push(ni*n+nj);
              v[ni][nj] = true; // mark it first!
            }
          }
        }
        ans++;
      }
    }
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  // vvi g = {
  //   {1,1,1,1,0},
  //   {1,1,0,1,0},
  //   {1,1,0,0,0},
  //   {0,0,0,0,0},
  // };

  // vvc g = {
  //   {'1','1','0','0','0'},
  //   {'1','1','1','0','0'},
  //   {'0','0','1','0','0'},
  //   {'0','0','0','1','1'},
  // };

  typedef vector<string> vs;
  vs rg = {
    "11111011111111101011",
    "01111111111110111110",
    "10111001101111111111",
    "11110111111111111111",
    "10011111111111111111",
    "10111111011101110111",
    "01111111111101101111",
    "11111111111101111011",
    "11111111110111111111",
    "11111111111111111111",
    "01111111011111111111",
    "11111111111111111111",
    "11111111111111111111",
    "11111011111110111111",
    "10111110111011110111",
    "11111111111101111110",
    "11111111111110111100",
    "11111111111111111111",
    "11111111111111111111",
    "11111111111111111111"
  };
  vvc g;
  for(auto &s : rg) g.push_back(vc(s.begin(), s.end()));
  Solution so;
  cout << so.numIslands(g) << endl;
  return 0;
}
