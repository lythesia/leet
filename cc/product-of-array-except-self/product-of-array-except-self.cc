#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Solution {
public:
  vi productExceptSelf(vi &n) {
    vi ans;
    long all = 1, nz = 1;
    int z = 0;
    for(int x : n) (x ? nz *= x : z++), all *= x;
    for(int x : n) {
      if(x) ans.push_back(int(all / x));
      else if(z > 1) ans.push_back(0);
      else ans.push_back(nz);
    }
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  vi n = {1,2,3,4};
  Solution so;
  vi ans = so.productExceptSelf(n);
  copy(ans.begin(), ans.end(), ostream_iterator<int>(cout, " ")); cout << endl;
  return 0;
}
