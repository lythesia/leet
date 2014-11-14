#include <cstdio>
#include <iostream>
#include <algorithm>
using namespace std;

class Solution {
public:
  int removeDuplicates(int A[], int n) {
    if(!n) return n;

    int dup = 0;
    int i = 0, p = 0;
    while(++i != n) {
      if(A[i] == A[p]) {
        if(!dup) A[++p] = A[i];
        ++dup;
      }
      else dup = 0, A[++p] = A[i];
    }
    return ++p;
  }
};

int main(int argc, const char *argv[])
{
  int A[] = {1,1,1,1,1,1};
  Solution so;
  int n = so.removeDuplicates(A, sizeof(A)/sizeof(int));
  for(int i=0; i<n; i++) cout << A[i] << ' '; cout << endl;
  return 0;
}
