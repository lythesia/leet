#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Solution {
public:
  bool wordPattern(string pat, string str) {
    string h[26];
    unordered_map<string, char> rh;
    vs v;
    for(auto p=str.begin();;) {
      auto np = find(p, str.end(), ' ');
      v.emplace_back(string(p, np));
      if(np == str.end()) break;
      p = ++np;
    }
    if(pat.length() != v.size()) return false;
    for(size_t i=0; i<pat.length(); i++) {
      if(h[pat[i] - 'a'].empty()) {
        if(!rh.count(v[i])) h[pat[i] - 'a'] = v[i], rh[v[i]] = pat[i];
        else return false;
      }
      else if(h[pat[i] - 'a'] != v[i]) return false;
    }
    return true;
  }
};

int main(int argc, const char *argv[])
{
  // string pat = "abba", str = "dog cat cat dog";
  // string pat = "abba", str = "dog cat cat fish";
  // string pat = "aaaa", str = "dog cat cat dog";
  string pat = "abba", str = "dog dog dog dog";
  Solution so;
  cout << boolalpha << so.wordPattern(pat, str) << endl;
  return 0;
}
