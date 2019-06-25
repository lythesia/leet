#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
  /* 
   * since a[-1] < a[0], we only need to find 1st a[i] that a[i] > a[i+1]
   */
  int findPeakElement(const vector<int> &n) {
    int len = n.size(), l = 0, h = len - 1;
    if(len <= 1) return 0;
    while(l < h) {
      int mid = (l + h) / 2; // mid !== h, since l < h
      if (n[mid] > n[mid + 1]) h = mid;
      else if(n[mid] < n[mid + 1]) l = mid + 1;
    }
    return l;
  }
};
