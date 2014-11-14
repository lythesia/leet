#include <cstdio>
#include <iostream>
#include <vector>
using namespace std;

typedef vector<int> vi;
typedef vector<vector<int> > vvi;
#define __incdir(x) x = ((x) + 1)%4 

class Solution {
public:
  vector<vector<int> > generateMatrix(int n) {
    vvi res;
    for(int i=0; i<n; i++) {
      vi r;
      for(int j=0; j<n; j++) r.push_back(0);
      res.push_back(r);
    }
    int cnt = 1, dir = 0, r1 = 0, r2 = n, c1 = 1, c2 = n, i = 0, j = 0;
    while(cnt <= n*n) {
      switch(dir) {
        case 0:
          if(i < c2) {
            res[i][j++] = cnt++;
            if(j == c2) { // right wall
              --j,++i;
              __incdir(dir);
              --c2;
            }
          }
        break;
        case 1:
          if(j < r2) {
            res[i++][j] = cnt++;
            if(i == r2) { // down wall
              --i,--j;
              __incdir(dir);
              --r2;
            }
          }
        break;
        case 2:
          if(j >= r1) {
            res[i][j--] = cnt++;
            if(j < r1) { // left wall
              ++j,--i;
              __incdir(dir);
              ++r1;
            }
          }
        break;
        case 3:
          if(i >= c1) {
            res[i--][j] = cnt++;
            if(i < c1) { // up wall
              ++i,++j;
              __incdir(dir);
              ++c1;
            }
          }
        break;
      }
    }
    return res;
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  vvi res = so.generateMatrix(4);
  for(auto i : res) {
    for(auto ii : i) cout << ii << ' '; cout << endl;
  }
  return 0;
}
