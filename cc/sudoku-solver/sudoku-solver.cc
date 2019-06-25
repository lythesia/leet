// #include <bits/stdc++.h>
#include <cstdio>
#include <iostream>
#include <cstring>
#include <vector>
#include <utility>
#include <iterator>
using namespace std;

typedef vector<char> vc;
typedef vector<vc> vvc;
typedef vector<bool> vb;

void pboard(vvc &board) {
  for(auto &v : board) {
    copy(v.begin(), v.end(), ostream_iterator<char>(cout, " ")); cout << endl;
  }
}
class Solution {
public:
  bool check(vvc &board, int i, int j) {
    bool v[10] = { false };
    // row
    for(int k=0; k<9; k++) {
      char ch = board[i][k];
      if(ch != '.') {
        if(!v[ch-'0']) v[ch-'0'] = true;
        else return false;
      }
    }
    // col
    memset(v, 0, sizeof(v));
    // fill(v.begin(), v.end(), false);
    for(int k=0; k<9; k++) {
      char ch = board[k][j];
      if(ch != '.') {
        if(!v[ch-'0']) v[ch-'0'] = true;
        else return false;
      }
    }
    // 3x3
    memset(v, 0, sizeof(v));
    // fill(v.begin(), v.end(), false);
    int ii = i / 3, jj = j / 3;
    for(int k=0; k<9; k++) {
      int r = ii*3 + k/3, c = jj*3 + k%3;
      char ch = board[r][c];
      if(ch != '.') {
        if(!v[ch-'0']) v[ch-'0'] = true;
        else return false;
      }
    }
    return true;
  }

  bool dfs(vvc &board, int k) {
    if(k == 81) return true;
    int i = k/9, j = k%9;
    char c = board[i][j];
    if(c != '.') return dfs(board, k+1);
    else {
      for(char t='1'; t<='9'; t++) {
        board[i][j] = t;
        if(check(board, i, j)) {
          if(dfs(board, k+1)) return true;
        }
      }
        board[i][j] = '.';
    }
    return false;
  }

  void solveSudoku(vvc &board) {
    dfs(board, 0);
  }
};

int main(int argc, const char *argv[])
{
  vvc board(9, vc(9));
  Solution so;
  for(int i=0; i<9; i++) {
    for(int j=0; j<9; j++) {
      int x = 0;
      scanf("%d", &x);
      board[i][j] = !x ? '.' : x+'0';
    }
  }
  so.solveSudoku(board);
  pboard(board);
  return 0;
}
