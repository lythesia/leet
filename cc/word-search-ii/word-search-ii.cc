#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;
typedef vector<char> vc;
typedef vector<vc> vvc;

const int dir[][2] = {
  {0,-1},
  {-1,0},
  {0,1},
  {1,0},
};
class Solution {
public:
  struct Trie {
    Trie *chd[26] = {NULL};
    bool isw = false;
    int wi = -1;
  };
  typedef Trie *node;

public:
  void insert(node p, vs &ws, int i) {
    for(char c : ws[i]) p = p->chd[c-'a'] ? p->chd[c-'a'] : p->chd[c-'a'] = new Trie();
    p->isw = true, p->wi = i;
  }
  inline bool in(int i, int j, int m, int n) {
    return i>=0 && i<m && j>=0 && j<n;
  }
  void dfs(vs &ans, int x, int y, int m, int n, vvc &b, node p, vs &ws) {
    char ch = b[x][y];
    node pp = p->chd[ch-'a'];
    if(!pp) return; // not prefix
    else {
      if(pp->isw) {
        ans.push_back(ws[pp->wi]);
        pp->isw = false; // dummy remove word
      }
      b[x][y] = 0;
      for(int i=0; i<4; i++) {
        int nx = x + dir[i][0], ny = y + dir[i][1];
        if(in(nx, ny, m, n) && b[nx][ny]) dfs(ans, nx, ny, m, n, b, pp, ws);
      }
      b[x][y] = ch;
    }
  }
  vs findWords(vvc &b, vs &ws) {
    vs ans;
    if(!b.size() || !b[0].size() || !ws.size()) return ans;
    int m = b.size(), n = b[0].size();
    node t = new Trie();
    for(size_t i=0; i<ws.size(); i++) insert(t, ws, i); // build trie
    for(int i=0; i<m; i++) for(int j=0; j<n && ws.size() > ans.size(); j++) dfs(ans, i, j, m, n, b, t, ws);
    return ans;
  }
};

void pboard(vvc &board) {
  for(auto &v : board) {
    copy(v.begin(), v.end(), ostream_iterator<char>(cout, " ")); cout << endl;
  }
}

int main(int argc, const char *argv[])
{
  // vs rowb = {
    // "oaan",
    // "etae",
    // "ihkr",
    // "iflv",
    //
    // "ab",
    // "cd",
  // };
  vs rowb = {
    "baabab","abaaaa","abaaab","ababba","aabbab","aabbba","aabaab"
  };
  // vs words = {"oath","pea","eat","rain"};
  // vs words = {"acdb"};
  vs words = {"bbaabaabaaaaabaababaaaaababb","aabbaaabaaabaabaaaaaabbaaaba","babaababbbbbbbaabaababaabaaa","bbbaaabaabbaaababababbbbbaaa","babbabbbbaabbabaaaaaabbbaaab","bbbababbbbbbbababbabbbbbabaa","babababbababaabbbbabbbbabbba","abbbbbbaabaaabaaababaabbabba","aabaabababbbbbbababbbababbaa","aabbbbabbaababaaaabababbaaba","ababaababaaabbabbaabbaabbaba","abaabbbaaaaababbbaaaaabbbaab","aabbabaabaabbabababaaabbbaab","baaabaaaabbabaaabaabababaaaa","aaabbabaaaababbabbaabbaabbaa","aaabaaaaabaabbabaabbbbaabaaa","abbaabbaaaabbaababababbaabbb","baabaababbbbaaaabaaabbababbb","aabaababbaababbaaabaabababab","abbaaabbaabaabaabbbbaabbbbbb","aaababaabbaaabbbaaabbabbabab","bbababbbabbbbabbbbabbbbbabaa","abbbaabbbaaababbbababbababba","bbbbbbbabbbababbabaabababaab","aaaababaabbbbabaaaaabaaaaabb","bbaaabbbbabbaaabbaabbabbaaba","aabaabbbbaabaabbabaabababaaa","abbababbbaababaabbababababbb","aabbbabbaaaababbbbabbababbbb","babbbaabababbbbbbbbbaabbabaa"};

  // to board
  vvc b;
  for(auto &s : rowb) b.push_back(vc(s.begin(), s.end()));
  pboard(b);
  cout << "----" << endl;

  Solution so;
  vs ans = so.findWords(b, words);
  cout << "words:" << endl;
  copy(ans.begin(), ans.end(), ostream_iterator<string>(cout, "\n"));
  return 0;
}
