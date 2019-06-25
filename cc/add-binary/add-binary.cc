#include <cstdio>
#include <iostream>
#include <string>
#include <algorithm>

using namespace std;

#define see(x) cout << #x << ": " << (x) << endl
#define __n(c) ((c) - '0')
#define __c(n) ((n) + '0')

class Solution {
public:
  string addBinary(string a, string b) {
    string res;
    int c = 0;
    int i, j;
    if(a.size() < b.size()) swap(a, b);
    for(i=a.size()-1,j=b.size()-1; i>=0 && j>=0; i--,j--) {
      int s = __n(a[i]) + __n(b[j]) + c;
      c = s / 2;
      res.insert(res.begin(), __c(s%2));
    }
    while(i >= 0) {
      int s = __n(a[i--]) + c;
      c = s / 2;
      res.insert(res.begin(), __c(s%2));
    }
    if(c) res.insert(res.begin(), __c(c));
    return res;
    //return res.substr(res.find_first_not_of('0'));
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  argc >= 3 && printf("%s = %s + %s\n", so.addBinary(argv[1], argv[2]).c_str(), argv[1], argv[2]);
  return 0;
}
