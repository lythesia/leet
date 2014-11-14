#include <cstdio>
#include <cstring>
#include <iostream>
#include <string>

using namespace std;

#define see(x) cout << #x << ": " << (x) << endl

class Solution {
public:
  string intToRoman(int num) {
    static int dig[] = {1000, 100, 10, 1};
    static char ch[] = {'M', 'C', 'X', 'I'};
    static char fiv[] = {    'D', 'L', 'V'};
    string res;
    for(int i=0; i<4; i++) {
      int nd = num / dig[i];
      if(nd) {
        if(nd < 4) {
          for(int j=0; j<nd; j++) res += ch[i];
        }
        else if(nd == 4) {
          res += ch[i], res += fiv[i-1];
        }
        else if(nd == 5) {
          res += fiv[i-1];
        }
        else if(nd < 9) {
          res += fiv[i-1];
          for(int j=0; j<nd-5; j++) res += ch[i];
        }
        else if(nd == 9) {
          res += ch[i], res += ch[i-1];
        }
        num -= nd*dig[i];
      }
    }
    return res;
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  int num;
  while(scanf("%d", &num) != EOF) {
    see(so.intToRoman(num));
  }
  return 0;
}
