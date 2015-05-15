#include <bits/stdc++.h>
using namespace std;

typedef vector<char> vc;
typedef vector<vc> vvc;
typedef pair<int,int> pii;
static int dir[][2] = {
  {0,1},
  {1,0},
  {0,-1},
  {-1,0}
};
class Solution {
public:
  inline bool out(int i, int j, int m, int n) {
    return i<0 || i>=m || j<0 || j>=n;
  }

  void solve(vvc &b) {
    if(!b.size() || !b[0].size()) return;
    int m = b.size(), n = b[0].size();
    if(m*n == 1) return;
    vector<vector<int>> mark(m, vector<int>(n, 0));
    int si = 0, sj = 0, w = (m+n)*2 - 4, d = 0;
    if(!w) w = 1; // only one
    for(int i=0; i<w;) {
      printf("(%d, %d):\n", si, sj);
      if(b[si][sj] == 'O') {
        // bfs
        queue<pii> Q;
        Q.push(pii(si, sj));
        mark[si][sj] = 1;
        while(!Q.empty()) {
          auto t = Q.front();
          Q.pop();
          for(int k=0; k<4; k++) {
            int ni = t.first+dir[k][0], nj = t.second+dir[k][1];
            if(!out(ni, nj, m, n) && !mark[ni][nj] && b[ni][nj] == 'O') {
              mark[ni][nj] = 1;
              Q.push(pii(ni, nj));
            }
          }
        }
      }
      int ti = si+dir[d][0], tj =  sj+dir[d][1];
      out(ti, tj, m, n) ? (++d, si = si+dir[d][0], sj = sj+dir[d][1]) : (si = ti, sj = tj);
      i++;
    }
    for(int i=0; i<m; i++) {
      for(int j=0; j<n; j++) {
        printf("%d ", mark[i][j]);
        if(b[i][j] == 'O' && !mark[i][j]) b[i][j] = 'X';
      }
      puts("");
    }
  }
};

int main(int argc, const char *argv[])
{
  typedef vector<string> vs;
  // 'O' not '0' !!
  // vs rg = {
  //   "XXXX",
  //   "X00X",
  //   "XX0X",
  //   "X0XX"
  // };
  vs rg = {
    "O",
  };
  vvc g;
  for(auto &s : rg) g.push_back(vc(s.begin(), s.end()));
  for(auto &v : g) {
    copy(v.begin(), v.end(), ostream_iterator<char>(cout, " ")); cout << endl;
  }
  puts("----");
  Solution so;
  so.solve(g);
  for(auto &v : g) {
    copy(v.begin(), v.end(), ostream_iterator<char>(cout, " ")); cout << endl;
  }
  return 0;
}
