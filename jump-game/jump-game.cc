#include <bits/stdc++.h>
using namespace std;

typedef vector<int> vi;
class Solution {
public:
  bool canJump(vi &n) {
    if(n.size() < 2) return true;
    int l = n.size() - 1, p = 0;
    while(n[p]) {
      int mmax = p;
      for(int i=1; i<=n[p] && p+i <= l; i++) {
        if(p+i+n[p+i] >= mmax + n[mmax]) mmax = p+i;
      }
      p = mmax;
      if(p + n[p] >= l) return true;
    }
    return false;
  }
};

int main(int argc, const char *argv[])
{
  vi v = {2,3,1,1,4};
  // vi v = {3,2,1,0,4};
  Solution so;
  cout << boolalpha << so.canJump(v) << endl;
  return 0;
}
