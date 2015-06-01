#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Solution {
public:
  bool containsNearbyAlmostDuplicate(vi n, int k, int t) {
    if(t < 0) return false;
    map<long, int> tab;
    int len = n.size();
    for(int i=0; i<len; i++) {
      auto low = tab.lower_bound((long)n[i]-t), up = tab.lower_bound((long)n[i]+t+1);
      if((low == tab.end() && up == tab.end()) || low == up) tab[n[i]] = i;
      else {
        cout << n[i] << " :[" << low->first << "," << up->first << ")" << endl;
        for(auto it=low; it!=up; it++) {
          if(abs(it->second - i) <= k) return true;
          else if(it->first == n[i]) tab[n[i]] = i;
        }
      }
    }
    return false;
  }
};

int main(int argc, const char *argv[])
{
  vi v = {0,2147483647};
  int k = 1, t = 2147483647;
  Solution so;
  cout << boolalpha << so.containsNearbyAlmostDuplicate(v, k, t) << endl;
  return 0;
}
