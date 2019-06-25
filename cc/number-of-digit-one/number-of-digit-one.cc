#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

/*
 * key: count `1`'s # occurance on each digit
 *
 * example:
 * n = 3141592, with i = 100, means now we want count on hundred-bit
 * then a = n/i, b = n%i, we have:
 * a = 31415, b = 92, task is to count xxxx1oo, where # of the hundred-bit 1:
 * when right part `1oo` fixed, we have "".to("3141") totally 3142 such `1oo`, and with "".to("99") 100 in total
 * of suffix `1(oo)`, hence sums up to (a/10 + 1) * i such `xxxx1oo`
 *
 * what if last bit is just 1? take i = 1000, we have:
 * a = 3141, b = 592, task: xxx1ooo, for thousand-bit 1:
 * similar we have "".to("314"), say 315 such pattern, note that the last bit of a is 1, so ""1ooo has just ooo
 * # of that thousand-bit 1 occurance, so we borrow it from a, (a/10) * i + (b + 1)
 *
 * if just 0, simply substract it from (a/10 + 1), aka a/10, and no more need for b part
 *
 * we can use (n/i + 8)/10 to determine if we need adjust for b part:
 * if n/i%10 < 2, (a + 8)/10 == a/10
 * else (a + 8)/10 == a/10 + 1
 *
 * and use a % 10 == 1 to indicate b part
 */
class Solution {
public:
  int countDigitOne(int n) {
    int ans = 0;
    for(long i=1; i<=n; i*=10) ans += (n/i + 8) / 10 * i + (n/i % 10 == 1) * (n%i + 1);
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  //
  return 0;
}
