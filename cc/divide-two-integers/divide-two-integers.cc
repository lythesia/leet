#include <cstdio>
#include <cstdlib>
#include <iostream>

using namespace std;

#define see(x) cout << #x << ": " << (x) << endl

typedef long long ll;

class Solution {
public:
  int divide(int dividend, int divisor) {
    ll res = 0, d = abs((ll)divisor), n = abs((ll)dividend);

    if(d > n) return 0;
    int di = 0;
    while(d <= n) (d <<= 1), ++di; 
    while(di >= 0) {
      if(d <= n) res += (1<<di), n -= d;
      --di, (d >>= 1);
    }

    return (divisor ^ dividend) < 0 ? -res : res;
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  int a, b;
  while(scanf("%d%d", &a, &b) != EOF) printf("%d / %d = %d\n", a, b, so.divide(a, b));
  return 0;
}
