#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
  string convertToTitle(int n) {
    string ans;
    while(n) {
      int q = n/26, r = n%26;
      if(!r) ans += 'Z', n = q - 1;
      else ans += 'A' + r - 1, n = q;
    }
    return string(ans.rbegin(), ans.rend());
  }
};

int main(int argc, const char *argv[])
{
  // int n = 702;  // ZZ
  int n = 703;  // AAA
  Solution so;
  cout << so.convertToTitle(n) << endl;
  return 0;
}
