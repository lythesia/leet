#include <bits/stdc++.h>
using namespace std;

typedef vector<int> vi;
class Solution {
public:
  double findMedianSortedArrays(vi &n1, vi &n2) {
    int n = n1.size() + n2.size();
    if(n & 1) return findKth(n1, 0, n2, 0, n/2 + 1);
    else return (findKth(n1, 0, n2, 0, n/2) + findKth(n1, 0, n2, 0, n/2+1)) / 2.0;
  }

  int findKth(vi &n1, int s1, vi &n2, int s2, int k) {
    // ensure A.len < B.len
    if(n1.size() - s1 > n2.size() - s2) return findKth(n2, s2, n1, s1, k);

    // if A or B is empty
    if(!(n1.size() - s1)) return n2[s2+k-1];

    if(k == 1) return min(n1[s1], n2[s2]);

    int k1 = min(k/2, (int)n1.size() - s1),
        k2 = k - k1;
    if(n1[s1+k1-1] == n2[s2+k2-1]) return n1[s1+k1-1];
    else if(n1[s1+k1-1] < n2[s2+k2-1]) return findKth(n1, s1+k1, n2, s2, k-k1);
    else return findKth(n1, s1, n2, s2+k2, k-k2);
  }
};

int main(int argc, const char *argv[])
{
  vi n1 = {1,2},
     n2 = {1,2};
  Solution so;
  cout << so.findKth(n1, 0, n2, 0, 2) << " " << so.findKth(n1, 0, n2, 0, 3) << endl;
  cout << so.findMedianSortedArrays(n1, n2) << endl;
  return 0;
}
