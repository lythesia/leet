#include <bits/stdc++.h>
using namespace std;

typedef vector<int> vi;
class Solution {
public:
  inline int siden(int n) { return n*(n+1)/2; }
  int candy(vi &r) {
    if(r.size() < 2) return r.size();
    int len = r.size(), ans = 0, lend = 0, lena = 0, prev = 0; // prev: asc or desc or stable
    for(int i=1; i<len; i++) {
      // current asc or desc or stable
      // we count stable as end of side
      int curr = r[i] > r[i-1] ? 1 :
                 r[i] < r[i-1] ? -1: 0;
      if((prev>0 && !curr) ||   // one side ends, but i is not peak
         (prev<0 && curr>=0)) {
        ans += siden(lena) +    // asc sum
               siden(lend) +    // desc sum
               max(lena, lend); // peak: actually max(..)+1, but the end of current mountain is start of next one, we count this end(1) to next
        lena = lend = 0;
      }
      if(curr > 0) lena++;
      else if(curr < 0) lend++;
      else ans++; // if next goes desc, then next's mountain will count this "peak"; else goes asc, it will be count into next's peak; else stable keep 1
      prev = curr;
    }
    return ans += siden(lena) + siden(lend) + max(lena, lend) + 1;
  }
};

int main(int argc, const char *argv[])
{
  vi rat = {
    58, 21, 72, 77, 48, 9, 38, 71, 68, 77, 82, 47, 25, 94, 89, 54, 26, 54, 54,
    99, 64, 71, 76, 63, 81, 82, 60, 64, 29, 51, 87, 87, 72, 12, 16, 20, 21, 54,
    43, 41, 83, 77, 41, 61, 72, 82, 15, 50, 36, 69, 49, 53, 92, 77, 16, 73, 12,
    28, 37, 41, 79, 25, 80, 3, 37, 48, 23, 10, 55, 19, 51, 38, 96, 92, 99, 68,
    75, 14, 18, 63, 35, 19, 68, 28, 49, 36, 53, 61, 64, 91, 2, 43, 68, 34, 46,
    57, 82, 22, 67, 89
  }; // 208
  Solution so;
  cout << so.candy(rat) << endl;
  return 0;
}
