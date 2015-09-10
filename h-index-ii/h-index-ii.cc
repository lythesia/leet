#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Solution {
public:
  int hIndex(vi &c) {
    int n = c.size(), s = 0, e = n - 1;
    while(s <= e) {
      int m = (s + e) / 2;
      if(c[m] > n - m) e = m - 1;
      else if(c[m] < n - m) s = m + 1;
      else return n - m;
    }
    return n - s;
  }
};

int main(int argc, const char *argv[])
{
  vi c = {0,1,3,5,6};
  Solution so;
  cout << so.hIndex(c) << endl;
  return 0;
}
