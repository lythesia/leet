#include <bits/stdc++.h>
using namespace std;

typedef vector<int> vi;
class Solution {
public:
  int canCompleteCircuit(vi &g, vi &c) {
    int n = g.size(), diff = 0, curr = 0, pos = 0, r = 0;
    for(int i=0; i<n; i++) {
      r += g[i] - c[i];
      if(diff <= c[i] - g[i]) diff = c[i] - g[i], pos = i;
    }
    if(r < 0) return -1;
    while(g[pos] < c[pos]) pos = (pos+1) % n;
    for(int i=0; i<n; i++) {
      int p = (i + pos) % n;
      curr += g[p] - c[p];
      if(curr < 0) return -1;
    }
    return curr >= 0 ? pos : -1;
  }
};

int main(int argc, const char *argv[])
{
  // vi g = { 2, 3, 1 },
  //    c = { 3, 1, 2 };
  vi g = {4}, c = {5};
  Solution so;
  cout << so.canCompleteCircuit(g, c) << endl;
  return 0;
}
