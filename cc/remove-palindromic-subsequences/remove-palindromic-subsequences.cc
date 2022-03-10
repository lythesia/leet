#include <bits/stdc++.h>
#include <ranges>
using namespace std;

class Solution {
public:
    int removePalindromeSub(string s) {
      return 2 - ranges::all_of(ranges::iota_view{0, s.size()/2}, [](int i) s[i] == s[s.size()-1-i]);
    }
};

int main(int argc, char *argv[])
{
  cout << Solution::removePalindromeSub("bab");
  cout << Solution::removePalindromeSub("babbb");
  return 0;
}
