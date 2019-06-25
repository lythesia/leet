#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <iostream>
#include <vector>

using namespace std;

typedef vector<char> vc;
typedef vector<vc> vvc;
class Solution {
public:
  bool isValidSudoku(vvc &board) {
    if(!board.size() || !board[0].size()) return false;
    static int index[][2] = {
      {0,0}, {0,3}, {0,6},
      {3,0}, {3,3}, {3,6},
      {6,0}, {6,3}, {6,6}
    };
    int b[9] = {0};
    // row
    for(int i=0; i<9; i++) {
      memset(b, 0, sizeof(b));
      for(int j=0; j<9; j++) {
        if(board[i][j] == '.') continue;
        int d = board[i][j] - '0';
        if(!b[d-1]) b[d-1] = 1;
        else return false; 
      } 
    }
    memset(b, 0, sizeof(b));
    // col
    for(int i=0; i<9; i++) {
      memset(b, 0, sizeof(b));
      for(int j=0; j<9; j++) {
        if(board[j][i] == '.') continue;
        int d = board[j][i] - '0';
        if(!b[d-1]) b[d-1] = 1;
        else return false; 
      } 
    }
    memset(b, 0, sizeof(b));
    // sub
    for(int i=0; i<9; i++) {
      memset(b, 0, sizeof(b));
      for(int j=0; j<3; j++) {
        for(int k=0; k<3; k++) {
          if(board[index[i][0]+j][index[i][1]+k] == '.') continue;
          int d = board[index[i][0]+j][index[i][1]+k] - '0';
          if(!b[d-1]) b[d-1] = 1;
          else return false;
        }
      }
    }
    return true;
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  vvc board;
  int x;
  for(int i=0; i<9; i++) {
    vc r;
    for(int j=0; j<9; j++) {
      scanf("%d", &x); 
      r.push_back( x ? (char)(x+'0') : '.');
    }
    board.push_back(r);
  }
  cout << so.isValidSudoku(board) << endl;
  return 0;
}
