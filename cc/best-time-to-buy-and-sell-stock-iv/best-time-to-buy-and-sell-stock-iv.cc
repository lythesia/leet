#include <bits/stdc++.h>
using namespace std;

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef pair<int,int> pii;
class Solution {
public:
  // TLE on 1000000000 * 10000
  // dp[t][i]: maximum profit using at most t transactions up to day i (including day i)
  // dp[t][i] = max(dp[t][i - 1], prices[i] - prices[j] + dp[t - 1][j]) for all j in range [0, i - 1]
  // int maxProfit(int K, vi &p) {
  //   int n = p.size();
  //   if(n < 2) return 0;
  //   vvi dp(K+1, vi(n, 0));
  //   int ans = 0;
  //   for(int k=1; k<=K; k++) {
  //     int take_day_i_mmax = dp[k-1][0] - p[0];
  //     for(int i=1; i<n; i++) {
  //       dp[k][i] = max(
  //           dp[k][i-1],             // not take day i
  //           p[i] + take_day_i_mmax  // take day i
  //       ),
  //       take_day_i_mmax = max(take_day_i_mmax, dp[k-1][i] - p[i]), // since we dp from i: 1->n-1 for each k, we can keep along the mmax during 1->i
  //       ans = max(dp[k][i], ans);   // since dp[k][n-1] >= dp[k][i] (i <= n-1), final ans is among dp[1..k][n01]
  //     }
  //   }
  //   return ans;
  // }

  // recall series-ii
  // we need at most lower buy and at most higher sell for each transaction, and we need top K such ones.
  int maxProfit(int K, vi &p) {
    int n = p.size();
    if(n < 2) return 0;
    int ans = 0, buy = 0, sell = 0;
    vi profit;
    stack<pii> tran; // transaction pairs
    while(sell < n) {
      /*          [x] <- sell
       * x       x   x
       *   x   x       ...
       *    [x] <- buy
       * day: ------>
       */
      for(buy = sell; buy<n-1 && p[buy] >= p[buy+1]; buy++);      // find lower buy point
      for(sell = buy+1; sell<n && p[sell] >= p[sell-1]; sell++);  // find higher sell point(consectively)
      // find one pair (buy, sell)
      // b1 < s1, b2 < s2
      // 1. if b1 <= b2:
      //    1. if s1 <= b2: we should take as 1 transaction: s2 - b1 (since b1 --> s2 can merge)
      //    2. else: we should take as 2s: s1-b1 + s2-b2 = s2-b1 + s1-b2 > s2-b1
      // 2. else: take b1,s1 as 1 transaction and keep it
      while(!tran.empty() && p[buy] < p[tran.top().first]) {
        auto &pp = tran.top();
        profit.push_back(p[pp.second-1] - p[pp.first]);
        tran.pop();
      }
      // b1 <= b2 falls here, thus we have: b1 <= b2 <= s2, where b2 <=> s1, s1 <=> s2
      // the below constraints s1 <= s2, so we have: b1 <= b2 <= s2 and b1 <= s1 <= s2, where s1 <=> b2
      // now we save s1-b2 no matter it >=0 or <0, and set current buy to b1, sell to s2+1
      // since for b1 (s1 b2) s2 ... S, given S > s2, pair (b1, S) is always took, we can consider S as new s2
      // whether s1-b2 will be among top k is not determined, but S-b1 is always larger
      while(!tran.empty() && p[sell-1] >= p[tran.top().second-1]) {
        auto &pp = tran.top();
        profit.push_back(p[pp.second-1] - p[buy]);
        buy = pp.first;
        tran.pop();
      }
      tran.push(pii(buy, sell));
    }
    // record rest pairs
    while(!tran.empty()) {
      auto &pp = tran.top();
      profit.push_back(p[pp.second-1] - p[pp.first]);
      tran.pop();
    }
    make_heap(profit.begin(), profit.end()); // O(N)
    for(int i=0; i<K && !profit.empty(); i++) {
      // pop max to end
      pop_heap(profit.begin(), profit.end());
      ans += profit.back();
      profit.pop_back();
    }
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  vi v;
  int n = 0;
  scanf("%d", &n);
  int x = 0;
  while(scanf("%d", &x) != EOF) {
    v.push_back(x);
  };
  printf("n: %d, size: %zu\n", n, v.size());
  Solution so;
  // in: 1648961
  cout << so.maxProfit(n, v) << endl;
  return 0;
}
