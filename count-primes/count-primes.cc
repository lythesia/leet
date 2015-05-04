#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
  int countPrimes(int n) {
    vector<bool> tab(n, true);
    tab[1] = false;
    int nn = (int)sqrt(n);
    for(int i=2; i<=nn; i++) {
      if(tab[i]) {
        for(int j=i*i; j<n; j+=i) tab[j] = false;
      }
    }
    int ans = 0;
    for(int i=1; i<n; i++) if(tab[i]) ans++;
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  int n = 25; // 9
  Solution so;
  cout << so.countPrimes(n) << endl;
  return 0;
}
