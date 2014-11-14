#include <cstdio>
#include <iostream>
#include <algorithm>

using namespace std;

class Solution {
public:
  int firstMissingPositive(int A[], int n) {
    for(int i=0; i<n; i++) {
      int v = A[i];
      while(v<=n && v>0 && A[v-1]!=v) {
        swap(A[i], A[v-1]);
        v = A[i];
      }
    }
    for(int i=0; i<n; i++) if(A[i] != i+1) return i+1;
    return n+1;
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  int a[] = {1,2,2,1,3,1,0,4,0};
  int res = so.firstMissingPositive(a, sizeof(a)/sizeof(int));
  cout << "a: [ ";
  for(auto i : a) cout << i << ' '; cout << ']' << endl;
  cout << res << endl;
  return 0;
}
