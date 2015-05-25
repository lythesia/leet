#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Solution {
public:
  int digits(int x) {
    int ans = 1;
    while(x) x /= 10, ans++; 
    return ans;
  }
  void buck_sort(vi &n) {
    vi buck[10], nn;
    nn.reserve(n.size());
    int mmax = *max_element(n.begin(), n.end()), maxd = digits(mmax);
    for(int d=0; d<maxd; d++) {
      int div = (int)pow(10, d);
      // bucket
      for(int x : n) buck[(x/div) % 10].push_back(x);
      // copy for next round
      for(vi &v : buck) {
        move(v.begin(), v.end(), back_inserter(nn));
        v.clear();
      }
      n.swap(nn); nn.clear();
    }
  }

  int maximumGap(vi &n) {
    int len = n.size(), ans = 0;
    if(len < 2) return ans;
    buck_sort(n);
    int p = n[0];
    for(size_t i=1; i<n.size(); i++) ans = max(ans, n[i]-p), p = n[i];
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  vi v = {1,1,1,1,1,5,5,5,5,5};
  Solution so;
  cout << so.maximumGap(v) << endl;
  return 0;
}
