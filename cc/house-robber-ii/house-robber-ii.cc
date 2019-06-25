#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Solution {
public:
  // int rob(vi &n) {
  //   int last[2] = { 0 }, nolast[2] = { 0 };
  //   for(size_t i=1; i<=n.size(); i++) {
  //     int t0 = nolast[0], t1 = nolast[1];
  //     nolast[0] = max(nolast[0], last[0]);
  //     nolast[1] = max(nolast[1], last[1]);
  //     if(i == 1) last[0] = 0, last[1] = t1 + n[i-1];
  //     else if(i == n.size()) last[0] = t0 + n[i-1], last[1] = t1;
  //     else last[0] = t0 + n[i-1], last[1] = t1 + n[i-1];
  //   }
  //   return max(max(nolast[0], nolast[1]), max(last[0], last[1]));
  // }

  // another simple
  // max (
  //  rob from 0 to n-2,
  //  rob from 1 to n-1
  // );
  int rb(vi &n, size_t s, size_t e) {
    int last = 0, nolast = 0, t = 0;
    for(size_t i=s+1; i<=e; i++) t = nolast, nolast = max(nolast, last), last = t + n[i-1];
    return max(nolast, last);
  }

  int rob(vi &n) {
    if(n.empty()) return 0;
    if(n.size() < 2) return n[0];
    return max(rb(n, 0, n.size()-1), rb(n, 1, n.size()));
  }
};

int main(int argc, const char *argv[])
{
  vi v =  {2,1};
  Solution so;
  cout << so.rob(v) << endl;
  return 0;
}
