#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

const int p[] = {2,3,5};
class Solution {
public:
  bool isUgly(int n) {
    if(n <= 0) return false;
    for(int i=0; i<3; i++) while(!(n % p[i])) n /= p[i];
    return n == 1;
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  cout << boolalpha << so.isUgly(6) << endl;
  cout << boolalpha << so.isUgly(8) << endl;
  cout << boolalpha << so.isUgly(14) << endl;
  return 0;
}
