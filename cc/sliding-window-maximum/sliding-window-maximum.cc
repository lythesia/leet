#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

/*
 * monotonic queue:
 * given a0, a1, .. an, any query Q(i, i+L), find min(max) in ai..ai+L
 *
 * keep pair (i, ai), when such ai is to be added to queue, all(j, aj) that aj > ai can never be the min in next
 * queries which covers ai, so they should be deleted.
 * then the front is min we can easily get in O(1), but when should we remove it as queue sliding forward? thus we
 * should keep the info of its position, or say, counter, meaning the # of deleted elems in front of it when add.
 *
 * for n elems, totaly n in/out to the queue, so the two amortized O(1)
 */
class Solution {
public:
  vi maxSlidingWindow(vi &n, int k) {
    vi ans;
    deque<int> dq; // only keep position, not pair (i, ai)
    for(int i=0; i<n.size(); i++) {
      if(!dq.empty() && dq.front() == i-k) dq.pop_front();      // sliding make front out of window range
      while(!dq.empty() && n[dq.back()] < n[i]) dq.pop_back();  // keep monotonic
      dq.push_back(i);
      if(i >= k-1) ans.push_back(n[dq.front()]);
    }
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  vi n = {1,3,-1,-3,5,3,6,7};
  int k = 3;
  Solution so;
  vi ans = so.maxSlidingWindow(n, k);
  copy(ans.begin(), ans.end(), ostream_iterator<int>(cout, " ")); cout << endl;
  return 0;
}
