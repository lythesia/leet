#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Solution {
public:
  int missingNumber(vi n) {
    return (n.size() + 1) * n.size() / 2 - accumulate(n.begin(), n.end(), 0, [](int acc, int x){return acc + x;});
  }
};

int main(int argc, const char *argv[])
{
  //
  return 0;
}
