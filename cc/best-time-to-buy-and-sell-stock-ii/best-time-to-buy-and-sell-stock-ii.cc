#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
  int maxProfit(vector<int> &p) {
    int n = p.size();
    if(n < 2) return 0;
    int ans = 0, cur = p[0], bors = -1;
    for(int i=1; i<n; i++) {
      if(bors == -1) {
        if(p[i] <= cur) {
          cur = p[i];
          continue;
        }
        else {
          bors = cur;
          cur = p[i];
        }
      }
      else {
        if(p[i] >= cur) {
          cur = p[i];
          continue;
        }
        else {
          ans += cur - bors;
          bors = -1;
          cur = p[i];
        }
      }
    }
    if(bors != -1) ans += p[n-1] > bors ? p[n-1] - bors : 0;
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  vector<int> p = { 2, 1, 2, 0, 1 };
  Solution so;
  cout << so.maxProfit(p) << endl;
  return 0;
}
