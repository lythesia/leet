#include <cstdio>
#include <cmath>
#include <iostream>

using namespace std;

#define see(x) cout << #x << ": " << (x) << endl

class Solution {
public:
  double pow(double x, int n) {
    /*double res = x;
    if(n == 0) return 1;
    else if(n == 1) return x;
    int i = 2, ind = i;
    for(; i<=n; i*=2) { // ungraceful, cause last valid `i` maybe << n, this gap is time-consuming by 1-step mult.
      res *= res;
      ind = i;
    }
    for(; ind<n; ind++) res *= x;
    return res;*/
    double res = 1.0;
    bool neg = false;
    if(n < 0) {
      n = -n;
      neg = true;
    }
    while(n > 0) {
      if(!(n % 2)) {
        x *= x;
        n /= 2;
      }
      else {
        --n;
        res *= x;
      }
    }
    return neg ? 1.0/res : res;
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  see(so.pow(0.00001, 2147483647));
  see(so.pow(34.0015, -3));
  //see(::pow(0.00001, 2147483647));
  return 0;
}
