#include <cstdio>
#include <iostream>

using namespace std;

#define see(x) cout << #x << ": " << (x) << endl

class Solution {
public:
  int reverse(int x) {
    bool neg = false;
    if(x < 0) {
      x = -x;
      neg = true;
    }
    int res = 0;
    while(x) {
      res = res*10 + x % 10;
      x /= 10;
    }
    return neg ? -res : res;
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  see(so.reverse(-123));
  return 0;
}
