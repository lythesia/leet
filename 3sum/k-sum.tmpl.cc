#include <iostream>
#include <vector>
#include <set>
#include <algorithm>
using namespace std;

#define see_vec(v) for(auto __i : (v)) cout << __i << ' '; cout << endl
typedef vector<int> vi;
typedef vector<vi> vvi;

/* assume `ns` sorted */
vvi ksum(vi &ns, int s, int k, int sum) {
  vvi res;
  vi tuple;
  set<int> visited;
  if(k == 2) {
    int l = s, r = ns.size() - 1;
    while(l < r) {
      int _s = ns[l] + ns[r];
      if(_s == sum && visited.find(ns[l]) == visited.end()) {
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
    for(int i=s; i<ns.size(); i++) { // to last(ns.size()-1) to avoid []
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

int main(int argc, const char *argv[])
{
  
  int x;
  vi num;
  while(scanf("%d", &x) != EOF) num.push_back(x);
  sort(num.begin(), num.end());
  see_vec(num);
  vvi res = ksum(num, 0, 3, 0);
  cout << res.size() << endl;
  for(auto _vi : res) {
    printf("( ");
    for(auto i: _vi) printf("%d ", i); puts(")");
  }
  return 0;
}
