#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Solution {
public:
  bool searchMatrix(vvi &mat, int t) {
    if(mat.empty() || mat[0].empty()) return false;
    auto r1 = lower_bound(mat.begin(), mat.end(), t, [](const vi &row, int val){return row.back() < val;}); // >=
    auto r2 = upper_bound(mat.begin(), mat.end(), t, [](int val, const vi &row){return val < row.front();});
    if(r1 == mat.end() || r2 == mat.begin()) return false;
    else {
      for(auto it=r1; it!=r2; it++) if(binary_search(it->begin(), it->end(), t)) return true;
      return false;
    }
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  // vvi mat = {
  //   {1,   4,  7, 11, 15},
  //   {2,   5,  8, 12, 19},
  //   {3,   6,  9, 16, 22},
  //   {10, 13, 14, 17, 24},
  //   {18, 21, 23, 26, 30}
  // };
  // cout << boolalpha << so.searchMatrix(mat, 5) << endl;
  // cout << boolalpha << so.searchMatrix(mat, 20) << endl;

  vvi mat = {{-5}};
  cout << boolalpha << so.searchMatrix(mat, -5) << endl;
  return 0;
}
