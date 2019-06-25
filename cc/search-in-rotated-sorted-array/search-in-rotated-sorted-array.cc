#include <iostream>
using namespace std;

class Solution {
public:
  int search(int A[], int n, int target) {
    int l = 0, h = n-1;
    while(l <= h) {
      int mid = (l+h) / 2, x = A[mid];
      if(x == target) return mid;
      if(x < A[l]) {  // e.g |6 . .|. . . . . 3|
        if(x <= target && target <= A[h]) l = mid+1;
        else h = mid-1;
      }
      else {          // e.g |. . . . . 3|. . 6|
        if(A[l] <= target && target <= A[mid]) h = mid-1;
        else l = mid+1;
      }
    }
    return -1;
  }
};

int main(int argc, const char *argv[])
{
  int A[] = {4,5,6,7,0,1,2};
  Solution so;
  cout << so.search(A, sizeof(A)/sizeof(int), 2) << endl;
  return 0;
}
