#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
  inline int __n(char c) {
    return c - 'A' + 1;
  }

  int titleToNumber(string s) {
    int ans = 0;
    for(int i = 0; s[i]; i++) {
      ans = ans*26 + __n(s[i]);
    }
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  printf("%d\n", so.titleToNumber("Z"));
  printf("%d\n", so.titleToNumber("AB"));
  printf("%d\n", so.titleToNumber("BA"));
  return 0;
}
