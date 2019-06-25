#include <cstdio>
#include <iostream>
#include <vector>
using namespace std;

typedef vector<int> vi;
typedef vector<vi> vvi;

void see_mat(vvi &mat) {
  for(auto i : mat) {
    for(auto j : i) printf("%4d ", j); puts("");
  }
} 

class Solution {
public:
  void setZeroes(vvi &matrix) {
    if(!matrix.size() || !matrix[0].size()) return;
    int m = matrix.size(), n = matrix[0].size();
    bool rowz = false, colz = false; // whether first row/col has zero(rowz/colz indicates 1st row/col should be 0)
    for(int i=0; i<m; i++) 
      if(!matrix[i][0]) {
        colz = true;
        break;
      }
    for(int i=0; i<n; i++) 
      if(!matrix[0][i]) {
        rowz = true;
        break;
      }
    // then stores if n-th row/col should be 0 in 1st row/col
    for(int i=1; i<m; i++) for(int j=1; j<n; j++) if(!matrix[i][j]) matrix[0][j] = 0, matrix[i][0] = 0;
    // set rows/cols to 0 if correspond 1st col/row entry is 0, but NOT set 1st row/col to 0
    for(int i=1; i<m; i++) for(int j=1; j<n; j++) if(!matrix[i][0] || !matrix[0][j]) matrix[i][j] = 0;
    // at last deal 1st row/col respect to rowz/colz
    if(rowz) for(int j=0; j<n; j++) matrix[0][j] = 0;
    if(colz) for(int i=0; i<m; i++) matrix[i][0] = 0;
  }
};

int main(int argc, const char *argv[])
{
  int m, n;
  scanf("%d%d", &m, &n);
  vvi mat;
  for(int i=0; i<m; i++) {
    vi row;
    for(int j=0; j<n; j++) {
      int x;
      scanf("%d", &x);
      row.push_back(x);
    }
    mat.push_back(row);
  }

  see_mat(mat);
  puts("-------------------------------------");

  Solution so;
  so.setZeroes(mat);
  see_mat(mat);
  return 0;
}
