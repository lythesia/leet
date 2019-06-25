#include <cstdio>
#include <iostream>

using namespace std;

class Solution {
public:
  int removeElement(int A[], int n, int elem) {
    int res = 0;
    for(int i=0; i<n-res; i++) {
      if(A[i] == elem) {
        ++res;
        A[i--] = A[n-res];
      }
    }
    return n-res;
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  int A[] = {1};
  int len = so.removeElement(A, 1, 1);
  for(int i=0; i<len; i++) printf("%d ", A[i]); puts("");
  return 0;
}
