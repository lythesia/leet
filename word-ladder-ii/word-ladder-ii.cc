#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

typedef unordered_set<string> uss;
class Solution {
public:
  void dfs(vvs &ans, vs &seg, unordered_map<string, uss> &path, string s, string &e) {
    if(!path.count(s)) {
      if(s == e) ans.push_back(seg);
      return;
    }
    for(auto &ss : path[s]) {
      seg.push_back(ss);
      dfs(ans, seg, path, ss, e);
      seg.pop_back();
    }
  }

  vvs findLadders(string s, string e, unordered_set<string> &dict) {
    vvs ans;
    vs seg = {s};
    if(s == e) {
      seg.push_back(e);
      ans.push_back(seg);
      return ans;
    }
    uss level[3];
    uss *prev = &level[0], *curr = &level[1], *next = &level[2], *p = NULL;
    unordered_map<string, uss> path;
    curr->insert(s);
    bool fnd = false;
    int len = s.length();
    while(!fnd) {
      next->clear();
      for(auto &t : *curr) {
        for(int i=0; i<len; i++) {
          string tmp = t;
          for(char c='a'; c<='z'; c++) {
            tmp[i] = c;
            if(tmp == e) {
              fnd = true;
              path[t].insert(e);
              continue;
            }
            if(dict.count(tmp) && !curr->count(tmp) && !prev->count(tmp)) {
              next->insert(tmp);
              path[t].insert(tmp);
            }
          }
        }
      }
      if(next->empty()) break;
      p = prev, prev = curr, curr = next, next = p;
    }
    dfs(ans, seg, path, s, e);
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  // string s = "hit", e = "cog";
  // vs d = {"hot","dot","dog","lot","log"};
  string s = "red", e = "tax";
  vs d = {"ted","tex","red","tax","tad","den","rex","pee"};
  unordered_set<string> dict(d.begin(), d.end());

  Solution so;
  vvs ans = so.findLadders(s, e, dict);
  for(auto &v : ans) {
    cout << "[ ";
    copy(v.begin(), v.end(), ostream_iterator<string>(cout, " "));
    cout << "]" << endl;
  }
  return 0;
}
