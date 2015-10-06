#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

const int dir[][2] = {
  {-1, -1},
  {-1, 0},
  {-1, 1},
  {0, -1},
  {0, 1},
  {1, -1},
  {1, 0},
  {1, 1}
};
class Solution {
public:
  void gameOfLife(vvi &b) {
    int m = b.size(), n = b[0].size();
    for(int i=0; i<m; i++) for(int j=0; j<n; j++) if(!b[i][j]) b[i][j] = -1;
    for(int i=0; i<m; i++) {
      for(int j=0; j<n; j++) {
        int c = 0;
        for(int k=0; k<8; k++) {
          int nx = i + dir[k][0], ny = j + dir[k][1];
          if(nx >= 0 && nx < m && ny >=0 && ny < n) {
            if(b[nx][ny] >= 0) c++;
          }
        }
        if(c) b[i][j] = b[i][j] >= 0 ? c : b[i][j] * c;
      }
    }
    for(int i=0; i<m; i++) {
      for(int j=0; j<n; j++) {
        if(b[i][j] < 0) b[i][j] = b[i][j] == -3 ? 1 : 0;
        else b[i][j] = b[i][j] == 2 || b[i][j] == 3 ? 1 : 0;
      }
    }
  }
};

void pmat(vvi &b) {
  for(size_t i=0; i<b.size(); i++) {
    for(size_t j=0; j<b[i].size(); j++) cout << b[i][j] << ' ';
    cout << endl;
  }
}

int main(int argc, const char *argv[])
{
  srand(time(nullptr));
  vvi b(4, vi(4));
  for(size_t i=0; i<b.size(); i++) {
    size_t cc = rand() % b[i].size() + 1;
    for(size_t j=0; j<cc; j++) b[i][j] = 1;
    random_shuffle(b[i].begin(), b[i].end());
  }
  pmat(b);
  cout << endl;
  Solution so;
  so.gameOfLife(b);
  pmat(b);
  return 0;
}
