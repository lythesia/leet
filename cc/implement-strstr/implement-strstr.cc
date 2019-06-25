#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
  int strStr(char *haystack, char *needle) {
    if(!haystack || !needle) return -1;
    int len = strlen(needle);
    auto c = haystack;
    do {
      if(!strncmp(c++, needle, len)) return c - haystack - 1;
    } while (*c);
    return -1;
  }
};

int main(int argc, const char *argv[])
{
  char str[] = "This is simple string";
  // char str[] = "";
  // char n[] = "simple";
  char n[] = "simple string string";
  // char n[] = "";
  Solution so;
  printf("%d\n", so.strStr(str, n));
  return 0;
}
