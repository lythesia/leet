#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

typedef unordered_map<string, bool> msb;
typedef pair<string, int> psi;
class Solution {
public:
  int ladderLength(string s, string e, unordered_set<string> &dict) {
    msb tab;
    for(auto &s : dict) tab.insert(make_pair(s, false));
    queue<psi> Q;
    Q.push(psi(s, 1));
    psi p;
    while(!Q.empty()) {
      p = Q.front();
      auto &t = p.first;
      if(t == e) return p.second;
      Q.pop();
      for(size_t i=0; i<t.length(); i++) {
        for(char c='a'; c<='z'; c++)
          if(t[i] != c) {
            char ch = t[i];
            t[i] = c;
            if(t == e) return p.second+1;
            if(tab.find(t) != tab.end() && !tab[t]) { // has but not used
              tab[t] = true;
              Q.push(psi(t, p.second+1));
            }
            t[i] = ch;
          }
      }
    }
    return 0;
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  // string s = "hit", e = "cog";
  // vs d = {"hot","dot","dog","lot","log"};
  string s = "a", e = "c";
  vs d = {"a", "b", "c"};
  unordered_set<string> dict(d.begin(), d.end());
  cout << so.ladderLength(s, e, dict) << endl;
  return 0;
}
