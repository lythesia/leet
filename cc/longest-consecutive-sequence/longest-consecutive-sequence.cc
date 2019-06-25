#include <cstdio>
#include <iostream>
#include <vector>
#include <unordered_map>
#include <algorithm>
using namespace std;

typedef vector<int> vi;
class Solution {
public:
  /* bidirect 
   * end[1] --> 5 and end[5] --> 1 indicates 1..5 consecutive
   * for any A[i]:
   * 1. if end[i-1]: 
   *   if end[i-1] <= i-1: end[i] = end[i-1], end[end[i]] = i keeps the rule (end[i-1] ... i-1, i)
   *   else: not possible, if i .. end[i-1] is consecutive, then i must be already included in
   *
   * 2. if end[i+1]:
   *   if end[i+1] >= i+1: end[end[i+1]] = end[i], end[end[i]] = end[i+1]  (end[i] ... i, i+1 ... end[i+1])
   *   else: similar not possible
   */
  int longestConsecutive(vi &num) {
    unordered_map<int, int> end;
    for(int i : num) {
      if(end.count(i)) continue;

      end[i] = i;
      if(end.count(i-1)) {
        end[i] = end[i-1];
        end[end[i]] = i; // end[end[i-1]] = i;
      }
      if(end.count(i+1)) {
        int last = end[i+1];
        end[last] = end[i];
        end[end[i]] = last;
      }
    }
    int res = 0;
    for(auto p : end) {
      res = max(res, p.second-p.first+1);
    }
    return res;
  }
};

int main(int argc, const char *argv[])
{
  vi n = {100,4,200,1,3,2};
  Solution so;
  cout << so.longestConsecutive(n) << endl;
  return 0;
}
