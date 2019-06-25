#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Solution {
public:
  string fractionToDecimal(int num, int den) {
    string ans;
    if((num^den) < 0 && num) ans.push_back('-');
    long numer = abs(long(num)), denom = abs(long(den));
    if(numer > denom) ans += to_string(numer/denom), numer %= denom;
    else ans.push_back('0');

    if(!numer) return ans;
    ans.push_back('.');

    unordered_map<long, int> qs;
    long q = 0;
    int p = ans.length() - 1;
    qs[numer] = ++p;
    while(true) {
      if(numer < denom) numer *= 10;
      q = numer / denom, numer %= denom;
      ans.push_back(q + '0');
      if(!numer) return ans; // no circle
      if(qs.count(numer)) break;
      else qs[numer] = ++p;
    }
    if(numer) {
      ans.insert(qs[numer], 1, '(');
      ans.push_back(')');
    }
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  int n = 1, d = 333;
  // int n = -2147483648, d = 1;
  // int n = 10, d = 23;
  Solution so;
  cout << so.fractionToDecimal(n, d) << endl;
  return 0;
}
