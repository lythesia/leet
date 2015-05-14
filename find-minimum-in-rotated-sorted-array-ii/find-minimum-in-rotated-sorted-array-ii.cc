#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
  int findMin(vector<int> &n) {
    int len = n.size(), l = 0, h = len - 1;
    while(l < h) {
      int m = (l + h) / 2;
      if(n[m] < n[l]) h = m;
      else if(n[m] > n[h]) l = m + 1;
      else h--;
    }
    return n[l];
  }
};
