#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Solution {
public:
  string largestNumber(vi &n) {
    vs ns(n.size());
    // transform cannot handles overloaded function!
    transform(n.begin(), n.end(), ns.begin(), [](const int x){return to_string(x);});
    sort(ns.begin(), ns.end(), [](const string &s1, const string &s2){return s1+s2 > s2+s1;});
    string ans = accumulate(ns.begin(), ns.end(), string()); 
    return ans.empty() || ans[0] == '0' ? "0" : ans;
  }
};

int main(int argc, const char *argv[])
{
  // vi v = {3, 30, 34, 5, 9};
  Solution so;
  cout << so.largestNumber(v) << endl;
  return 0;
}
