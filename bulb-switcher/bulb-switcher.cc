#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Solution {
public:
  int bulbSwitch(int n) {
    return sqrt(n);
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  cout << so.bulbSwitch(6) << endl;
  return 0;
}
