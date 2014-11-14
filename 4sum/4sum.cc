#include <iostream>
#include <vector>
#include <set>
#include <algorithm>
using namespace std;

#define see_vec(v) for(auto __i : (v)) cout << __i << ' '; cout << endl
typedef vector<int> vi;
typedef vector<vi> vvi;

class Solution {
public:
  vvi fourSum(vi &num, int target) {
    if(num.size() < 4) return vvi();
    sort(num.begin(), num.end());
    return ksum(num, 0, 4, target);
  }

  /* assume `ns` sorted */
  vvi ksum(vi &ns, int s, int k, int sum) {
    vvi res;
    vi tuple;
    set<int> visited;
    if(k == 2) {
      int l = s, r = ns.size() - 1;
      while(l < r) {
        int _s = ns[l] + ns[r];
        if(_s == sum && visited.find(ns[l]) == visited.end() { // `_s == sum` put latter will TLE
          tuple.clear();
          visited.insert(ns[l]);
          visited.insert(ns[r]);
          tuple.push_back(ns[l]);
          tuple.push_back(ns[r]);
          res.push_back(tuple);
          ++l, --r;
        }
        else if(_s > sum) --r;
        else ++l;
      }
    }
    else {
      for(int i=s; i<ns.size(); i++) {
        if(visited.find(ns[i]) == visited.end()) {
          visited.insert(ns[i]);
          vvi sub = ksum(ns, i+1, k-1, sum-ns[i]);
          if(!sub.empty()) {
            for(vi &j : sub) j.insert(j.begin(), ns[i]);
            res.insert(res.end(), sub.begin(), sub.end());
          }
        }
      }
    }
    return res;
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  vi num = {1,0,-1,0,-2,2};
  vvi res = so.fourSum(num, 0);
  for(auto _vi : res) {
    printf("( ");
    for(auto i: _vi) printf("%d ", i); puts(")");
  }
  return 0;
}
