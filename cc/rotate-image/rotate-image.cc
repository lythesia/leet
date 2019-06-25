#include <cstdio>
#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

#define NO_LEET

typedef vector<int> vi;
typedef vector<vi> vvi;

class Solution {
public:
  void rotate(vvi &matrix) {
#ifdef NO_LEET
    if(!matrix.size() || !matrix[0].size()) return;
#endif
    int m = matrix.size(), n = matrix[0].size();
#ifdef NO_LEET
    matrix.resize(max(m, n), vi(n));
    for(auto &i : matrix) i.resize(max(m, n));
#endif
    /* key part */
    reverse(matrix.begin(), matrix.end());
    for(int i=0; i<m; i++) for(int j=i+1; j<n; j++) swap(matrix[i][j], matrix[j][i]);
#ifdef NO_LEET
    matrix.resize(n);
    for(auto &i : matrix) i.erase(i.begin());
#endif
  }
};

int main(int argc, const char *argv[])
{
  vvi m = {
    {0, 1, 2, 3},
    {4, 5, 6, 7},
    {8, 9, 10, 11}
  };
  Solution so;
  so.rotate(m);
  cout << endl;
  for(auto i : m) {
    for(auto ii : i) cout << ii << ' '; cout << endl;
  }
  return 0;
}
