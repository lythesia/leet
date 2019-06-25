#include <cstdio>
#include <cstring>
#include <cctype>
#include <climits>
#include <iostream>
#include <string>

using namespace std;

#define see(x) cout << #x << ": " << (x) << endl

class Solution {
public:
    int atoi(const char *str) {
      int res = 0;
      bool sign = false;
      bool neg = false;
      bool num = false;

      int len = strlen(str);
      for(int i=0; i<len; i++) {
        if(isblank(str[i])) {
          if(sign) return 0;
          else if(num) return res;
          else continue;
        }

        if(str[i] == '-') {
          if(sign) return 0;
          else {
            neg = true;
            sign = true;
          }
        }
        else if(str[i] == '+') {
          if(sign) return 0;
          else sign = true;
        }
        else if(isdigit(str[i])) {
          num = true;
          long long tmp = (long long)(res)*10 + (str[i] - '0');
          if((neg ? -tmp:tmp) < INT_MIN) { return INT_MIN;}
          else if((neg ? -tmp:tmp) > INT_MAX) { return INT_MAX;}
          else {
            res = static_cast<int>(tmp);
          }
        }
        else break;
      }
      return neg ? -res : res;
    }
};

int main(int argc, const char *argv[])
{
  Solution so;
  char buf[128] = {0};

  while(fgets(buf, sizeof(buf), stdin)) {
    //buf[strlen(buf) - 1] = 0;
    see(so.atoi(buf));
  }
  return 0;
}
