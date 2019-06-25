#include <bits/stdc++.h>
using namespace std;

typedef unordered_map<int, bool> mib;
typedef unordered_set<int> si;
class Solution {
public:
  int gao(int n) {
    int ans = 0;
#define sq(x) (x)*(x)
    while(n) {
      ans += sq(n % 10);
      n /= 10;
    }
#undef sq
    return ans;
  }
  bool isHappy(int n) {
    si tab;
    while(true) {
      n = gao(n);
      if(n == 1) return true;
      if(tab.find(n) != tab.end()) return false;
      tab.insert(n);
    }
    return false;
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  cout << boolalpha << so.isHappy(19) << endl;
  return 0;
}
