#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
  int lengthOfLongestSubstring(string s) {
    int n = s.length();
    if(n < 2) return n;
    unordered_map<char, int> has;
    int l = 0, r = 0, mmax = 0;
    for(; r<n; r++) {
      char right = s[r];
      if(!has[right]) has[right]++;
      else {
        mmax = max(mmax, r-l);
        for(; l<r; l++) {
          char left = s[l];
          if(left != right) has[left] = 0;
          else break;
        }
        l++; // no need mmax, since always <= mmax
      }
    }
    mmax = max(mmax, r-l);
    return mmax;
  }
};

int main(int argc, const char *argv[])
{
  string s = "bdacabdefg";
  Solution so;
  cout << so.lengthOfLongestSubstring(s) << endl;
  return 0;
}
