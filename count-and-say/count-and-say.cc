#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
  string say(string n) {
    string ans;
    int len = n.length();
    if(len) {
      char last = n[0];
      int cnt = 1;
      for(int i=1; i<len; i++) {
        if(n[i] == last) cnt++;
        else ans+=to_string(cnt)+last, last = n[i], cnt = 1;
      }
      ans+=to_string(cnt)+last;
    }
    return ans;
  }

  string countAndSay(int n) {
    string a("1");
    for(int i=1; i<n; i++) a = say(a);
    return a;
  }
};

int main(int argc, const char *argv[])
{
  int n = 10;
  Solution so;
  cout << so.countAndSay(n) << endl;
  return 0;
}
