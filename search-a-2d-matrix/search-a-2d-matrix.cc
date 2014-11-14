#include <cstdio>
#include <iostream>
#include <vector>
using namespace std;

typedef vector<int> vi;
typedef vector<vi> vvi;
class Solution {
public:
  bool searchMatrix(vvi &matrix, int target) {
    if(!matrix.size() || !matrix[0].size()) return false;
    int m  = matrix.size(), n = matrix[0].size(), tot = m*n;
    int l = 0, h = tot;
    while(l < h) {
      int mid = (l+h)/2, x = matrix[mid/n][mid%n];
      if(target < x) h = mid;
      else if(target > x) l = mid+1;
      else return true;
    }
    return false;
  }
};

int main(int argc, const char *argv[])
{
  vvi m;
  m.push_back(vi{1,  3,  5,  7});
  m.push_back(vi{10, 11, 16, 20});
  m.push_back(vi{23, 30, 34, 50});
  Solution so;
  cout << so.searchMatrix(m, 4) << endl;
  return 0;
}
