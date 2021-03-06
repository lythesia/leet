#include <cstdio>
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>

using namespace std;

typedef vector<char> vc;
typedef vector<vc> vvc;
class Solution {
public:
  bool dfs(vvc &b, int x, int y, string word, int c, vvc &v) {
    if(c == word.size()) return true;
    if(x<0 || x>=b.size() || y<0 || y>=b[0].size() || v[x][y]) return false;
    if(b[x][y] != word[c]) return false;
    //printf("%d,%d: %c\n", x, y, word[c]);
    v[x][y] = 1;
    ++c;
    bool res =
      dfs(b, x  , y-1, word, c, v) ||
      dfs(b, x  , y+1, word, c, v) ||
      dfs(b, x-1, y  , word, c, v) ||
      dfs(b, x+1, y  , word, c, v);
    v[x][y] = 0;
    return res;
  }

  bool exist(vvc &board, string word) {
    if(!board.size() || !board[0].size()) return !word.size();
    if(!word.size()) return true;
    int m = board.size(), n = board[0].size();
    bool res = false;
    vvc visited;
    for(int i=0; i<m; i++) visited.push_back(vc(n, 0));

    for(int i=0; i<m; i++) {
      for(int j=0; j<n; j++) {
        if(board[i][j] == word[0]) {
          for(auto &row : visited) fill(row.begin(), row.end(), 0);
          res = dfs(board, i, j, word, 0, visited);
          if(res) return true;
        }
      }
    }
    return res;
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  vvc board;
  board.push_back(vc{'A','B','C','E'});
  board.push_back(vc{'S','F','E','S'});
  board.push_back(vc{'A','D','E','E'});
  cout << so.exist(board, "ABCESEEEFS") << endl;
  return 0;
}
