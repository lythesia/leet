#include <cstdio>
#include <iostream>
#include <string>

using namespace std;

class Solution {
public:
  string getPermutation(int n, int k) {
    static int facs[] = {
      1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880
    };

    string res;
    for(int i=1; i<=n; i++) res += char(i+'0');
    k--; // make index from 0
    for(int i=0; i<n; i++) { //
      int m = k % facs[n-i]; // next k-th of sub tree
      int s = k / facs[n-i]; // s-th sub tree
      if(!m && !n) return res;
      else {
        if(s > 0) {
          for(int j=i-1+s; j>i-1; j--) { // fix i-1 with (i-1)+s
            char tmp = res[j];
            res[j] = res[j-1];
            res[j-1] = tmp;
          } 
          if(!m) return res; // the 0-th of sub tree is self
        }
        k = m;
      }
    }
    return res;
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  cout << so.getPermutation(3, 6) << endl;
  return 0;
}
