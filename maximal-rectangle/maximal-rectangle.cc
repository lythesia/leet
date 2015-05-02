#include <bits/stdc++.h>
using namespace std;

typedef vector<char> vc;
typedef vector<vc> vvc;
typedef vector<int> vi;
class Solution {
public:
  int maximalRectangle(vvc &mat) {
    if(!mat.size() || !mat[0].size()) return 0;
    int m = mat.size(), n = mat[0].size(), ans = 0;
    vi left(n, 0), right(n, 0), height(n, 0);
    for(int k=0; k<m; k++) {
      // init
      fill(left.begin(), left.end(), 0);
      fill(right.begin(), right.end(), 0);
      left[0] = 0, right[n-1] = n-1;
      if(mat[k][0] == '1') height[0]++;
      else height[0] = 0;
      // dp range
      for(int i=1; i<n; i++) {
        if(mat[k][i] == '1') height[i]++;
        else height[i] = 0;
        left[i] = i;
        while(left[i]-1>=0 && height[i] <= height[left[i]-1]) left[i] = left[left[i]-1];
      }
      for(int i=n-2; i>=0; i--) {
        right[i] = i;
        while(right[i]+1<n && height[i] <= height[right[i]+1]) right[i] = right[right[i]+1];
      }
      // find mmax
      int mmax = 0;
      for(int i=0; i<n; i++) mmax = max(mmax, (right[i]-left[i]+1)*height[i]);
      ans = max(ans, mmax);
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
  cout << so.maximalRectangle(mat) << endl;
  return 0;
}
