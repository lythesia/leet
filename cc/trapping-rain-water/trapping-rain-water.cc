#include <cstdio>
#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

class Solution {
public:
  /*
  int trap(int A[], int n) {
    if(n < 3) return 0;
    int a = 0, b = n-1, leftmax = 0, rightmax = 0, res = 0;
    while(a < b) {
      leftmax = max(leftmax, A[a]);
      rightmax = max(rightmax, A[b]);
      if(leftmax < rightmax) res += (leftmax-A[a]), ++a;
      else res += (rightmax-A[b]), --b;
    }
    return res;
  }
  */
  int trap(int A[], int n) {
    if(n < 3) return 0;
    vector<int> left(n, 0), right(n, 0);
    left[0] = A[0];
    int mmax = A[0];
    for(int i=1; i<n-1; i++) mmax = max(A[i], mmax), left[i] = mmax;
    right[n-1] = A[n-1];
    mmax = A[n-1];
    for(int i=n-2; i>0; i--) mmax = max(A[i], mmax), right[i] = mmax;
    int res = 0;
    for(int i=1; i<n-1; i++) {
      int x = min(left[i], right[i]) - A[i];
      if(x > 0) res += x;
    }
    return res;
  }
};

int main(int argc, const char *argv[])
{
  int A[] = {2,0,2};
  Solution so;
  cout << so.trap(A, sizeof(A)/sizeof(int)) << endl;
  return 0;

}
