#include <bits/stdc++.h>
using namespace std;

void parray(int *a, int n) {
  for(int i=0; i<n; i++) printf("%d ", a[i]); puts("");
}

class Solution {
public:
#if 0
  void rotate(int nums[], int n, int k) {
    if(n < 2) return;
    int *buf = new int[n<<1];
    memcpy(buf, nums, n * sizeof(int));
    memcpy(buf+n, nums, n * sizeof(int));
    k %= n;
    for(int i=0; i<n; i++) nums[i] = buf[i+n-k];
    delete[] buf;
  }
#endif
  void rotate(int nums[], int n, int k) {
    reverse(nums, nums + n);
    reverse(nums, nums + k%n);
    reverse(nums + k%n, nums + n);
  }
};

int main(int argc, const char *argv[])
{
  int nums[] = {1, 2, 3, 4, 5, 6};
  int n = 6, k = 2;
  Solution so;
  so.rotate(nums, n, k);
  parray(nums, n);
  return 0;
}
