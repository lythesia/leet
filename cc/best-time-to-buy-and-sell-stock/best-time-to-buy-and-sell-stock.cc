#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
  int maxProfit(vector<int> &prices) {
    int n = prices.size();
    if(n < 2) return 0;
    vector<int> sell(n);
    int mmax = INT_MIN, ans = 0;
    for(int i=n-1; i>=0; i--) mmax = max(mmax, prices[i]), ans = max(ans, mmax - prices[i]);
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  vector<int> p = { 2, 1 };
  Solution so;
  cout << so.maxProfit(p) << endl;
  return 0;
}
