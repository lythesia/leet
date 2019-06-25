#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Solution {
public:
  int minSubArrayLen(int s, vi &n) {
    int p = 0, len = n.size(), ans = INT_MAX, sum = 0; 
    for(int i=0; i<len; i++) {
      if(sum + n[i] < s) sum += n[i];
      else {
        while(sum + n[i] >= s) sum -= n[p++];
        p--, sum += n[p] + n[i], ans = min(ans, i-p+1);
      }
    }
    return ans == INT_MAX ? 0 : ans;
  }
};

int main(int argc, const char *argv[])
{
  vi v = {2,3,1,2,4,3};
  int s = 7;
  // vi v = {1,2,3,4,5};
  // int s = 11;
  Solution so;
  cout << so.minSubArrayLen(s, v) << endl;
  return 0;
}
