#include <cstdio>
#include <iostream>
#include <algorithm>
using namespace std;

class Solution {
public:
  int searchInsert(int A[], int n, int target) {
    if(!n) return n;
    int l = 0, h = n;
    while(l < h) {
      int mid = (l+h)/2;
      if(mid==h || mid==l) break;
      if(target < A[mid]) h = mid;
      else if(target > A[mid]) l = mid;
      else return mid;
    }
    int res = min(l, h);
    return A[res] >= target ? res : res+1;
  }
};

int main(int argc, const char *argv[])
{
  int A[] = {1};
  Solution so;
  cout << so.searchInsert(A, sizeof(A)/sizeof(int), 2) << endl;
  return 0;
}
