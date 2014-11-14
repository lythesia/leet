#include <cstdio>
#include <iostream>
#include <algorithm>
using namespace std;

class Solution {
public:
  int removeDuplicates(int A[], int n) {
    int res = unique(A, A+n) - A;
    return res;
  }
};

int main(int argc, const char *argv[])
{
  int A[] = {1,1,2};
  Solution so;
  cout << so.removeDuplicates(A, sizeof(A)/sizeof(int)) << endl;
  return 0;
}
