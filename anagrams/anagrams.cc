#include <bits/stdc++.h>
using namespace std;

typedef vector<string> vs;
class Solution {
public:
  string sorts(string &s) {
    int n = s.length();
    int bucket[26] = {0};
    for(char c : s) bucket[(int)(c-'a')]++;
    int cnt = 0;
    string ans;
    for(int i=0; i<26 && cnt<n; i++) while(bucket[i]--) ans.push_back((cnt++, 'a'+i));
    return ans;
  }

  vs anagrams(vs &strs) {
    unordered_map<string, int> tab;
    vs ans;
    int len = strs.size();
    for(int i=0; i<len; i++) {
      string ss = sorts(strs[i]);
      auto it = tab.find(ss);
      if(it != tab.end()) {
        if(it->second >= 0) {
          ans.push_back(strs[it->second]);
          it->second = -1;
        }
        ans.push_back(strs[i]);
      }
      else tab[ss] = i;
    }
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  // string s = "uuyadba";
  // cout << so.sorts(s) << endl;
  // vs ss = {"cat","rye","aye","dog", "god","cud","cat","old","fop","bra"};
  vs ss = {"fuck"};
  vs ans = so.anagrams(ss);
  copy(ans.begin(), ans.end(), ostream_iterator<string>(cout, " ")); cout << endl;
  return 0;
}
