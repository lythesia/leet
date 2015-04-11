#include <bits/stdc++.h>
// #include <iostream>
// #include <string>
using namespace std;

class Solution {
public:
  int comp(string &s1, int p1, int e1, string &s2, int p2, int e2) {
    while(s1[p1] == '0') p1++;
    while(s2[p2] == '0') p2++;
    int diff = (e1 - p1) - (e2 - p2);
    if(diff) return diff > 0 ? 1 : -1;
    for(; p1 < e1 && p2 < e2; p1++, p2++) {
      if(s1[p1] > s2[p2]) return 1;
      else if(s1[p1] < s2[p2]) return -1;
    }
    return 0;
  }

  int compareVersion(string v1, string v2) {
    size_t p1 = 0, p2 = 0, e1 = v1.find('.', p1), e2 = v2.find('.', p2);
    while(e1 != string::npos && e2 != string::npos) {
      int ans = comp(v1, p1, e1, v2, p2, e2);
      if(ans) return ans;
      p1 = e1 + 1, p2 = e2 + 1;
      e1 = v1.find('.', p1), e2 = v2.find('.', p2);
    }
    if(e1 == string::npos && e2 == string::npos)  return comp(v1, p1, v1.length(), v2, p2, v2.length());
    else if(e1 == string::npos) {
      int ans = comp(v1, p1, v1.length(), v2, p2, e2);
      if(ans) return ans;
      return v2.find_first_not_of("0.", e2+1) != string::npos ? -1 : 0;
    }
    else {
      int ans = comp(v1, p1, e1, v2, p2, v2.length());
      if(ans) return ans;
      return v1.find_first_not_of("0.", e1+1) != string::npos ? 1 : 0;
    }
  }
};

int main(int argc, const char *argv[])
{
  // string s1 = "1.1",  // 1.(1)
  //        s2 = "1.10"; // 1,(10)
  // string s1 = "0.1", s2 = "0.0.1";
  // string s1 = "01", s2 = "1";
  string s1 = "1.0", s2 = "1";
  Solution so;
  cout << so.compareVersion(s1, s2) << endl;
  return 0;
}
