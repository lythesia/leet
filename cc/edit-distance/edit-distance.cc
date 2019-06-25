#include <bits/stdc++.h>
using namespace std;

typedef vector<int> vi;
typedef vector<vi> vvi;
/*
 * let e(i,j) the distance of w1[1,i] -> w2[1,j], then:
 * if i==0 && j==0, e(0,0) = 0
 * elif i==0 e(0,j) = j (j insert)
 * elif j==0 e(i,0) = i (i delete)
 * else e(i,j) = {
 *    e(i-1,j-1)  if w1[i] == w2[j]
 *    min(e(i-1,j) + 1,    (i-1 can-to j, so delete last i)
 *        e(i,j-1) + 1,    (i can-to j-1, so insert last j)
 *        e(i-1,j-1) + 1)  (i-1 can-to j-1, so substitute i with j)
 * }
 */
class Solution {
public:
  int minDistance(string w1, string w2) {
    int m = w1.length(), n = w2.length();
    vvi dp(++m, vi(++n));
    for(int i=0; i<m; i++) dp[i][0] = i;
    for(int j=0; j<n; j++) dp[0][j] = j;
    for(int i=1; i<m; i++) for(int j=1; j<n; j++)
      dp[i][j] = w1[i-1] == w2[j-1] ? 
        dp[i-1][j-1] :
        min(min(dp[i-1][j] + 1,   // del
                dp[i][j-1] + 1),  // ins
            dp[i-1][j-1] + 1);    // sub
    return dp[--m][--n];
  }
};

int main(int argc, const char *argv[])
{
  string w1 = "kitten",
         w2 = "sitting";
  Solution so;
  cout << so.minDistance(w1, w2) << endl;
  return 0;
}
