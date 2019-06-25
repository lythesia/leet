#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
  int singleNumber(int A[], int n) {
    int ret = 0;
    for_each(A, A+n, [&ret](int i) { return ret^=i; });
    return ret;
  }
};

int main(int argc, const char *argv[])
{
  int A[] = { 1, 2, 1 };
  Solution so;
  printf("%d\n", so.singleNumber(A, sizeof(A) / sizeof(int)));
  return 0;
}
