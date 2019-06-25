#include <cstdio>
#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

typedef vector<int> vi;
class Solution {
public:
  /* dp
   * left[i]:    left-most idx of consecutive heights >= height[i] who are left to i
   * right[i]:   right-most idx of consecutive heights >= height[i] who are right to i
   * e.g: 4 3 7 6 5 7 7 8 0 9
   *          |   i     |
   *          L         R
   */
  int largestRectangleArea(vi &height) {
    int n = height.size();
    if(!n) return 0;
    vi left(n, 0), right(n, 0);
    left[0] = 0, right[n-1] = n-1;
    for(int i=1; i<n; i++) {
      left[i] = i;
      while(left[i]-1 >= 0 && height[i] <= height[left[i]-1]) left[i] = left[left[i] - 1];
    }
    for(int i=n-2; i>=0; i--) {
      right[i] = i;
      while(right[i]+1 < n && height[i] <= height[right[i]+1]) right[i] = right[right[i] + 1];
    }
    int res = 0;
    for(int i=0; i<n; i++) res = max(res, (right[i] - left[i] + 1)*height[i]);
    return res;
  }
};

int main(int argc, const char *argv[])
{
  vi h = {2,1,5,6,2,3};
  Solution so;
  cout << so.largestRectangleArea(h) << endl;
  return 0;
}
