#include <cstdio>
#include <cstring>
#include <iostream>
#include <vector>
using namespace std;

class Solution {
public:
  vector<int> getRow(int row) {
    vector<int> vi;
    for(int i=0; i<=row; i++) vi.push_back(comb(row, i));
    return vi;
  }

  int comb(int m, int n) {
    long double res = 1;
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
  vector<int> vi = so.getRow(3);
  for(auto i : vi) cout << i << ' ';
  cout << endl;
  return 0;
}
