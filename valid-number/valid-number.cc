#include <cstdio>
#include <cstring>
#include <cctype>
#include <iostream>

using namespace std;

#define see(x) cout << #x << ": " << (x) << endl

class Solution {
public:
  bool isNumber(const char *s) {
    bool sign = false;
    bool num = false;
    bool point = false;
    bool exp = false;
    bool exp_sgn = false;
    bool exp_num = false;
    bool rspace = false;
    int len = strlen(s);
    for(int i=0; i<len; i++) {
      if(isblank(s[i])) {
        if(!num && (sign || point)) return false;
        else if(num) rspace = true;
        else continue;
      }
      else if(s[i]=='-' || s[i]=='+') {
        if(!exp) {
          if(sign || num || point || rspace) return false;
          else sign = true;;
        }
        else {
          if(exp_sgn || exp_num || rspace) return false;
          else exp_sgn = true;
        }
      }
      else if(s[i]=='.') {
        if(point || exp || rspace) return false;
        else point = true;
      }
      else if(isdigit(s[i])) {
        if(rspace) return false;
        else if(exp) exp_num = true;
        else num = true;
      }
      else if(s[i] == 'e') {
        if(!num || exp) return false;
        else exp = true;
      }
      else return false;
    }
    return exp ? num&&exp_num : num;
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  char buf[128] = {0};

  while(fgets(buf, sizeof(buf), stdin)) {
    buf[strlen(buf)-1] = 0;
    bool b = so.isNumber(buf);
    cout << buf << ": " << b << endl;
  }
 
  return 0;
}
