#include <bits/stdc++.h>
using namespace std;

typedef vector<int> vi;
class Solution {
public:
  int maxProfit(vi &p) {
    int n = p.size();
    if(n < 2) return 0;
    /* buy/sell are money we have after we buy/sell(1,2) */
    int buy1 = INT_MIN, buy2 = INT_MIN,
        sell1 = 0, sell2 = 0;
    for(int v : p)
      sell2 = max(sell2, buy2 + v),
      buy2  = max(buy2, sell1 - v), // -v so init as MIN
      sell1 = max(sell1, buy1 + v),
      buy1  = max(buy1, 0 - v);     // -v so init as MIN
    return sell2; // since sell2 always >= sell1
  }
};

int main(int argc, const char *argv[])
{
  vi v = {2,1,2,0,1};
  Solution so;
  cout << so.maxProfit(v) << endl;
  return 0;
}
