#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;


class WordDictionary {
public:
  struct Node {
    Node(bool w = false) : isw(w) {}
    bool isw;
    Node *child[26] = {NULL};
  };
  typedef Node *node;

  void addWord(string w) {
    node p = root;
    for(char c : w) {
      if(!p->child[c-'a']) p->child[c-'a'] = new Node();
      p = p->child[c-'a'];
    }
    p->isw = true;
  }

  bool search(string w) {
    return dfs(root, w, 0);
  }

  bool dfs(node p, string &w, size_t s) {
    if(!p) return false;
    if(s == w.length()) return p->isw;

    char c = w[s];
    if(c != '.') return dfs(p->child[c-'a'], w, s+1);
    else for(int i=0; i<26; i++) if(dfs(p->child[i], w, s+1)) return true;
    return false;
  }

private:
  node root = new Node();
};

int main(int argc, const char *argv[])
{
  WordDictionary w;
  cout << boolalpha;
  w.addWord("bad");
  w.addWord("dad");
  w.addWord("mad");

  cout << w.search(".") << endl;   // -> false
  cout << w.search("..") << endl;  // -> false
  cout << w.search("...") << endl; // -> false
  cout << w.search("pad") << endl; // -> false
  cout << w.search("bad") << endl; // -> true
  cout << w.search(".ad") << endl; // -> true
  cout << w.search("b..") << endl; // -> true
  cout << w.search("m.k") << endl; // -> false

  // string s;
  // while(cin >> s) w.addWord(s);
  // cout << w.search("") << endl;
  return 0;
}
