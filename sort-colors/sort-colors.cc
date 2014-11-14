#include <cstdio>
#include <iostream>
#include <algorithm>
using namespace std;

class Solution {
public:
  void sortColors(int A[], int n) {
    int s = 0, e = 0;
    for(int i=0; i<n-e; i++) {
      if(A[i] == 0) {
        swap(A[i], A[s++]);
      }
      else if(A[i] == 2) {
        swap(A[i], A[n-1-(e++)]);
        --i;
      }
    }
  }
};

int main(int argc, const char *argv[])
{
  int A[] = { 2,1,0,1,1,2,2,0,0,1,0,2,2,0,1,0 };
  Solution so;
  so.sortColors(A, sizeof(A)/sizeof(int));
  for(int i : A) cout << i << ' '; cout << endl;
  return 0;
}
