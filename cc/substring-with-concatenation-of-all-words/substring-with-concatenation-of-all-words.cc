#include <cstdio>
#include <iostream>
#include <string>
#include <vector>
#include <map>
#include <algorithm>
using namespace std;

typedef vector<int> vi;
typedef vector<string> vs;
typedef map<string, int> msi;
class Solution {
public:
  vi findSubstring(string S, vs &L) {
    vi res;
    if(!S.size() || !L.size()) return res;

    int n = S.size(), w = L[0].size();
    msi dic, used;
    for(auto s : L) ++dic[s];
    for(int wi = 0; wi < w; wi++) {
      used.clear();
      int start = wi, cnt = 0; // valid string from `start` !
      for(int i=wi; i<n; i+=w) {
        auto sp = S.substr(i, w);
        if(!dic.count(sp)) {    /* not match any */
          used.clear();
          cnt = 0, start = i+w; // advanced to next seg
        }
        else if(used[sp] < dic[sp]) ++used[sp], ++cnt; /* match one and available */
        else { /* match but not available: cut 1st occurance of current word, from there on */
          string cur;
          while((cur = S.substr(start, w)) != sp) {
            --used[cur], --cnt;
            start += w;
          }
          start += w; // found, cut it
        }

        if(cnt == L.size()) { // valid string 
          res.push_back(start);
          // then cut 1st word, try to search next word append to last to make it valid again
          string sub = S.substr(start, w);
          --used[sub], --cnt;
          start += w;
        }
      }
    }
    sort(res.begin(), res.end());
    return res;
  }
};

int main(int argc, const char *argv[])
{
  vs L = {"aa", "ab"}; // note: allow duplicate!
  Solution so;
  vi res = so.findSubstring("aababaacab", L);
  for(int i : res) cout << i << " "; cout << endl;
  return 0;
}
