#include <cstdio>
#include <cstdlib>
#include <climits>
#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

typedef vector<int> vi;

class Solution {
public:
  int threeSumClosest(vi &num, int target) {
    int res, mmin = INT_MAX;
    sort(num.begin(), num.end());
    for(int i=0; i<num.size()-2; i++) {
      int x = num[i], l = i+1, r = num.size()-1;
      while(l < r) {
        int s = x + num[l] + num[r];
        if(s == target) return s;
        else {
          if(abs(s-target) < mmin) res = s, mmin = abs(s-target);
          if(s > target) --r;
          else ++l;
        }
      }
    }
    return res;
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  vi num = {-1, 2, 1, -4};
  cout << so.threeSumClosest(num, 1) << endl;
  return 0;
}
