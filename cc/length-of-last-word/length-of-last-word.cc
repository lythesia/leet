#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
  int lengthOfLastWord(const char *s) {
    char *buf = const_cast<char*>(s);
    char *p;
    int len = 0;
    p = strtok(buf, " ");
    while(p) {
      len = strlen(p);
      p = strtok(NULL, " ");
    }
    return len;
  }
};

int main(int argc, const char *argv[])
{
  vector<string> ss = {
    "helo world",
    "helo world ",
    "",
    " ",
    "hahah ",
    "   a   b   "
  };
  Solution so;
  for(auto s : ss) printf("%d\n", so.lengthOfLastWord(s.c_str()));
  return 0;
}
