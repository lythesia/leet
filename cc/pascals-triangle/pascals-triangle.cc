#include <cstdio>
#include <cstring>
#include <iostream>
#include <vector>
using namespace std;

class Solution {
public:
  vector<vector<int> > generate(int rows) {
    vector<vector<int> > vvi;
    for(int i=0; i<rows; i++) {
      vector<int> vi;
      for(int j=0; j<=i; j++) {
        vi.push_back(comb(i, j));
      }
      vvi.push_back(vi);
    }
    return vvi;
  }

  int comb(int m, int n) {
    long long res = 1;
    if(n > m - n) n = m - n;
    for(int i=m,cnt=1; cnt<=n; i--,cnt++) {
      res *= i;
    }
    for(int i=1; i<=n; i++) {
      res /= i;
    }
    return static_cast<int>(res);
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  vector<vector<int> > vvi = so.generate(20);
  for(auto vi : vvi) {
    for(auto i : vi) cout << i << ' ';
    cout << endl;
  }
  return 0;
}
