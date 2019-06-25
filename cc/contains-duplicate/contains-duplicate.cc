#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;
typedef unordered_set<int> si;

class Solution {
public:
  bool containsDuplicate(vi &n) {
    si s;
    for(int i : n) {
      if(s.count(i)) return true;
      else s.insert(i);
    }
    return false;
  }
};
