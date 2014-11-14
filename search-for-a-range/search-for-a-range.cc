#include <cstdio>
#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

typedef vector<int> vi;
class Solution {
public:
  vi searchRange(int A[], int n, int target) {
    if(!n) return vi{-1, -1};
    auto b = equal_range(A, A+n, target);
    int s = (int)(b.first-A), e = (int)(b.second-A);
    if(s == n || A[s]!=target) return vi{-1, -1};
    return vi{s, e-1};
  }
};

int main(int argc, const char *argv[])
{
  int n[] = {5, 7, 7, 8, 8, 10};
  Solution so;
  auto res = so.searchRange(n, sizeof(n)/sizeof(int), 4);
  cout << res[0] << ':' << res[1] << endl;
  return 0;
}
