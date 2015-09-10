#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Solution {
public:
  int firstBadVersion(int n) {
    int s = 1, e = n;
    while(s <= e) {
      int m = s + (e - s) / 2;
      if(isBadVersion(m)) e = m - 1;
      else s = m + 1;
    }
    return s;
  }
};
