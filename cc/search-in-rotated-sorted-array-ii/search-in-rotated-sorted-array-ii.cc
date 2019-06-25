#include <iostream>
using namespace std;

class Solution {
public:
  bool search(int A[], int n, int target) {
    int l = 0, h = n-1;
    while(l <= h) {
      int mid = (l+h)/2, x = A[mid];
      if(x == target) return true;
      if(x < A[l]) {
        if(x<target && target<=A[h]) l = mid+1;
        else h = mid-1;
      }
      else if(x > A[l]) {
        if(A[l] <= target && target<x) h = mid-1;
        else l = mid+1;
      }
      else if(x == A[l]) {
        if(x == A[h]) ++l,--h; // {1,1,3,1} or {1,3,1,1} cannot be decided by `x==A[h]` so linear
        else l = mid+1;
      }
    }
    return false;
  }
};

int main(int argc, const char *argv[])
{
  int A[] = {1,1,3,1};
  Solution so;
  cout << so.search(A, sizeof(A)/sizeof(int), 3) << endl;
  return 0;
}
