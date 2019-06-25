#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
  bool isPalindrome(string s) {
    int len = s.length(), h = 0, t = len - 1;
    if(!len || len == 1) return true;
    while(h < t) {
      bool ah = isalnum(s[h]), at = isalnum(s[t]);
      if(ah && at) {
        if(tolower(s[h]) == tolower(s[t])) {
          h++, t--;
          continue;
        }
        else return false;
      }
      else if(ah) t--;
      else if(at) h++;
      else t--, h++;
    }
    return true;
  }
};

int main(int argc, const char *argv[])
{
  // string s = "A man, a plan, a canal: Panama";
  string s = "race a car";
  Solution so;
  cout << so.isPalindrome(s) << endl;
  return 0;
}
