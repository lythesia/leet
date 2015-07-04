#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Solution {
public:
  vi majorityElement(vi n) {
    int a, b, ca = 0, cb = 0;
    for(int i : n) {
      if(i == a) ca++;
      else if(i == b) cb++;
      else if(!ca) a = i, ca = 1;
      else if(!cb) b = i, cb = 1;
      else ca--, cb--;
    }
    ca = cb = 0;
    for(int i : n) ca += (i == a), cb += (i == b);
    vi ans;
    int m = n.size() / 3;
    if(ca > m) ans.push_back(a);
    if(cb > m) ans.push_back(b);
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  // vi n = {1,2,3,2,1};
  // vi n = {1,2,3};
  vi n = {0,0,0};
  Solution so;
  vi ans = so.majorityElement(n);
  copy(ans.begin(), ans.end(), ostream_iterator<int>(cout, " ")); cout << endl;
  return 0;
}
