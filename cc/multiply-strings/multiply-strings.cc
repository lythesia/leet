#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
  string multiply(string n1, string n2) {
    if(n1.length() > 1) assert(n1[0] != '0');
    if(n2.length() > 1) assert(n2[0] != '0');

    string ans;
    int c = 0, p = 0;
    vector<int> pv(n1.size()+n2.size(), 0);
    for(auto i=n2.rbegin(); i!=n2.rend(); i++) {
      int lsd = i-n2.rbegin();
      for(auto j=n1.rbegin(); j!=n1.rend(); j++) p = (*i-'0') * (*j-'0') + c, pv[lsd] += p, c = pv[lsd] / 10, pv[lsd++] %= 10;
      pv[lsd] += c, c = 0;
    }
    pv.erase(find_if_not(pv.rbegin(), pv.rend(), [](int x) {return x == 0;}).base(), pv.end());
    transform(pv.rbegin(), pv.rend(), back_inserter(ans), [](int x) {return x+'0';});
    return ans.empty() ? "0" : ans;
  }
};

int main(int argc, const char *argv[])
{
  string n1 = "93553535314",
         n2 = "25247452591474"; // 2361988447605003674312836
  // string n1 = "0", n2 = "0";
  Solution so;
  cout << so.multiply(n1, n2) << endl;
  return 0;
}
