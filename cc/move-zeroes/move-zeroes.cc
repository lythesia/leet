#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Solution {
public:
  void moveZeroes(vi &n) {
    auto i = find(n.begin(), n.end(), 0);
    for(auto j=i+1; i!=n.end() && j!=n.end(); j++) if(*j) *i++ = *j;
    fill(i, n.end(), 0);
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  vi n = {0, 1, 0, 3, 12};
  so.moveZeroes(n);
  copy(n.begin(), n.end(), ostream_iterator<int>(cout, " ")); cout << endl;
  return 0;
}
