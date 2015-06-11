#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;
typedef vector<char> vc;
typedef vector<vc> vvc;

class Solution {
public:
  int maximalSquare(vvc &mat) {
    if(!mat.size() || !mat[0].size()) return 0;
    int m = mat.size(), n = mat[0].size(), ans = 0;
    vi left(n), right(n), height(n);
    for(int i=0; i<m; i++) {
      // update height
      for(int j=0; j<n; j++) mat[i][j] == '1' ? height[j]++ : (height[j] = 0);
      // left
      fill(left.begin(), left.end(), 0);
      left[0] = 0;
      for(int j=1; j<n; j++) {
        left[j] = j;
        while(left[j]-1 >= 0 && height[j] <= height[left[j]-1]) left[j] = left[left[j]-1];
      }
      // right
      fill(right.begin(), right.end(), 0);
      right[n-1] = n-1;
      for(int j=n-2; ~j; j--) {
        right[j] = j;
        while(right[j]+1 < n && height[j] <= height[right[j]+1]) right[j] = right[right[j]+1];
      }
      // square
      for(int j=0; j<n; j++) {
        int a = min(height[j], right[j]-left[j]+1);
        ans = max(ans, a*a);
      }
    }
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  int m = 0, n = 0;
  scanf("%d%d", &m, &n);
  vvc mat(m, vc(n));
  for(int i=0; i<m; i++) {
    for(int j=0; j<n; j++) {
      int d;
      scanf("%d", &d);
      mat[i][j] = d+'0';
    }
  }
  for(int i=0; i<m; i++) {
    for(int j=0; j<n; j++) {
      cout << mat[i][j] << ' ';
    }
    cout << endl;
  }
  Solution so;
  cout << so.maximalSquare(mat) << endl;
  return 0;
}
