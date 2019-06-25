#include <bits/stdc++.h>
using namespace std;

typedef vector<int> vi;
typedef vector<vi> vvi;
/*
 * given a comb: [a1, a2, .. ak], checks if ak can swap with larger number ak+1..an
 * if not, find reversely for 1st ai that ai != i, ++ai, and make ai+1, ai+2 ... afterwards
 * if no such ai, all combinations found
 */
class Solution {
public:
  vvi combine(int n, int k) {
    vvi ans;
    if(k > n || !k) return ans;
    vi comb(k+1);
    for(int i=1; i<=k; i++) comb[i] = i;
    while(true) {
      for(int j=comb[k]; j<=n; j++) {
        comb[k] = j;
        vi t(comb.begin()+1, comb.begin()+k+1);
        ans.push_back(t);
      }
      int c = 1;
      while(k-c && comb[k-c] == n-c) c++;
      if(c == k) break;
      comb[k-c]++;
      for(int i=k-c+1; i<=k; i++) comb[i] = comb[i-1] + 1;
    }
    return ans;
  }
};

void pvec(const vi &v) {
  for(int i : v) cout << i << ' '; cout << endl;
}

int main(int argc, const char *argv[])
{
  Solution so;
  vvi ans = so.combine(4, 0);
  for(auto &v : ans) pvec(v);
  return 0;
}
