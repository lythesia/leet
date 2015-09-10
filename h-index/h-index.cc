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
    sort(c.begin(), c.end(), greater<int>());
    int n = c.size();
    for(int i=0; i<n; i++) if(i >= c[i]) return i;
    return n;
  }
};

int main(int argc, const char *argv[])
{
  vi c = {3, 0, 6, 1, 5};
  Solution so;
  cout << so.hIndex(c) << endl;
  return 0;
}
