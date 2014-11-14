#include <cstdio>
#include <cstring>
#include <iostream>
#include <string>

using namespace std;

#define see(x) cout << #x << ": " << (x) << endl

class Solution {
public:
  int romanToInt(string s) {
    int res = 0;
    char prev = 0;
    for(char c : s) {
      if(c == 'I') {
        ++res;
      }
      else if(c == 'V') {
        if(prev == 'I') res += 3;
        else res += 5;
      }
      else if(c == 'X') {
        if(prev == 'I') res += 8;
        else res += 10;
      }
      else if(c == 'L') {
        if(prev == 'X') res += 30;
        else res += 50;
      }
      else if(c == 'C') {
        if(prev == 'X') res += 80;
        else res += 100;
      }
      else if(c == 'D') {
        if(prev == 'C') res += 300;
        else res += 500;
      }
      else if(c == 'M') {
        if(prev == 'C') res += 800;
        else res += 1000;
      }
      prev = c;
    }
    return res;
  }
};

int main(int argc, const char *argv[])
{
  char buf[128] = {0};
  Solution so;
  while(fgets(buf, sizeof(buf), stdin)) {
    buf[strlen(buf)-1] = 0;
    see(so.romanToInt(buf));
  }
  return 0;
}
