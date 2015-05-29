#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Solution {
public:
  bool containsNearbyDuplicate(vi &n, int k) {
    unordered_map<int, int> t;
    int len = n.size();
    for(int i=0; i<len; i++) {
      if(t.count(n[i]) && i-t[n[i]] <= k) return true;
      else t[n[i]] = i;
    }
    return false;
  }
};

int main(int argc, const char *argv[])
{
  
  return 0;
}
