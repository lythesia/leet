#include <bits/stdc++.h>
using namespace std;

typedef vector<int> vi;
class Solution {
public:
  int jump(vi &n) {
    int len = n.size();
    if(len < 2) return 0;
    int p = 0, ans = 0;
    while(n[p]) {
      int mmax = p;
      if(p + n[p] >= len-1) return ans+1; // current can jump to end directly
      for(int i=1; i<=n[p] && p+i<len; i++) {
        int j = p+i + n[p+i];
        if(j >= mmax+n[mmax]) mmax = p+i; // find longest jump
      }
      p = mmax, ans++;
      if(p >= len-1) break;       // longest can to end
      else if(p+n[p] >= len-1) {  // or next-longest can to end
        ans++;
        break;
      }
    }
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  // vi v = {2,3,1,1,4}; // 2
  // vi v = {1,2}; // 1
  vi v = {2,3,1}; // 1
  Solution so;
  cout << so.jump(v) << endl;
  return 0;
}
