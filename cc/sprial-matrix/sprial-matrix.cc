#include <cstdio>
#include <iostream>

using namespace std;

class Solution {
public:
  vector<int> spiralOrder(vector<vector<int> > &vi) {
    vector<int> vo;
    if(vi.size() == 0 || vi[0].size() == 0) return vo;
    int m = vi.size(), n = vi[0].size(), dir = 0, cnt = m*n, i = 1, j = 0;
    int ii = 0, jj = 0;
    while(cnt) {
      switch(dir) {
        case 0:
          if(jj < n) { 
            vo.push_back(vi[ii][jj++]), cnt--;
            if(jj == n) {
              jj--; ii++; dir = (dir+1) % 4;
              n--;
            }
          }
          break;
        case 1:
          if(ii < m) {
            vo.push_back(vi[ii++][jj]), cnt--;
            if(ii == m) {
              ii--; jj--; dir = (dir + 1) % 4;
              m--;
            }
          }
          break;
        case 2:
          if (jj >= j) {
            vo.push_back(vi[ii][jj--]), cnt--;
            if(jj < j) {
              jj++; ii--; dir = (dir + 1) % 4;
              j++;
            }
          }
          break;
        case 3:
          if (ii >= i) {
            vo.push_back(vi[ii--][jj]), cnt--;
            if(ii < i) {
              ii++; jj++; dir = (dir + 1) % 4;
              i++;
            }
          }
          break;
      }
    }
    return vo;
  }
};
