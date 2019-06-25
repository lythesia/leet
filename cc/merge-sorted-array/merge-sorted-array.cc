#include <cstdio>
#include <iostream>
#include <algorithm>

using namespace std;

class Solution {
public:
  // asc assumed
  void merge(int A[], int m, int B[], int n) {
    int i = m-1, j = n-1, k = m+n-1;
    while(i>=0 && j>=0) A[k--] = A[i] > B[j] ? A[i--] : B[j--];
    while(i>=0) A[k--] = A[i--];
    while(j>=0) A[k--] = B[j--];
  }
};

int main(int argc, const char *argv[])
{
  int A[] = {1},
      B[] = {};
  Solution so;
  so.merge(A, 1, B, 0);
  for(auto i : A) printf("%d ", i); puts("");
  return 0;
}
